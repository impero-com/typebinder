use ts_json_subset::types::{PredefinedType, PrimaryType, TsType};

use super::path::PathSolver;
use crate::{
    contexts::exporter::ExporterContext,
    error::TsExportError,
    type_solving::fn_solver::AsFnSolver,
    type_solving::{SolverResult, TypeInfo, TypeSolver, TypeSolverExt},
};

pub struct ChronoSolver {
    inner: PathSolver,
}

fn solve_datetime(
    _solving_context: &ExporterContext,
    _solver_info: &TypeInfo,
) -> SolverResult<TsType, TsExportError> {
    SolverResult::Solved(
        TsType::PrimaryType(PrimaryType::Predefined(PredefinedType::String)),
        Vec::new(),
    )
}

impl Default for ChronoSolver {
    fn default() -> Self {
        let mut inner = PathSolver::default();
        inner.add_entry(
            "chrono::DateTime".to_string(),
            solve_datetime.fn_solver().into_rc(),
        );

        ChronoSolver { inner }
    }
}

impl TypeSolver for ChronoSolver {
    fn solve_as_type(
        &self,
        solving_context: &ExporterContext,
        solver_info: &TypeInfo,
    ) -> SolverResult<TsType, TsExportError> {
        self.inner.solve_as_type(solving_context, solver_info)
    }
}
