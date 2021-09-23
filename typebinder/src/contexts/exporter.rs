use std::str::FromStr;

use super::{import::ImportContext, type_solving::TypeSolvingContext};
use crate::{
    error::TsExportError,
    macros::{context::MacroSolvingContext, MacroInfo},
    type_solving::{
        generic_constraints::GenericConstraints, member_info::MemberInfo, result::SolverResult,
        type_info::TypeInfo,
    },
    type_solving::{result::Solved, ImportEntry},
};
use serde_derive_internals::{
    ast::{Container, Data, Field, Style, Variant},
    attr::TagType,
};
use syn::{GenericParam, Generics, ItemType};
use ts_json_subset::{
    declarations::{interface::InterfaceDeclaration, type_alias::TypeAliasDeclaration},
    export::ExportStatement,
    ident::{IdentError, StrictTSIdent, TSIdent},
    types::{
        IntersectionType, LiteralType, ObjectType, ParenthesizedType, PrimaryType, PropertyName,
        PropertySignature, TsType, TupleType, TypeBody, TypeMember, TypeParameter, TypeParameters,
        UnionType,
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

pub fn apply_generic_constraints(
    parameters: &mut TypeParameters,
    constraints: &GenericConstraints,
) {
    parameters
        .parameters
        .iter_mut()
        .for_each(|param| param.constraint = constraints.get_constraints(&param.identifier))
}

fn extract_type_parameters(generics: &Generics) -> Result<Option<TypeParameters>, IdentError> {
    // TODO: rename to parameters
    let identifiers: Vec<TSIdent> = generics
        .params
        .iter()
        .filter_map(|param| match param {
            // TODO: generate a TypeParameter instead
            GenericParam::Type(ty) => Some(TSIdent::from_str(&ty.ident.to_string())),
            _ => None,
        })
        .collect::<Result<_, _>>()?;

    if identifiers.is_empty() {
        Ok(None)
    } else {
        let parameters = identifiers
            .into_iter()
            .map(|identifier| TypeParameter {
                identifier,
                constraint: None,
            })
            .collect();
        Ok(Some(TypeParameters { parameters }))
    }
}

impl ExporterContext<'_> {
    pub fn solve_type(&self, solver_info: &TypeInfo) -> Result<Solved<TsType>, TsExportError> {
        for solver in self.type_solving_context.solvers() {
            match solver.as_ref().solve_as_type(&self, solver_info) {
                SolverResult::Continue => (),
                SolverResult::Solved(solved) => return Ok(solved),
                SolverResult::Error(inner) => return Err(inner),
            }
        }
        Err(TsExportError::UnsolvedType(solver_info.ty.clone()))
    }

    pub fn solve_member(
        &self,
        solver_info: &MemberInfo,
    ) -> Result<Solved<TypeMember>, TsExportError> {
        for solver in self.type_solving_context.solvers() {
            match solver.as_ref().solve_as_member(&self, solver_info) {
                SolverResult::Continue => (),
                SolverResult::Solved(solved) => return Ok(solved),
                SolverResult::Error(inner) => return Err(inner),
            }
        }
        Err(TsExportError::UnsolvedField(solver_info.field.clone()))
    }

    pub fn export_statements_from_macro(
        &self,
        macro_info: &MacroInfo,
    ) -> Result<Solved<Vec<ExportStatement>>, TsExportError> {
        for solver in self.macro_context.solvers() {
            match solver.as_ref().solve_macro(macro_info) {
                SolverResult::Continue => (),
                SolverResult::Solved(solved) => return Ok(solved.map(|inner| vec![inner])),
                SolverResult::Error(inner) => return Err(inner),
            }
        }
        // TODO: Maybe have an error variant ?
        Ok(Solved::default())
    }

    pub fn export_statements_from_container(
        &self,
        container: Container,
    ) -> Result<Solved<Vec<ExportStatement>>, TsExportError> {
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
                Style::Unit => Ok(Solved::new(vec![])), // Unit structs are a no-op because they dont have a TS representation
                Style::Newtype => self.export_struct_newtype(name, container.generics, fields),
                Style::Tuple => self.export_struct_tuple(name, container.generics, fields),
                Style::Struct => self.export_struct_struct(name, container.generics, fields),
            },
        }
    }

    pub fn export_statements_from_type_alias(
        &self,
        type_alias: ItemType,
    ) -> Result<Solved<Vec<ExportStatement>>, TsExportError> {
        let ident = StrictTSIdent::from_str(&type_alias.ident.to_string())?;
        let solver_info = TypeInfo {
            generics: &type_alias.generics,
            ty: type_alias.ty.as_ref(),
        };
        let solved = self.solve_type(&solver_info)?;
        // TODO: Or maybe apply_generic_constraints inside extract_type_parameters ?
        let mut type_params = extract_type_parameters(&type_alias.generics)?;
        if let Some(params) = type_params.as_mut() {
            apply_generic_constraints(params, &solved.generic_constraints);
        }
        Ok(solved.map(move |inner_type| {
            vec![ExportStatement::TypeAliasDeclaration(
                TypeAliasDeclaration {
                    ident,
                    inner_type,
                    type_params,
                },
            )]
        }))
    }

    fn export_struct_struct(
        &self,
        ident: String,
        generics: &Generics,
        fields: Vec<Field>,
    ) -> Result<Solved<Vec<ExportStatement>>, TsExportError> {
        let mut imports = Vec::new();
        let mut constraints = GenericConstraints::default();
        let members: Vec<TypeMember> = fields
            .into_iter()
            .filter_map(|field| {
                if field.attrs.skip_serializing() {
                    return None;
                }
                let solver_info = MemberInfo::from_generics_and_field(generics, &field);
                Some(self.solve_member(&solver_info))
            })
            .collect::<Result<Vec<Solved<TypeMember>>, TsExportError>>()?
            .into_iter()
            .map(|mut solved| {
                imports.append(&mut solved.import_entries);
                constraints.merge(solved.generic_constraints);
                solved.inner
            })
            .collect();
        let mut type_params = extract_type_parameters(generics)?;
        if let Some(params) = type_params.as_mut() {
            apply_generic_constraints(params, &constraints);
        }
        let ident = StrictTSIdent::from_str(&ident)?;
        Ok(Solved {
            inner: vec![ExportStatement::InterfaceDeclaration(
                InterfaceDeclaration {
                    ident,
                    extends_clause: None,
                    type_params,
                    obj_type: ObjectType {
                        body: TypeBody { members },
                    },
                },
            )],
            import_entries: imports,
            generic_constraints: constraints,
        })
    }

    fn export_struct_newtype(
        &self,
        ident: String,
        generics: &Generics,
        fields: Vec<Field>,
    ) -> Result<Solved<Vec<ExportStatement>>, TsExportError> {
        let field = &fields[0];
        let solver_info = TypeInfo {
            generics,
            ty: field.ty,
        };
        let solved = self.solve_type(&solver_info)?;
        let mut type_params = extract_type_parameters(generics)?;
        if let Some(params) = type_params.as_mut() {
            apply_generic_constraints(params, &solved.generic_constraints);
        }
        let ident = StrictTSIdent::from_str(&ident)?;
        Ok(solved.map(|inner_type| {
            vec![TypeAliasDeclaration {
                ident,
                inner_type,
                type_params,
            }
            .into()]
        }))
    }

    fn export_struct_tuple(
        &self,
        ident: String,
        generics: &Generics,
        fields: Vec<Field>,
    ) -> Result<Solved<Vec<ExportStatement>>, TsExportError> {
        let mut imports: Vec<ImportEntry> = Vec::new();
        let mut constraints = GenericConstraints::default();
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
            .map(|mut solved| {
                imports.append(&mut solved.import_entries);
                constraints.merge(solved.generic_constraints);
                solved.inner
            })
            .collect();
        let inner_type = TsType::PrimaryType(PrimaryType::TupleType(TupleType { inner_types }));
        let mut type_params = extract_type_parameters(generics)?;
        if let Some(params) = type_params.as_mut() {
            apply_generic_constraints(params, &constraints);
        }
        let ident = StrictTSIdent::from_str(&ident)?;
        Ok(Solved {
            inner: vec![TypeAliasDeclaration {
                ident,
                inner_type,
                type_params,
            }
            .into()],
            import_entries: imports,
            generic_constraints: constraints,
        })
    }

    fn export_enum_internal(
        &self,
        ident: String,
        generics: &Generics,
        variants: Vec<Variant>,
        tag: &str,
    ) -> Result<Solved<Vec<ExportStatement>>, TsExportError> {
        let mut imports: Vec<ImportEntry> = Vec::new();
        let mut constraints = GenericConstraints::default();

        let types: Vec<TsType> = variants
            .into_iter()
            .map(|variant| {
                // See serde's TaggedSerializer in order to understand the big match here, namely :
                // * Tuples are not supported
                // * Unit structs are no ops
                // * Structs are to be exported field by field in a TS ObjectType
                // * Newtypes have most types unsupported, but does support :
                //  * Exporting a map
                //  * Exporting a struct
                //  * (References are supported because they are assumed not to deserialize into a
                //     supported type here)
                let variant_type = match (variant.style, variant.fields.as_slice()) {
                    (Style::Tuple, _) => {
                        return Err(TsExportError::InvalidSerdeRepresentation(format!(
                            "{}::{}",
                            ident, variant.ident
                        )));
                    }
                    (Style::Newtype, fields) => {
                        let solver_info = MemberInfo::from_generics_and_field(generics, &fields[0]);
                        let mut solved = self.solve_member(&solver_info)?;
                        imports.append(&mut solved.import_entries);
                        constraints.merge(solved.generic_constraints);
                        let TypeMember::PropertySignature(property) = solved.inner;
                        match property.inner_type {
                            TsType::PrimaryType(ref primary) => match primary {
                                PrimaryType::ObjectType(_) => Some(property.inner_type),
                                PrimaryType::TypeReference(_) => Some(property.inner_type),
                                PrimaryType::ArrayType(_)
                                | PrimaryType::TupleType(_)
                                | PrimaryType::Predefined(_)
                                | PrimaryType::LiteralType(_) => {
                                    return Err(TsExportError::InvalidSerdeRepresentation(
                                        format!("{}::{}", ident, variant.ident),
                                    ));
                                }
                            },
                            _ => Some(property.inner_type),
                        }
                    }
                    (Style::Unit, []) => None,
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
                            .map(|mut solved| {
                                imports.append(&mut solved.import_entries);
                                constraints.merge(solved.generic_constraints);
                                solved.inner
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
        let mut type_params = extract_type_parameters(generics)?;
        if let Some(params) = type_params.as_mut() {
            apply_generic_constraints(params, &constraints);
        }
        let ident = StrictTSIdent::from_str(&ident)?;
        Ok(Solved {
            inner: vec![ExportStatement::TypeAliasDeclaration(
                TypeAliasDeclaration {
                    ident,
                    inner_type: TsType::UnionType(UnionType { types }),
                    type_params,
                },
            )],
            import_entries: imports,
            generic_constraints: constraints,
        })
    }

    fn export_enum_untagged(
        &self,
        ident: String,
        generics: &Generics,
        variants: Vec<Variant>,
    ) -> Result<Solved<Vec<ExportStatement>>, TsExportError> {
        let mut imports: Vec<ImportEntry> = Vec::new();
        let mut constraints = GenericConstraints::default();
        let types: Vec<TsType> = variants
            .into_iter()
            .map(|variant| match variant.style {
                Style::Unit => Ok(Solved::new(TsType::PrimaryType(PrimaryType::Predefined(
                    ts_json_subset::types::PredefinedType::Null,
                )))),
                Style::Newtype => {
                    let field = &variant.fields[0];
                    self.solve_type(&TypeInfo {
                        generics,
                        ty: field.ty,
                    })
                }
                Style::Tuple => {
                    let mut imports = Vec::new();
                    let mut constraints = GenericConstraints::default();
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
                        .map(|mut solved| {
                            imports.append(&mut solved.import_entries);
                            constraints.merge(solved.generic_constraints);
                            solved.inner
                        })
                        .collect();
                    Ok(Solved {
                        inner: TsType::PrimaryType(PrimaryType::TupleType(TupleType {
                            inner_types,
                        })),
                        import_entries: imports,
                        generic_constraints: constraints,
                    })
                }
                Style::Struct => {
                    let mut imports = Vec::new();
                    let mut constraints = GenericConstraints::default();
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
                        .map(|mut solved| {
                            imports.append(&mut solved.import_entries);
                            constraints.merge(solved.generic_constraints);
                            solved.inner
                        })
                        .collect();
                    Ok(Solved {
                        inner: TsType::PrimaryType(PrimaryType::ObjectType(ObjectType {
                            body: TypeBody { members },
                        })),
                        import_entries: imports,
                        generic_constraints: constraints,
                    })
                }
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|mut solved| {
                imports.append(&mut solved.import_entries);
                constraints.merge(solved.generic_constraints);
                solved.inner
            })
            .collect();
        let inner_type = TsType::UnionType(UnionType { types });
        let mut type_params = extract_type_parameters(generics)?;
        if let Some(params) = type_params.as_mut() {
            apply_generic_constraints(params, &constraints);
        }
        let ident = StrictTSIdent::from_str(&ident)?;
        Ok(Solved {
            inner: vec![TypeAliasDeclaration {
                ident,
                inner_type,
                type_params,
            }
            .into()],
            import_entries: imports,
            generic_constraints: constraints,
        })
    }

    fn export_enum_adjacent(
        &self,
        ident: String,
        generics: &Generics,
        variants: Vec<Variant>,
        tag: &str,
        content: &str,
    ) -> Result<Solved<Vec<ExportStatement>>, TsExportError> {
        let mut imports: Vec<ImportEntry> = Vec::new();
        let mut constraints = GenericConstraints::default();
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
                    .map(|mut solved| {
                        imports.append(&mut solved.import_entries);
                        constraints.merge(solved.generic_constraints);
                        solved.inner
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
        let mut type_params = extract_type_parameters(generics)?;
        if let Some(params) = type_params.as_mut() {
            apply_generic_constraints(params, &constraints);
        }
        let ident = StrictTSIdent::from_str(&ident)?;
        Ok(Solved {
            inner: vec![TypeAliasDeclaration {
                ident,
                inner_type,
                type_params,
            }
            .into()],
            import_entries: imports,
            generic_constraints: constraints,
        })
    }

    fn export_enum_external(
        &self,
        ident: String,
        generics: &Generics,
        variants: Vec<Variant>,
    ) -> Result<Solved<Vec<ExportStatement>>, TsExportError> {
        let mut imports = Vec::new();
        let mut constraints = GenericConstraints::default();
        let types: Vec<TsType> = variants
            .into_iter()
            .map(|variant| {
                let variant_name = variant.attrs.name().serialize_name();
                let container = match (variant.style, variant.fields.as_slice()) {
                    (Style::Unit, []) => TsType::PrimaryType(PrimaryType::LiteralType(
                        LiteralType::StringLiteral(variant_name.into()),
                    )),
                    (Style::Newtype, [field]) => {
                        let mut solved = self.solve_type(&TypeInfo {
                            generics,
                            ty: &field.ty,
                        })?;
                        imports.append(&mut solved.import_entries);
                        constraints.merge(solved.generic_constraints);

                        TsType::PrimaryType(PrimaryType::ObjectType(ObjectType {
                            body: TypeBody {
                                members: vec![TypeMember::PropertySignature(PropertySignature {
                                    inner_type: solved.inner,
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
                            .map(|mut solved| {
                                imports.append(&mut solved.import_entries);
                                constraints.merge(solved.generic_constraints);
                                solved.inner
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
                            .map(|mut solved| {
                                imports.append(&mut solved.import_entries);
                                constraints.merge(solved.generic_constraints);
                                solved.inner
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
        let mut type_params = extract_type_parameters(generics)?;
        if let Some(params) = type_params.as_mut() {
            apply_generic_constraints(params, &constraints);
        }
        let ident = StrictTSIdent::from_str(&ident)?;
        Ok(Solved {
            inner: vec![TypeAliasDeclaration {
                ident,
                inner_type,
                type_params,
            }
            .into()],
            import_entries: imports,
            generic_constraints: constraints,
        })
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
