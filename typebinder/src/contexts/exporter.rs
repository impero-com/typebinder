use std::str::FromStr;

use super::{import::ImportContext, type_solving::TypeSolvingContext};
use crate::{
    error::TsExportError,
    macros::{context::MacroSolvingContext, MacroInfo},
    type_solving::ImportEntry,
    type_solving::{member_info::MemberInfo, result::SolverResult, type_info::TypeInfo},
};
use serde_derive_internals::{
    ast::{Container, Data, Field, Style, Variant},
    attr::TagType,
};
use syn::{GenericParam, Generics, ItemType};
use ts_json_subset::{
    declarations::{interface::InterfaceDeclaration, type_alias::TypeAliasDeclaration},
    export::ExportStatement,
    ident::{IdentError, TSIdent},
    types::{
        IntersectionType, LiteralType, ObjectType, ParenthesizedType, PrimaryType, PropertyName,
        PropertySignature, TsType, TupleType, TypeBody, TypeMember, TypeParameters, UnionType,
    },
};

/// The global exporting context. Wraps the other contexts.
pub struct ExporterContext<'a> {
    /// A context to solve a Rust type to a TS type
    pub type_solving_context: &'a TypeSolvingContext,
    /// A context to solve a Rust macro invocations
    pub macro_context: &'a MacroSolvingContext,
    /// A context that contains all the imports
    pub import_context: ImportContext,
}

fn extract_type_parameters(generics: &Generics) -> Result<Option<TypeParameters>, IdentError> {
    let identifiers: Vec<TSIdent> = generics
        .params
        .iter()
        .filter_map(|param| match param {
            GenericParam::Type(ty) => Some(TSIdent::from_str(&ty.ident.to_string())),
            _ => None,
        })
        .collect::<Result<_, _>>()?;

    if identifiers.is_empty() {
        Ok(None)
    } else {
        Ok(Some(TypeParameters { identifiers }))
    }
}

impl ExporterContext<'_> {
    pub fn solve_type(
        &self,
        solver_info: &TypeInfo,
    ) -> Result<(TsType, Vec<ImportEntry>), TsExportError> {
        for solver in self.type_solving_context.solvers() {
            match solver.as_ref().solve_as_type(&self, solver_info) {
                SolverResult::Continue => (),
                SolverResult::Solved(inner, imports) => return Ok((inner, imports)),
                SolverResult::Error(inner) => return Err(inner),
            }
        }
        Err(TsExportError::UnsolvedType(solver_info.ty.clone()))
    }

    pub fn solve_member(
        &self,
        solver_info: &MemberInfo,
    ) -> Result<(TypeMember, Vec<ImportEntry>), TsExportError> {
        for solver in self.type_solving_context.solvers() {
            match solver.as_ref().solve_as_member(&self, solver_info) {
                SolverResult::Continue => (),
                SolverResult::Solved(inner, imports) => return Ok((inner, imports)),
                SolverResult::Error(inner) => return Err(inner),
            }
        }
        Err(TsExportError::UnsolvedField(solver_info.field.clone()))
    }

    pub fn export_statements_from_macro(
        &self,
        macro_info: &MacroInfo,
    ) -> Result<(Vec<ExportStatement>, Vec<ImportEntry>), TsExportError> {
        for solver in self.macro_context.solvers() {
            match solver.as_ref().solve_macro(macro_info) {
                SolverResult::Continue => (),
                SolverResult::Solved(inner, imports) => return Ok((vec![inner], imports)),
                SolverResult::Error(inner) => return Err(inner),
            }
        }
        // TODO: Maybe have an error variant ?
        Ok((Vec::new(), Vec::new()))
    }

    pub fn export_statements_from_container(
        &self,
        container: Container,
    ) -> Result<(Vec<ExportStatement>, Vec<ImportEntry>), TsExportError> {
        let name = container.ident.to_string();
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
                Style::Unit => Ok((vec![], vec![])), // Unit structs are a no-op because they dont have a TS representation
                Style::Newtype => self.export_struct_newtype(name, container.generics, fields),
                Style::Tuple => self.export_struct_tuple(name, container.generics, fields),
                Style::Struct => self.export_struct_struct(name, container.generics, fields),
            },
        }
    }

    pub fn export_statements_from_type_alias(
        &self,
        type_alias: ItemType,
    ) -> Result<(Vec<ExportStatement>, Vec<ImportEntry>), TsExportError> {
        let ident = TSIdent::from_str(&type_alias.ident.to_string())?;
        let type_params = extract_type_parameters(&type_alias.generics)?;
        let solver_info = TypeInfo {
            generics: &type_alias.generics,
            ty: type_alias.ty.as_ref(),
        };
        self.solve_type(&solver_info).map(|(inner_type, imports)| {
            (
                vec![ExportStatement::TypeAliasDeclaration(
                    TypeAliasDeclaration {
                        ident,
                        inner_type,
                        type_params,
                    },
                )],
                imports,
            )
        })
    }

    fn export_struct_struct(
        &self,
        ident: String,
        generics: &Generics,
        fields: Vec<Field>,
    ) -> Result<(Vec<ExportStatement>, Vec<ImportEntry>), TsExportError> {
        let mut imports = Vec::new();
        let members: Vec<TypeMember> = fields
            .into_iter()
            .filter_map(|field| {
                if field.attrs.skip_serializing() {
                    return None;
                }
                let solver_info = MemberInfo::from_generics_and_field(generics, &field);
                Some(self.solve_member(&solver_info))
            })
            .collect::<Result<Vec<(TypeMember, Vec<ImportEntry>)>, TsExportError>>()?
            .into_iter()
            .map(|(member, mut entries)| {
                imports.append(&mut entries);
                member
            })
            .collect();
        let type_params = extract_type_parameters(generics)?;
        let ident = TSIdent::from_str(&ident)?;
        Ok((
            vec![ExportStatement::InterfaceDeclaration(
                InterfaceDeclaration {
                    ident,
                    extends_clause: None,
                    type_params,
                    obj_type: ObjectType {
                        body: TypeBody { members },
                    },
                },
            )],
            imports,
        ))
    }

    fn export_struct_newtype(
        &self,
        ident: String,
        generics: &Generics,
        fields: Vec<Field>,
    ) -> Result<(Vec<ExportStatement>, Vec<ImportEntry>), TsExportError> {
        let field = &fields[0];
        let solver_info = TypeInfo {
            generics,
            ty: field.ty,
        };
        let type_params = extract_type_parameters(generics)?;
        let ident = TSIdent::from_str(&ident)?;
        self.solve_type(&solver_info).map(|(inner_type, imports)| {
            (
                vec![TypeAliasDeclaration {
                    ident,
                    inner_type,
                    type_params,
                }
                .into()],
                imports,
            )
        })
    }

    fn export_struct_tuple(
        &self,
        ident: String,
        generics: &Generics,
        fields: Vec<Field>,
    ) -> Result<(Vec<ExportStatement>, Vec<ImportEntry>), TsExportError> {
        let mut imports: Vec<ImportEntry> = Vec::new();
        let inner_types: Vec<TsType> = fields
            .into_iter()
            .map(|field| {
                let solver_info = TypeInfo {
                    generics,
                    ty: field.ty,
                };
                self.solve_type(&solver_info)
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|(member, mut entries)| {
                imports.append(&mut entries);
                member
            })
            .collect();
        let inner_type = TsType::PrimaryType(PrimaryType::TupleType(TupleType { inner_types }));
        let type_params = extract_type_parameters(generics)?;
        let ident = TSIdent::from_str(&ident)?;
        Ok((
            vec![TypeAliasDeclaration {
                ident,
                inner_type,
                type_params,
            }
            .into()],
            imports,
        ))
    }

    fn export_enum_internal(
        &self,
        ident: String,
        generics: &Generics,
        variants: Vec<Variant>,
        tag: &str,
    ) -> Result<(Vec<ExportStatement>, Vec<ImportEntry>), TsExportError> {
        let mut imports: Vec<ImportEntry> = Vec::new();
        let types: Vec<TsType> = variants
            .into_iter()
            .map(|variant| {
                let variant_type = match (variant.style, variant.fields.as_slice()) {
                    (Style::Unit, []) | (Style::Tuple, _) => None,
                    (Style::Newtype, [field]) => {
                        let (ty, mut entries) = self.solve_type(&TypeInfo {
                            generics,
                            ty: &field.ty,
                        })?;
                        imports.append(&mut entries);
                        Some(ty)
                    }
                    (Style::Struct, fields) => {
                        let members: Vec<TypeMember> = fields
                            .iter()
                            .map(|field| {
                                let solver_info =
                                    MemberInfo::from_generics_and_field(generics, field);
                                self.solve_member(&solver_info)
                            })
                            .collect::<Result<Vec<_>, _>>()?
                            .into_iter()
                            .map(|(member, mut entries)| {
                                imports.append(&mut entries);
                                member
                            })
                            .collect();
                        Some(TsType::PrimaryType(PrimaryType::ObjectType(ObjectType {
                            body: TypeBody { members },
                        })))
                    }
                    _ => return Err(TsExportError::MalformedInput),
                };

                let tag_type = TsType::PrimaryType(PrimaryType::ObjectType(ObjectType {
                    body: TypeBody {
                        members: vec![TypeMember::PropertySignature(PropertySignature {
                            name: PropertyName::from(tag.to_string()),
                            inner_type: TsType::PrimaryType(PrimaryType::LiteralType(
                                LiteralType::StringLiteral(
                                    variant.attrs.name().serialize_name().into(),
                                ),
                            )),
                            optional: false,
                        })],
                    },
                }));
                let inter = TsType::IntersectionType(IntersectionType {
                    types: Some(tag_type).into_iter().chain(variant_type).collect(),
                });
                Ok(TsType::ParenthesizedType(ParenthesizedType {
                    inner: Box::new(inter),
                }))
            })
            .collect::<Result<_, TsExportError>>()?;
        let type_params = extract_type_parameters(generics)?;

        let ident = TSIdent::from_str(&ident)?;
        Ok((
            vec![ExportStatement::TypeAliasDeclaration(
                TypeAliasDeclaration {
                    ident,
                    inner_type: TsType::UnionType(UnionType { types }),
                    type_params,
                },
            )],
            imports,
        ))
    }

    fn export_enum_untagged(
        &self,
        ident: String,
        generics: &Generics,
        variants: Vec<Variant>,
    ) -> Result<(Vec<ExportStatement>, Vec<ImportEntry>), TsExportError> {
        let mut imports: Vec<ImportEntry> = Vec::new();
        let types: Vec<TsType> = variants
            .into_iter()
            .map(|variant| match variant.style {
                Style::Unit => Ok((
                    TsType::PrimaryType(PrimaryType::Predefined(
                        ts_json_subset::types::PredefinedType::Null,
                    )),
                    Vec::new(),
                )),
                Style::Newtype => {
                    let field = &variant.fields[0];
                    self.solve_type(&TypeInfo {
                        generics,
                        ty: field.ty,
                    })
                }
                Style::Tuple => {
                    let mut imports = Vec::new();
                    let inner_types = variant
                        .fields
                        .into_iter()
                        .map(|field| {
                            self.solve_type(&TypeInfo {
                                generics,
                                ty: field.ty,
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()?
                        .into_iter()
                        .map(|(member, mut entries)| {
                            imports.append(&mut entries);
                            member
                        })
                        .collect();
                    Ok((
                        TsType::PrimaryType(PrimaryType::TupleType(TupleType { inner_types })),
                        imports,
                    ))
                }
                Style::Struct => {
                    let mut imports = Vec::new();
                    let members: Vec<TypeMember> = variant
                        .fields
                        .into_iter()
                        .map(|field| {
                            self.solve_member(&MemberInfo::from_generics_and_field(
                                generics, &field,
                            ))
                        })
                        .collect::<Result<Vec<_>, _>>()?
                        .into_iter()
                        .map(|(member, mut entries)| {
                            imports.append(&mut entries);
                            member
                        })
                        .collect();
                    Ok((
                        TsType::PrimaryType(PrimaryType::ObjectType(ObjectType {
                            body: TypeBody { members },
                        })),
                        imports,
                    ))
                }
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|(member, mut entries)| {
                imports.append(&mut entries);
                member
            })
            .collect();
        let inner_type = TsType::UnionType(UnionType { types });
        let type_params = extract_type_parameters(generics)?;
        let ident = TSIdent::from_str(&ident)?;
        Ok((
            vec![TypeAliasDeclaration {
                ident,
                inner_type,
                type_params,
            }
            .into()],
            imports,
        ))
    }

    fn export_enum_adjacent(
        &self,
        ident: String,
        generics: &Generics,
        variants: Vec<Variant>,
        tag: &str,
        content: &str,
    ) -> Result<(Vec<ExportStatement>, Vec<ImportEntry>), TsExportError> {
        let mut imports: Vec<ImportEntry> = Vec::new();
        let types: Vec<TsType> = variants
            .into_iter()
            .map(|variant| {
                let members: Vec<TypeMember> = variant
                    .fields
                    .into_iter()
                    .map(|field| {
                        let solver_info = MemberInfo::from_generics_and_field(generics, &field);
                        self.solve_member(&solver_info)
                    })
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter()
                    .map(|(member, mut entries)| {
                        imports.append(&mut entries);
                        member
                    })
                    .collect();

                let inner_type = match variant.style {
                    Style::Unit => None,
                    Style::Newtype => extract_inner_types(members).into_iter().next(),
                    Style::Tuple => {
                        let inner_types = extract_inner_types(members);
                        Some(TsType::PrimaryType(PrimaryType::TupleType(TupleType {
                            inner_types,
                        })))
                    }
                    Style::Struct => Some({
                        let members = members;
                        TsType::PrimaryType(PrimaryType::ObjectType(ObjectType {
                            body: TypeBody { members },
                        }))
                    }),
                };

                let content_member = inner_type.map(|inner_type| {
                    TypeMember::PropertySignature(PropertySignature {
                        name: PropertyName::from(content.to_string()),
                        inner_type,
                        optional: false,
                    })
                });

                let tag_member = TypeMember::PropertySignature(PropertySignature {
                    name: PropertyName::from(tag.to_string()),
                    inner_type: TsType::PrimaryType(PrimaryType::LiteralType(
                        LiteralType::StringLiteral(variant.attrs.name().serialize_name().into()),
                    )),
                    optional: false,
                });

                let members = Some(tag_member).into_iter().chain(content_member).collect();

                Ok(TsType::PrimaryType(PrimaryType::ObjectType(ObjectType {
                    body: TypeBody { members },
                })))
            })
            .collect::<Result<_, TsExportError>>()?;
        let inner_type = TsType::UnionType(UnionType { types });
        let type_params = extract_type_parameters(generics)?;
        let ident = TSIdent::from_str(&ident)?;
        Ok((
            vec![TypeAliasDeclaration {
                ident,
                inner_type,
                type_params,
            }
            .into()],
            imports,
        ))
    }

    fn export_enum_external(
        &self,
        ident: String,
        generics: &Generics,
        variants: Vec<Variant>,
    ) -> Result<(Vec<ExportStatement>, Vec<ImportEntry>), TsExportError> {
        let mut imports = Vec::new();
        let types: Vec<TsType> = variants
            .into_iter()
            .map(|variant| {
                let variant_name = variant.attrs.name().serialize_name();
                let container = match (variant.style, variant.fields.as_slice()) {
                    (Style::Unit, []) => TsType::PrimaryType(PrimaryType::LiteralType(
                        LiteralType::StringLiteral(variant_name.into()),
                    )),
                    (Style::Newtype, [field]) => {
                        let (inner_type, mut entries) = self.solve_type(&TypeInfo {
                            generics,
                            ty: &field.ty,
                        })?;
                        imports.append(&mut entries);

                        TsType::PrimaryType(PrimaryType::ObjectType(ObjectType {
                            body: TypeBody {
                                members: vec![TypeMember::PropertySignature(PropertySignature {
                                    inner_type,
                                    optional: false,
                                    name: PropertyName::StringLiteral(variant_name.into()),
                                })],
                            },
                        }))
                    }
                    (Style::Struct, fields) => {
                        let members: Vec<TypeMember> = fields
                            .iter()
                            .map(|field| {
                                self.solve_member(&MemberInfo::from_generics_and_field(
                                    generics, field,
                                ))
                            })
                            .collect::<Result<Vec<_>, _>>()?
                            .into_iter()
                            .map(|(member, mut entries)| {
                                imports.append(&mut entries);
                                member
                            })
                            .collect();
                        let inner_type = TsType::PrimaryType(PrimaryType::ObjectType(ObjectType {
                            body: TypeBody { members },
                        }));
                        TsType::PrimaryType(PrimaryType::ObjectType(ObjectType {
                            body: TypeBody {
                                members: vec![TypeMember::PropertySignature(PropertySignature {
                                    inner_type,
                                    optional: false,
                                    name: PropertyName::StringLiteral(variant_name.into()),
                                })],
                            },
                        }))
                    }
                    (Style::Tuple, fields) => {
                        let inner_types: Vec<TsType> = fields
                            .iter()
                            .map(|field| {
                                self.solve_type(&TypeInfo {
                                    generics,
                                    ty: &field.ty,
                                })
                            })
                            .collect::<Result<Vec<_>, _>>()?
                            .into_iter()
                            .map(|(member, mut entries)| {
                                imports.append(&mut entries);
                                member
                            })
                            .collect();
                        let inner_type =
                            TsType::PrimaryType(PrimaryType::TupleType(TupleType { inner_types }));
                        TsType::PrimaryType(PrimaryType::ObjectType(ObjectType {
                            body: TypeBody {
                                members: vec![TypeMember::PropertySignature(PropertySignature {
                                    inner_type,
                                    optional: false,
                                    name: PropertyName::StringLiteral(variant_name.into()),
                                })],
                            },
                        }))
                    }
                    _ => return Err(TsExportError::MalformedInput),
                };
                Ok(container)
            })
            .collect::<Result<_, TsExportError>>()?;
        let inner_type = TsType::UnionType(UnionType { types });
        let type_params = extract_type_parameters(generics)?;
        let ident = TSIdent::from_str(&ident)?;
        Ok((
            vec![TypeAliasDeclaration {
                ident,
                inner_type,
                type_params,
            }
            .into()],
            imports,
        ))
    }
}

fn extract_inner_types(members: Vec<TypeMember>) -> Vec<TsType> {
    members
        .into_iter()
        .map(|member| match member {
            TypeMember::PropertySignature(PropertySignature {
                name: _name,
                inner_type,
                optional: _optional,
            }) => inner_type,
        })
        .collect()
}
