use syn::{GenericArgument, Generics, PathArguments, Type, TypePath};
use ts_json_subset::types::{PrimaryType, TsType, TypeArguments, TypeName, TypeReference};

use crate::{
    display_path::DisplayPath,
    error::TsExportError,
    exporter_context::ExporterContext,
    type_solver::{SolverResult, TypeInfo, TypeSolver},
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
                        if ty_import_dp == ty_path_dp {
                            // This type exists in the import and no further information about the path can be obtained,
                            // so it is a special case that we must handl
                            match solve_type_path(solving_context, generics, ty_path.clone()) {
                                Ok(ts_type) => return SolverResult::Solved(ts_type),
                                Err(e) => return SolverResult::Error(e),
                            }
                        }

                        // We got more information from the imports !
                        // Try to recurse through all solvers again
                        match solving_context.solve_type(&TypeInfo {
                            generics,
                            ty: &Type::Path(ty_import.clone()),
                        }) {
                            Ok(ts_type) => SolverResult::Solved(ts_type),
                            Err(e) => SolverResult::Error(e),
                        }
                    }
                    None => match solve_type_path(solving_context, generics, ty_path.clone()) {
                        Ok(ts_type) => SolverResult::Solved(ts_type),
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
) -> Result<TsType, TsExportError> {
    let segment = ty_path.path.segments.last().expect("Empty path");
    let ident = segment.ident.to_string();
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
            .collect(),
        _ => Ok(Vec::new()),
    }?;

    let args = if types.is_empty() {
        None
    } else {
        Some(TypeArguments { types })
    };

    Ok(TsType::PrimaryType(PrimaryType::TypeReference(
        TypeReference {
            name: TypeName {
                ident,
                namespace: None,
            },
            args,
        },
    )))
}
