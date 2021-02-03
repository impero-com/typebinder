/// Solver for :
/// * Vec<T>
/// * VecDeque<T>
/// * HashSet<T>
use crate::{
    display_path::DisplayPath,
    error::TsExportError,
    exporter::ExporterContext,
    type_solver::{SolverResult, TypeInfo, TypeSolver},
};
use syn::{GenericArgument, PathArguments, Type};
use ts_json_subset::types::{ArrayType, PrimaryType, TsType};

pub struct CollectionsSolver;

impl TypeSolver for CollectionsSolver {
    fn solve_as_type(
        &self,
        solving_context: &ExporterContext,
        solver_info: &TypeInfo,
    ) -> SolverResult<TsType, TsExportError> {
        let TypeInfo { generics, ty } = solver_info;
        let ty: Result<TsType, TsExportError> = match ty {
            Type::Path(ty) => {
                let ident = DisplayPath(&ty.path).to_string();
                let segment = ty.path.segments.last().expect("Empty path");
                match ident.as_str() {
                    "std::vec::Vec" | "std::vec::VecDeque" | "std::collections::HashSet" => {
                        match &segment.arguments {
                            PathArguments::AngleBracketed(inner_generics) => {
                                if let Some(first_arg) = inner_generics.args.first() {
                                    match first_arg {
                                        GenericArgument::Type(ty) => {
                                            solving_context.solve_type(&TypeInfo { generics, ty })
                                        }
                                        _ => return SolverResult::Continue,
                                    }
                                } else {
                                    Err(TsExportError::ExpectedGenerics)
                                }
                            }
                            _ => return SolverResult::Continue,
                        }
                    }
                    _ => return SolverResult::Continue,
                }
            }
            _ => return SolverResult::Continue,
        };
        match ty {
            Ok(TsType::PrimaryType(primary)) => SolverResult::Solved(TsType::PrimaryType(
                PrimaryType::ArrayType(ArrayType::new(primary)),
            )),
            Ok(ts_ty) => SolverResult::Error(TsExportError::UnexpectedType(ts_ty)),
            Err(e) => SolverResult::Error(e),
        }
    }
}
