use serde_derive_internals::ast::Field;
use syn::{GenericArgument, Generics, PathArguments, Type, TypePath};
use ts_json_subset::types::{
    PrimaryType, PropertyName, PropertySignature, TsType, TypeArguments, TypeMember, TypeName,
    TypeReference,
};

use crate::{
    display_path::DisplayPath,
    error::TsExportError,
    exporter_context::ExporterContext,
    type_solver::{ImportEntry, MemberInfo, SolverResult, TypeInfo, TypeSolver},
};

/// The last solver, recurses after trying to solve the type using
/// the import context
pub struct ImportSolver;

impl TypeSolver for ImportSolver {
    fn solve_as_type(
        &self,
        solving_context: &ExporterContext,
        solver_info: &TypeInfo,
    ) -> SolverResult<TsType, TsExportError> {
        let TypeInfo { generics, ty } = solver_info;
        match ty {
            Type::Path(ty_path) => {
                // TODO: import_context.solver_import returns a TypePath anyway
                match solving_context.import_context.solve_import(ty_path) {
                    Some(Type::Path(ty_import)) => {
                        let ty_import_dp = DisplayPath(&ty_import.path).to_string();
                        let ty_path_dp = DisplayPath(&ty_path.path).to_string();
                        log::debug!("(type) ty_import is : {}", ty_import_dp);
                        log::debug!("(type) ty_path is : {}", ty_path_dp);
                        if ty_import_dp == ty_path_dp {
                            log::warn!("This type exists in the import");
                            // This type exists in the import and no further information about the path can be obtained,
                            // so it is a special case that we must handle
                            match solve_type_path(solving_context, generics, ty_path.clone()) {
                                Ok((ts_type, imports)) => {
                                    return SolverResult::Solved(ts_type, imports);
                                }
                                Err(e) => return SolverResult::Error(e),
                            }
                        }

                        // We got more information from the imports !
                        // Try to recurse through all solvers again
                        match solving_context.solve_type(&TypeInfo {
                            generics,
                            ty: &Type::Path(ty_import.clone()),
                        }) {
                            Ok((ts_type, imports)) => SolverResult::Solved(ts_type, imports),
                            Err(e) => SolverResult::Error(e),
                        }
                    }
                    None => match solve_type_path(solving_context, generics, ty_path.clone()) {
                        Ok((ts_type, imports)) => SolverResult::Solved(ts_type, imports),
                        Err(e) => SolverResult::Error(e),
                    },
                    _ => unreachable!(),
                }
            }
            _ => SolverResult::Continue,
        }
    }

    fn solve_as_member(
        &self,
        solving_context: &ExporterContext,
        solver_info: &MemberInfo,
    ) -> SolverResult<TypeMember, TsExportError> {
        let MemberInfo { generics, field } = solver_info;
        match field.ty {
            Type::Path(ty_path) => {
                // TODO: import_context.solver_import returns a TypePath anyway
                match solving_context.import_context.solve_import(ty_path) {
                    Some(Type::Path(ty_import)) => {
                        let ty_import_dp = DisplayPath(&ty_import.path).to_string();
                        let ty_path_dp = DisplayPath(&ty_path.path).to_string();
                        log::debug!("(member) ty_import is : {}", ty_import_dp);
                        log::debug!("(member) ty_path is : {}", ty_path_dp);
                        if ty_import_dp == ty_path_dp {
                            // This type exists in the import and no further information about the path can be obtained,
                            // so it is a special case that we must handle
                            match solve_type_path(solving_context, generics, ty_path.clone()) {
                                Ok((ts_type, imports)) => {
                                    return SolverResult::Solved(
                                        TypeMember::PropertySignature(PropertySignature {
                                            inner_type: ts_type,
                                            name: PropertyName::Identifier(
                                                field.attrs.name().serialize_name(),
                                            ),
                                            optional: false,
                                        }),
                                        imports,
                                    );
                                }
                                Err(e) => return SolverResult::Error(e),
                            }
                        }

                        // We got more information from the imports !
                        // Try to recurse through all solvers again
                        let new_field = Field {
                            attrs: field.attrs.clone(),
                            member: field.member.clone(),
                            original: field.original,
                            ty: &Type::Path(ty_import.clone()),
                        };

                        match solving_context.solve_member(&MemberInfo {
                            generics,
                            field: new_field,
                        }) {
                            Ok((ts_type, imports)) => SolverResult::Solved(ts_type, imports),
                            Err(e) => SolverResult::Error(e),
                        }
                    }
                    None => match solve_type_path(solving_context, generics, ty_path.clone()) {
                        Ok((ts_type, imports)) => {
                            return SolverResult::Solved(
                                TypeMember::PropertySignature(PropertySignature {
                                    inner_type: ts_type,
                                    name: PropertyName::Identifier(
                                        field.attrs.name().serialize_name(),
                                    ),
                                    optional: false,
                                }),
                                imports,
                            )
                        }
                        Err(e) => SolverResult::Error(e),
                    },
                    _ => unreachable!(),
                }
            }
            _ => SolverResult::Continue,
        }
    }
}

pub fn solve_type_path(
    solving_context: &ExporterContext,
    generics: &Generics,
    ty_path: TypePath,
) -> Result<(TsType, Vec<ImportEntry>), TsExportError> {
    let segment = ty_path.path.segments.last().expect("Empty path");
    let ident = segment.ident.to_string();
    let mut imports: Vec<ImportEntry> = Vec::new();

    let path_len = ty_path.path.segments.len();
    let path_segments: Vec<String> = ty_path
        .path
        .segments
        .iter()
        .take(path_len - 1)
        .map(|segm| segm.ident.to_string())
        .collect();
    let path = path_segments.join("::");
    let mut other_imports = vec![ImportEntry {
        ident: segment.ident.to_string(),
        path,
    }];
    imports.append(&mut other_imports);

    let types: Vec<TsType> = match &segment.arguments {
        PathArguments::AngleBracketed(inner_generics) => inner_generics
            .args
            .iter()
            .filter_map(|arg| match arg {
                GenericArgument::Type(ty) => {
                    Some(solving_context.solve_type(&TypeInfo { generics, ty }))
                }
                _ => None,
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|(ty, mut entries)| {
                imports.append(&mut entries);
                ty
            })
            .collect(),
        _ => Vec::new(),
    };

    let args = if types.is_empty() {
        None
    } else {
        Some(TypeArguments { types })
    };

    Ok((
        TsType::PrimaryType(PrimaryType::TypeReference(TypeReference {
            name: TypeName {
                ident,
                namespace: None,
            },
            args,
        })),
        imports,
    ))
}
