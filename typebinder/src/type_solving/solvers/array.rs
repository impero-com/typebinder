use crate::{
    contexts::exporter::ExporterContext,
    error::TsExportError,
    type_solving::{result::Solved, SolverResult, TypeInfo, TypeSolver},
};
use syn::Type;
use ts_json_subset::types::{ArrayType, PrimaryType, TsType};

/// Solver for the Array type variant
/// Solves both Array and Slices
pub struct ArraySolver;

impl TypeSolver for ArraySolver {
    fn solve_as_type(
        &self,
        solving_context: &ExporterContext,
        solver_info: &TypeInfo,
    ) -> SolverResult<TsType, TsExportError> {
        let result = match solver_info.ty {
            Type::Array(ty) => solving_context.solve_type(&TypeInfo {
                generics: solver_info.generics,
                ty: ty.elem.as_ref(),
            }),
            Type::Slice(ty) => solving_context.solve_type(&TypeInfo {
                generics: solver_info.generics,
                ty: ty.elem.as_ref(),
            }),
            _ => {
                return SolverResult::Continue;
            }
        };

        match result {
            Ok(Solved {
                inner: TsType::PrimaryType(primary),
                import_entries,
                generic_constraints,
            }) => SolverResult::Solved(Solved {
                inner: TsType::PrimaryType(PrimaryType::ArrayType(ArrayType::new(primary))),
                import_entries,
                generic_constraints,
            }),
            // TODO: This is maybe unreachable ?
            Ok(Solved { inner, .. }) => SolverResult::Error(TsExportError::UnexpectedType(inner)),
            Err(e) => SolverResult::Error(e),
        }
    }
}
