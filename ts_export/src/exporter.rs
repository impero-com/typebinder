use serde_derive_internals::{
    ast::{Container, Data, Field, Style, Variant},
    attr::TagType,
};
use syn::{GenericParam, Generics, ItemType};
use ts_json_subset::{
    declarations::{interface::InterfaceDeclaration, type_alias::TypeAliasDeclaration},
    export::ExportStatement,
    types::{
        LiteralType, ObjectType, PrimaryType, PropertyName, PropertySignature, TsType, TupleType,
        TypeBody, TypeMember, TypeParameters, UnionType,
    },
};

use crate::type_solver::{MemberInfo, TypeInfo, TypeSolvingContext};

pub struct Exporter {
    pub solving_context: TypeSolvingContext,
}

fn extract_type_parameters(generics: &Generics) -> Option<TypeParameters> {
    let identifiers: Vec<String> = generics
        .params
        .iter()
        .filter_map(|param| match param {
            GenericParam::Type(ty) => Some(ty.ident.to_string()),
            _ => None,
        })
        .collect();

    if identifiers.is_empty() {
        None
    } else {
        Some(TypeParameters { identifiers })
    }
}

impl Exporter {
    pub fn export_statements_from_container(&self, container: Container) -> Vec<ExportStatement> {
        let name = container.attrs.name().serialize_name();
        match container.data {
            Data::Enum(variants) => match container.attrs.tag() {
                TagType::External => self.export_enum_external(name, container.generics, variants),
                TagType::Internal { tag } => {
                    self.export_enum_internal(name, container.generics, variants, tag)
                }
                TagType::Adjacent { tag, content } => {
                    self.export_enum_adjacent(name, container.generics, variants, tag, content)
                }
                TagType::None => self.export_enum_untagged(name, container.generics, variants),
            },
            Data::Struct(style, fields) => match style {
                Style::Unit => vec![], // Unit structs are a no-op because they dont have a TS representation
                Style::Newtype => self.export_struct_newtype(name, container.generics, fields),
                Style::Tuple => self.export_struct_tuple(name, container.generics, fields),
                Style::Struct => self.export_struct_struct(name, container.generics, fields),
            },
        }
    }

    pub fn export_statements_from_type_alias(&self, type_alias: ItemType) -> Vec<ExportStatement> {
        let ident = type_alias.ident.to_string();
        let params = extract_type_parameters(&type_alias.generics);
        let solver_info = TypeInfo {
            generics: &type_alias.generics,
            ty: type_alias.ty.as_ref(),
        };
        self.solving_context
            .solve_type(&solver_info)
            .map(|inner_type| {
                ExportStatement::TypeAliasDeclaration(TypeAliasDeclaration {
                    ident,
                    inner_type,
                    params,
                })
            })
            .into_iter()
            .collect()
    }

    fn export_struct_struct(
        &self,
        ident: String,
        generics: &Generics,
        fields: Vec<Field>,
    ) -> Vec<ExportStatement> {
        let members: Vec<TypeMember> = fields
            .into_iter()
            .filter_map(|field| {
                // TODO: Handle skip_serializing_if.
                // How ? : mark TypeMember as optional if skip_seriazing_if is `Some`
                if field.attrs.skip_serializing() {
                    return None;
                }
                let solver_info = MemberInfo { generics, field };
                self.solving_context.solve_member(&solver_info)
            })
            .collect();
        let type_params = extract_type_parameters(generics);
        vec![ExportStatement::InterfaceDeclaration(
            InterfaceDeclaration {
                ident,
                extends_clause: None,
                type_params,
                obj_type: ObjectType {
                    body: Some(TypeBody { members }),
                },
            },
        )]
    }

    fn export_struct_newtype(
        &self,
        ident: String,
        generics: &Generics,
        fields: Vec<Field>,
    ) -> Vec<ExportStatement> {
        let field = &fields[0];
        let solver_info = TypeInfo {
            generics,
            ty: field.ty,
        };
        let params = extract_type_parameters(generics);
        self.solving_context
            .solve_type(&solver_info)
            .map(|inner_type| {
                TypeAliasDeclaration {
                    ident,
                    inner_type,
                    params,
                }
                .into()
            })
            .into_iter()
            .collect()
    }

    fn export_struct_tuple(
        &self,
        ident: String,
        generics: &Generics,
        fields: Vec<Field>,
    ) -> Vec<ExportStatement> {
        let inner_types: Vec<TsType> = fields
            .into_iter()
            .filter_map(|field| {
                let solver_info = TypeInfo {
                    generics,
                    ty: field.ty,
                };
                self.solving_context.solve_type(&solver_info)
            })
            .collect();
        let inner_type = TsType::PrimaryType(PrimaryType::TupleType(TupleType { inner_types }));
        let params = extract_type_parameters(generics);

        vec![TypeAliasDeclaration {
            ident,
            inner_type,
            params,
        }
        .into()]
    }

    fn export_enum_internal(
        &self,
        ident: String,
        generics: &Generics,
        variants: Vec<Variant>,
        tag: &String,
    ) -> Vec<ExportStatement> {
        let types: Vec<TsType> = variants
            .into_iter()
            .map(|variant| {
                let mut members: Vec<TypeMember> = variant
                    .fields
                    .into_iter()
                    .filter_map(|field| {
                        // TODO: Filter Variants which have unnamed members since those will fail to serialize
                        let solver_info = MemberInfo { generics, field };
                        self.solving_context.solve_member(&solver_info)
                    })
                    .collect();
                members.push(TypeMember::PropertySignature(PropertySignature {
                    name: PropertyName::Identifier(tag.clone()),
                    inner_type: TsType::PrimaryType(PrimaryType::LiteralType(
                        LiteralType::StringLiteral(variant.attrs.name().serialize_name().into()),
                    )),
                    optional: false,
                }));
                TsType::PrimaryType(PrimaryType::ObjectType(ObjectType {
                    body: Some(TypeBody { members }),
                }))
            })
            .collect();
        let params = extract_type_parameters(generics);

        vec![ExportStatement::TypeAliasDeclaration(
            TypeAliasDeclaration {
                ident,
                inner_type: TsType::UnionType(UnionType { types }),
                params,
            },
        )]
    }

    fn export_enum_untagged(
        &self,
        ident: String,
        generics: &Generics,
        variants: Vec<Variant>,
    ) -> Vec<ExportStatement> {
        let types: Vec<TsType> = variants
            .into_iter()
            .filter_map(|variant| match variant.style {
                Style::Unit => Some(TsType::PrimaryType(PrimaryType::Predefined(
                    ts_json_subset::types::PredefinedType::Null,
                ))),
                Style::Newtype => {
                    let field = &variant.fields[0];
                    self.solving_context.solve_type(&TypeInfo {
                        generics,
                        ty: field.ty,
                    })
                }
                Style::Tuple => {
                    let inner_types = variant
                        .fields
                        .into_iter()
                        .filter_map(|field| {
                            self.solving_context.solve_type(&TypeInfo {
                                generics,
                                ty: field.ty,
                            })
                        })
                        .collect();
                    Some(TsType::PrimaryType(PrimaryType::TupleType(TupleType {
                        inner_types,
                    })))
                }
                Style::Struct => {
                    let members: Vec<TypeMember> = variant
                        .fields
                        .into_iter()
                        .filter_map(|field| {
                            self.solving_context
                                .solve_member(&MemberInfo { generics, field })
                        })
                        .collect();
                    Some(TsType::PrimaryType(PrimaryType::ObjectType(ObjectType {
                        body: Some(TypeBody { members }),
                    })))
                }
            })
            .collect();
        let inner_type = TsType::UnionType(UnionType { types });
        let params = extract_type_parameters(generics);
        vec![TypeAliasDeclaration {
            ident,
            inner_type,
            params,
        }
        .into()]
    }

    fn export_enum_adjacent(
        &self,
        ident: String,
        generics: &Generics,
        variants: Vec<Variant>,
        tag: &String,
        content: &String,
    ) -> Vec<ExportStatement> {
        let types: Vec<TsType> = variants
            .into_iter()
            .map(|variant| {
                let members: Vec<TypeMember> = variant
                    .fields
                    .into_iter()
                    .filter_map(|field| {
                        // TODO: Filter Variants which have unnamed members since those will fail to serialize
                        let solver_info = MemberInfo { generics, field };
                        self.solving_context.solve_member(&solver_info)
                    })
                    .collect();
                let content_member = TypeMember::PropertySignature(PropertySignature {
                    name: PropertyName::Identifier(content.clone()),
                    inner_type: TsType::PrimaryType(PrimaryType::ObjectType(ObjectType {
                        body: Some(TypeBody { members }),
                    })),
                    optional: false,
                });
                let tag_member = TypeMember::PropertySignature(PropertySignature {
                    name: PropertyName::Identifier(tag.clone()),
                    inner_type: TsType::PrimaryType(PrimaryType::LiteralType(
                        LiteralType::StringLiteral(variant.attrs.name().serialize_name().into()),
                    )),
                    optional: false,
                });
                TsType::PrimaryType(PrimaryType::ObjectType(ObjectType {
                    body: Some(TypeBody {
                        members: vec![tag_member, content_member],
                    }),
                }))
            })
            .collect();
        let inner_type = TsType::UnionType(UnionType { types });
        let params = extract_type_parameters(generics);
        vec![TypeAliasDeclaration {
            ident,
            inner_type,
            params,
        }
        .into()]
    }

    fn export_enum_external(
        &self,
        ident: String,
        generics: &Generics,
        variants: Vec<Variant>,
    ) -> Vec<ExportStatement> {
        let types: Vec<TsType> = variants
            .into_iter()
            .filter_map(|variant| {
                let variant_name = variant.attrs.name().serialize_name();
                let members = variant
                    .fields
                    .into_iter()
                    .filter_map(|field| {
                        self.solving_context
                            .solve_member(&MemberInfo { generics, field })
                    })
                    .collect();
                let inner_type = TsType::PrimaryType(PrimaryType::ObjectType(ObjectType {
                    body: Some(TypeBody { members }),
                }));
                let container = TsType::PrimaryType(PrimaryType::ObjectType(ObjectType {
                    body: Some(TypeBody {
                        members: vec![TypeMember::PropertySignature(PropertySignature {
                            inner_type,
                            optional: false,
                            name: PropertyName::StringLiteral(variant_name.into()),
                        })],
                    }),
                }));
                Some(container)
            })
            .collect();
        let inner_type = TsType::UnionType(UnionType { types });
        let params = extract_type_parameters(generics);
        vec![TypeAliasDeclaration {
            ident,
            inner_type,
            params,
        }
        .into()]
    }
}
