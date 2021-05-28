use ts_json_subset::types::{PredefinedType, PrimaryType, TsType};

use crate::{
    contexts::exporter::ExporterContext,
    error::TsExportError,
    type_solving::{fn_solver::AsFnSolver, result::Solved},
    type_solving::{SolverResult, TypeInfo, TypeSolver, TypeSolverExt},
};

use super::path::PathSolver;

/// Support for serde_json::Value.
/// It will deserialize to the any type.
pub struct SerdeJsonValueSolver {
    path_solver: PathSolver,
}

fn solve_serde_json_value(
    _exporter_context: &ExporterContext,
    _type_info: &TypeInfo,
) -> SolverResult<TsType, TsExportError> {
    SolverResult::Solved(Solved::new(TsType::PrimaryType(PrimaryType::Predefined(
        PredefinedType::Any,
    ))))
}

impl Default for SerdeJsonValueSolver {
    fn default() -> Self {
        let mut path_solver = PathSolver::default();
        path_solver.add_entry(
            "serde_json::Value",
            solve_serde_json_value.fn_solver().into_rc(),
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
