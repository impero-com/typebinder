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

use crate::{
    error::TsExportError,
    import::ImportContext,
    macros::{context::MacroSolvingContext, MacroInfo},
    type_solver::{ImportEntry, MemberInfo, SolverResult, TypeInfo, TypeSolvingContext},
};

pub struct ExporterContext<'a> {
    pub solving_context: &'a TypeSolvingContext,
    pub macro_context: &'a MacroSolvingContext,
    pub import_context: ImportContext,
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

impl ExporterContext<'_> {
    pub fn solve_type(
        &self,
        solver_info: &TypeInfo,
    ) -> Result<(TsType, Vec<ImportEntry>), TsExportError> {
        for solver in self.solving_context.solvers() {
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
        for solver in self.solving_context.solvers() {
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
        // TODO: maybe have an error variant ?
        Ok((Vec::new(), Vec::new()))
    }

    pub fn export_statements_from_container(
        &self,
        container: Container,
    ) -> Result<(Vec<ExportStatement>, Vec<ImportEntry>), TsExportError> {
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
        let ident = type_alias.ident.to_string();
        let params = extract_type_parameters(&type_alias.generics);
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
                        params,
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
                // TODO: Handle skip_serializing_if.
                // How ? : mark TypeMember as optional if skip_seriazing_if is `Some`
                if field.attrs.skip_serializing() {
                    return None;
                }
                let solver_info = MemberInfo::from_generics_and_field(generics, field);
                Some(self.solve_member(&solver_info))
            })
            .collect::<Result<Vec<(TypeMember, Vec<ImportEntry>)>, TsExportError>>()?
            .into_iter()
            .map(|(member, mut entries)| {
                imports.append(&mut entries);
                member
            })
            .collect();
        let type_params = extract_type_parameters(generics);
        Ok((
            vec![ExportStatement::InterfaceDeclaration(
                InterfaceDeclaration {
                    ident,
                    extends_clause: None,
                    type_params,
                    obj_type: ObjectType {
                        body: Some(TypeBody { members }),
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
        let params = extract_type_parameters(generics);
        self.solve_type(&solver_info).map(|(inner_type, imports)| {
            (
                vec![TypeAliasDeclaration {
                    ident,
                    inner_type,
                    params,
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
        let params = extract_type_parameters(generics);

        Ok((
            vec![TypeAliasDeclaration {
                ident,
                inner_type,
                params,
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
                let mut members: Vec<TypeMember> = variant
                    .fields
                    .into_iter()
                    .map(|field| {
                        // TODO: Filter Variants which have unnamed members since those will fail to serialize
                        let solver_info = MemberInfo::from_generics_and_field(generics, field);
                        self.solve_member(&solver_info)
                    })
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter()
                    .map(|(member, mut entries)| {
                        imports.append(&mut entries);
                        member
                    })
                    .collect();
                members.push(TypeMember::PropertySignature(PropertySignature {
                    name: PropertyName::Identifier(tag.to_string()),
                    inner_type: TsType::PrimaryType(PrimaryType::LiteralType(
                        LiteralType::StringLiteral(variant.attrs.name().serialize_name().into()),
                    )),
                    optional: false,
                }));
                Ok(TsType::PrimaryType(PrimaryType::ObjectType(ObjectType {
                    body: Some(TypeBody { members }),
                })))
            })
            .collect::<Result<_, TsExportError>>()?;
        let params = extract_type_parameters(generics);

        Ok((
            vec![ExportStatement::TypeAliasDeclaration(
                TypeAliasDeclaration {
                    ident,
                    inner_type: TsType::UnionType(UnionType { types }),
                    params,
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
                            self.solve_member(&MemberInfo::from_generics_and_field(generics, field))
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
                            body: Some(TypeBody { members }),
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
        let params = extract_type_parameters(generics);
        Ok((
            vec![TypeAliasDeclaration {
                ident,
                inner_type,
                params,
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
                        // TODO: Filter Variants which have unnamed members since those will fail to serialize
                        let solver_info = MemberInfo::from_generics_and_field(generics, field);
                        self.solve_member(&solver_info)
                    })
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter()
                    .map(|(member, mut entries)| {
                        imports.append(&mut entries);
                        member
                    })
                    .collect();
                let content_member = TypeMember::PropertySignature(PropertySignature {
                    name: PropertyName::Identifier(content.to_string()),
                    inner_type: wrap_members(members),
                    optional: false,
                });
                let tag_member = TypeMember::PropertySignature(PropertySignature {
                    name: PropertyName::Identifier(tag.to_string()),
                    inner_type: TsType::PrimaryType(PrimaryType::LiteralType(
                        LiteralType::StringLiteral(variant.attrs.name().serialize_name().into()),
                    )),
                    optional: false,
                });
                Ok(TsType::PrimaryType(PrimaryType::ObjectType(ObjectType {
                    body: Some(TypeBody {
                        members: vec![tag_member, content_member],
                    }),
                })))
            })
            .collect::<Result<_, TsExportError>>()?;
        let inner_type = TsType::UnionType(UnionType { types });
        let params = extract_type_parameters(generics);
        Ok((
            vec![TypeAliasDeclaration {
                ident,
                inner_type,
                params,
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
                let members: Vec<TypeMember> = variant
                    .fields
                    .into_iter()
                    .map(|field| {
                        self.solve_member(&MemberInfo::from_generics_and_field(generics, field))
                    })
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter()
                    .map(|(member, mut entries)| {
                        imports.append(&mut entries);
                        member
                    })
                    .collect();
                let members_empty = members.is_empty();
                let inner_type = wrap_members(members);
                let container = if members_empty {
                    TsType::PrimaryType(PrimaryType::LiteralType(LiteralType::StringLiteral(
                        variant_name.into(),
                    )))
                } else {
                    TsType::PrimaryType(PrimaryType::ObjectType(ObjectType {
                        body: Some(TypeBody {
                            members: vec![TypeMember::PropertySignature(PropertySignature {
                                inner_type,
                                optional: false,
                                name: PropertyName::StringLiteral(variant_name.into()),
                            })],
                        }),
                    }))
                };
                Ok(container)
            })
            .collect::<Result<_, TsExportError>>()?;
        let inner_type = TsType::UnionType(UnionType { types });
        let params = extract_type_parameters(generics);
        Ok((
            vec![TypeAliasDeclaration {
                ident,
                inner_type,
                params,
            }
            .into()],
            imports,
        ))
    }
}

fn wrap_members(members: Vec<TypeMember>) -> TsType {
    if members.is_empty() {
        TsType::PrimaryType(PrimaryType::ObjectType(ObjectType { body: None }))
    } else {
        TsType::PrimaryType(PrimaryType::ObjectType(ObjectType {
            body: Some(TypeBody { members }),
        }))
    }
}
