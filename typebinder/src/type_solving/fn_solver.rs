use ts_json_subset::types::TsType;

use crate::{
    contexts::exporter::ExporterContext,
    error::TsExportError,
    type_solving::result::SolverResult,
    type_solving::{type_info::TypeInfo, TypeSolver},
};

pub struct FnSolver<F>(F);

impl<F> TypeSolver for FnSolver<F>
where
    F: Fn(&ExporterContext, &TypeInfo) -> SolverResult<TsType, TsExportError>,
{
    fn solve_as_type(
        &self,
        solving_context: &ExporterContext,
        solver_info: &TypeInfo,
    ) -> SolverResult<TsType, TsExportError> {
        self.0(solving_context, solver_info)
    }
}

pub trait AsFnSolver: Sized {
    fn fn_solver(self) -> FnSolver<Self>;
}

impl<F> AsFnSolver for F
where
    F: Fn(&ExporterContext, &TypeInfo) -> SolverResult<TsType, TsExportError>,
{
    fn fn_solver(self) -> FnSolver<Self> {
        FnSolver(self)
    }
}
