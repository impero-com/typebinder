use ts_json_subset::types::TsType;

use crate::{
    error::TsExportError,
    type_solver::{SolverResult, TypeInfo, TypeSolver, TypeSolvingContext},
};

pub struct FnSolver<F>(F);

impl<F> TypeSolver for FnSolver<F>
where
    F: Fn(&TypeSolvingContext, &TypeInfo) -> SolverResult<TsType, TsExportError>,
{
    fn solve_as_type(
        &self,
        solving_context: &TypeSolvingContext,
        solver_info: &TypeInfo,
    ) -> SolverResult<TsType, TsExportError> {
        self.0(solving_context, solver_info)
    }
}

pub trait AsSolver: Sized {
    fn as_solver(self) -> FnSolver<Self>;
}

impl<F> AsSolver for F
where
    F: Fn(&TypeSolvingContext, &TypeInfo) -> SolverResult<TsType, TsExportError>,
{
    fn as_solver(self) -> FnSolver<Self> {
        FnSolver(self)
    }
}
