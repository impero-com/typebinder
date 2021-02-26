use std::collections::HashMap;

use crate::{
    error::TsExportError,
    exporter_context::ExporterContext,
    type_solver::{SolverResult, TypeInfo, TypeSolver, TypeSolverExt},
};
use ts_json_subset::types::{PredefinedType, PrimaryType, TsType};

use super::{fn_solver::AsFnSolver, path::PathSolver};

pub struct PrimitivesSolver {
    inner: PathSolver,
}

fn solve_number(
    _exporter: &ExporterContext,
    _solver_info: &TypeInfo,
) -> SolverResult<TsType, TsExportError> {
    SolverResult::Solved(
        PrimaryType::Predefined(PredefinedType::Number).into(),
        Vec::new(),
    )
}

impl Default for PrimitivesSolver {
    fn default() -> Self {
        let solver_number = solve_number.fn_solver().into_rc();

        let solver_string = (|_: &ExporterContext, _: &TypeInfo| {
            SolverResult::Solved(
                PrimaryType::Predefined(PredefinedType::String).into(),
                Vec::new(),
            )
        })
        .fn_solver()
        .into_rc();

        let solver_bool = (|_: &ExporterContext, _: &TypeInfo| {
            SolverResult::Solved(
                PrimaryType::Predefined(PredefinedType::Boolean).into(),
                Vec::new(),
            )
        })
        .fn_solver()
        .into_rc();

        let mut inner = PathSolver {
            entries: HashMap::default(),
        };

        inner.add_entry("u8", solver_number.clone());
        inner.add_entry("u16", solver_number.clone());
        inner.add_entry("u32", solver_number.clone());
        inner.add_entry("u64", solver_number.clone());
        inner.add_entry("usize", solver_number.clone());
        inner.add_entry("i8", solver_number.clone());
        inner.add_entry("i16", solver_number.clone());
        inner.add_entry("i32", solver_number.clone());
        inner.add_entry("i64", solver_number.clone());
        inner.add_entry("isize", solver_number.clone());
        inner.add_entry("f32", solver_number.clone());
        inner.add_entry("f64", solver_number);

        inner.add_entry("char", solver_string.clone());
        inner.add_entry("str", solver_string.clone());
        inner.add_entry("std::string::String", solver_string.clone());
        inner.add_entry("std::borrow::Cow", solver_string);

        inner.add_entry("bool", solver_bool);

        PrimitivesSolver { inner }
    }
}

impl TypeSolver for PrimitivesSolver {
    fn solve_as_type(
        &self,
        solving_context: &ExporterContext,
        solver_info: &TypeInfo,
    ) -> SolverResult<TsType, TsExportError> {
        self.inner.solve_as_type(solving_context, solver_info)
    }
}
