use syn::{GenericArgument, PathArguments, Type};
use ts_json_subset::types::{PrimaryType, TsType, TypeArguments, TypeName, TypeReference};

use crate::{
    display_path::DisplayPath,
    error::TsExportError,
    exporter::ExporterContext,
    type_solver::{SolverResult, TypeInfo, TypeSolver},
};

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
                match solving_context.import_context.solve_import(ty_path) {
                    Ok(Type::Path(ty_import)) => {
                        let ty_import_dp = DisplayPath(&ty_import.path).to_string();
                        let ty_path_dp = DisplayPath(&ty_path.path).to_string();
                        if ty_import_dp == ty_path_dp {
                            // This type exists in the import and no further information about the path can be obtained,
                            // so it is a special case that we must handle
                            let segment = ty_path.path.segments.last().expect("Empty path");
                            let ident = segment.ident.to_string();
                            let types: Result<Vec<TsType>, TsExportError> = match &segment.arguments
                            {
                                PathArguments::AngleBracketed(inner_generics) => inner_generics
                                    .args
                                    .iter()
                                    .filter_map(|arg| match arg {
                                        GenericArgument::Type(ty) => Some(
                                            solving_context.solve_type(&TypeInfo { generics, ty }),
                                        ),
                                        _ => None,
                                    })
                                    .collect(),
                                _ => Ok(Vec::new()),
                            };

                            let args = match types {
                                Ok(types) if types.is_empty() => None,
                                Ok(types) => Some(TypeArguments { types }),
                                Err(e) => {
                                    return SolverResult::Error(e);
                                }
                            };

                            return SolverResult::Solved(TsType::PrimaryType(
                                PrimaryType::TypeReference(TypeReference {
                                    name: TypeName {
                                        ident,
                                        namespace: None,
                                    },
                                    args,
                                }),
                            ));
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
                    Err(e) => SolverResult::Error(e),
                    _ => unreachable!(),
                }
            }
            _ => SolverResult::Continue,
        }
    }
}
