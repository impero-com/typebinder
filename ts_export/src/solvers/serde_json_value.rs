use ts_json_subset::types::{PredefinedType, PrimaryType, TsType};

use crate::{
    error::TsExportError,
    exporter_context::ExporterContext,
    type_solver::{SolverResult, TypeInfo, TypeSolver, TypeSolverExt},
};

use super::{fn_solver::AsFnSolver, path::PathSolver};

pub struct SerdeJsonValueSolver {
    path_solver: PathSolver,
}

fn solve_serde_json_value(
    _exporter_context: &ExporterContext,
    _type_info: &TypeInfo,
) -> SolverResult<TsType, TsExportError> {
    SolverResult::Solved(
        TsType::PrimaryType(PrimaryType::Predefined(PredefinedType::Any)),
        Vec::new(),
    )
}

impl Default for SerdeJsonValueSolver {
    fn default() -> Self {
        let mut path_solver = PathSolver::default();
        path_solver.add_entry(
            "serde_json::Value",
            solve_serde_json_value.as_fn_solver().as_rc(),
        );
        SerdeJsonValueSolver { path_solver }
    }
}

impl TypeSolver for SerdeJsonValueSolver {
    fn solve_as_type(
        &self,
        context: &ExporterContext,
        type_info: &TypeInfo,
    ) -> SolverResult<TsType, TsExportError> {
        self.path_solver.solve_as_type(context, type_info)
    }
}
