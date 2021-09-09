use std::str::FromStr;

use syn::{GenericArgument, Generics, PathArguments, Type, TypePath};
use ts_json_subset::{
    ident::StrictTSIdent,
    types::{
        PrimaryType, PropertyName, PropertySignature, TsType, TypeArguments, TypeMember,
        TypeReference,
    },
};

use crate::{
    contexts::exporter::ExporterContext,
    error::TsExportError,
    type_solving::ImportEntry,
    type_solving::{
        generic_constraints::GenericConstraints, member_info::MemberInfo, result::Solved,
    },
    type_solving::{SolverResult, TypeInfo, TypeSolver},
    utils::display_path::DisplayPath,
};

/// The last solver of the pipeline. It recurses after trying to solve the type using
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
                        if ty_import_dp == ty_path_dp {
                            log::warn!("This type exists in the import");
                            // This type exists in the import and no further information about the path can be obtained,
                            // so it is a special case that we must handle
                            match solve_type_path(solving_context, generics, ty_path.clone()) {
                                Ok(solved) => return SolverResult::Solved(solved),
                                Err(e) => return SolverResult::Error(e),
                            }
                        }

                        // We got more information from the imports !
                        // Try to recurse through all solvers again
                        match solving_context.solve_type(&TypeInfo {
                            generics,
                            ty: &Type::Path(ty_import),
                        }) {
                            Ok(solved) => SolverResult::Solved(solved),
                            Err(e) => SolverResult::Error(e),
                        }
                    }
                    None => match solve_type_path(solving_context, generics, ty_path.clone()) {
                        Ok(solved) => SolverResult::Solved(solved),
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
        let MemberInfo {
            generics,
            ty,
            name,
            field,
            serde_field,
        } = solver_info;
        match ty {
            Type::Path(ty_path) => {
                // TODO: import_context.solver_import returns a TypePath anyway
                match solving_context.import_context.solve_import(ty_path) {
                    Some(Type::Path(ty_import)) => {
                        let ty_import_dp = DisplayPath(&ty_import.path).to_string();
                        let ty_path_dp = DisplayPath(&ty_path.path).to_string();
                        if ty_import_dp == ty_path_dp {
                            // This type exists in the import and no further information about the path can be obtained,
                            // so it is a special case that we must handle
                            match solve_type_path(solving_context, generics, ty_path.clone()) {
                                Ok(solved) => {
                                    return SolverResult::Solved(solved.map(|ts_type| {
                                        TypeMember::PropertySignature(PropertySignature {
                                            inner_type: ts_type,
                                            name: PropertyName::from(name.to_string()),
                                            optional: false,
                                        })
                                    }));
                                }
                                Err(e) => return SolverResult::Error(e),
                            }
                        }

                        let member_info = MemberInfo {
                            generics,
                            ty: &Type::Path(ty_import),
                            field,
                            name: name.to_string(),
                            serde_field,
                        };

                        match solving_context.solve_member(&member_info) {
                            Ok(solved) => SolverResult::Solved(solved),
                            Err(e) => SolverResult::Error(e),
                        }
                    }
                    None => match solve_type_path(solving_context, generics, ty_path.clone()) {
                        Ok(solved) => SolverResult::Solved(solved.map(|ts_type| {
                            TypeMember::PropertySignature(PropertySignature {
                                inner_type: ts_type,
                                name: PropertyName::from(name.to_string()),
                                optional: false,
                            })
                        })),
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
) -> Result<Solved<TsType>, TsExportError> {
    let segment = ty_path.path.segments.last().expect("Empty path");
    let ident = StrictTSIdent::from_str(&segment.ident.to_string())?;
    let mut imports: Vec<ImportEntry> = Vec::new();
    let mut constraints = GenericConstraints::default();

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
            .map(
                |Solved {
                     inner,
                     mut import_entries,
                     generic_constraints,
                 }| {
                    imports.append(&mut import_entries);
                    constraints.merge(generic_constraints);
                    inner
                },
            )
            .collect(),
        _ => Vec::new(),
    };

    let args = if types.is_empty() {
        None
    } else {
        Some(TypeArguments { types })
    };

    Ok(Solved {
        inner: TsType::PrimaryType(PrimaryType::TypeReference(TypeReference {
            name: ident,
            args,
        })),
        import_entries: imports,
        generic_constraints: constraints,
    })
}
