/// When serializing, serde will treat references as a no-op
use crate::{
    error::TsExportError,
    exporter::ExporterContext,
    type_solver::{SolverResult, TypeInfo, TypeSolver},
};
use syn::Type;
use ts_json_subset::types::TsType;

pub struct ReferenceSolver;

impl TypeSolver for ReferenceSolver {
    fn solve_as_type(
        &self,
        solving_context: &ExporterContext,
        solver_info: &TypeInfo,
    ) -> SolverResult<TsType, TsExportError> {
        let TypeInfo { generics, ty } = solver_info;
        match ty {
            Type::Reference(ty) => {
                let ty = ty.elem.as_ref();
                match solving_context.solve_type(&TypeInfo { generics, ty }) {
                    Ok(t) => SolverResult::Solved(t),
                    Err(e) => SolverResult::Error(e),
                }
            }
            _ => SolverResult::Continue,
        }
    }
}
