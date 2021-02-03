use std::collections::HashMap;

use crate::{
    error::TsExportError,
    exporter::ExporterContext,
    type_solver::{SolverResult, TypeInfo, TypeSolver, TypeSolverExt},
};
use ts_json_subset::types::{PredefinedType, PrimaryType, TsType};

use super::{fn_solver::AsFnSolver, path::PathSolver};

pub struct PrimitivesSolver {
    inner: PathSolver,
}

impl Default for PrimitivesSolver {
    fn default() -> Self {
        let solver_number = (|_: &ExporterContext, _: &TypeInfo| {
            SolverResult::Solved(PrimaryType::Predefined(PredefinedType::Number).into())
        })
        .as_fn_solver()
        .as_rc();

        let solver_string = (|_: &ExporterContext, _: &TypeInfo| {
            SolverResult::Solved(PrimaryType::Predefined(PredefinedType::String).into())
        })
        .as_fn_solver()
        .as_rc();

        let solver_bool = (|_: &ExporterContext, _: &TypeInfo| {
            SolverResult::Solved(PrimaryType::Predefined(PredefinedType::Boolean).into())
        })
        .as_fn_solver()
        .as_rc();

        let mut inner = PathSolver {
            entries: HashMap::default(),
        };

        inner.add_entry("u8".to_string(), solver_number.clone());
        inner.add_entry("u16".to_string(), solver_number.clone());
        inner.add_entry("u32".to_string(), solver_number.clone());
        inner.add_entry("u64".to_string(), solver_number.clone());
        inner.add_entry("usize".to_string(), solver_number.clone());
        inner.add_entry("i8".to_string(), solver_number.clone());
        inner.add_entry("i16".to_string(), solver_number.clone());
        inner.add_entry("i32".to_string(), solver_number.clone());
        inner.add_entry("i64".to_string(), solver_number.clone());
        inner.add_entry("isize".to_string(), solver_number.clone());
        inner.add_entry("f32".to_string(), solver_number.clone());
        inner.add_entry("f64".to_string(), solver_number);
        inner.add_entry("char".to_string(), solver_string.clone());
        inner.add_entry("str".to_string(), solver_string.clone());
        inner.add_entry("String".to_string(), solver_string);
        inner.add_entry("bool".to_string(), solver_bool);

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
