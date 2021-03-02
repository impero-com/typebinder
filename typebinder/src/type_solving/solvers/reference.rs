use crate::{
    contexts::exporter::ExporterContext,
    error::TsExportError,
    type_solving::{SolverResult, TypeInfo, TypeSolver},
};
use syn::Type;
use ts_json_subset::types::TsType;

/// A solver for a Reference.
/// Just recurses by passing the inner type through.
///
/// When serializing, serde will treat references as a no-op
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
                    Ok((t, imports)) => SolverResult::Solved(t, imports),
                    Err(e) => SolverResult::Error(e),
                }
            }
            _ => SolverResult::Continue,
        }
    }
}
