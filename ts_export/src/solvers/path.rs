use std::collections::HashMap;

use syn::Type;
use ts_json_subset::types::TsType;

use crate::{
    display_path::DisplayPath,
    error::TsExportError,
    type_solver::{SolverResult, TypeInfo, TypeSolver, TypeSolvingContext},
};

pub struct PathSolver {
    pub entries: HashMap<String, Box<dyn Fn(&TypeInfo) -> TsType>>,
}

impl TypeSolver for PathSolver {
    fn solve_as_type(
        &self,
        _solving_context: &TypeSolvingContext,
        solver_info: &TypeInfo,
    ) -> SolverResult<TsType, TsExportError> {
        let TypeInfo { ty, .. } = solver_info;
        match ty {
            Type::Path(ty) => {
                let ident = DisplayPath(&ty.path).to_string();
                match self.entries.get(&ident) {
                    Some(fun) => SolverResult::Solved(fun(solver_info)),
                    _ => SolverResult::Continue,
                }
            }
            _ => SolverResult::Continue,
        }
    }
}
