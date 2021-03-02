use std::{collections::HashMap, rc::Rc};

use syn::Type;
use ts_json_subset::types::{TsType, TypeMember};

use crate::{
    contexts::exporter::ExporterContext,
    error::TsExportError,
    type_solving::member_info::MemberInfo,
    type_solving::{SolverResult, TypeInfo, TypeSolver},
    utils::display_path::DisplayPath,
};

/// An abstraction/util solver that takes in a fully-defined path (e.g. std::collections::Vec).
/// It is used when a solver needs to match a certain type's name
#[derive(Default)]
pub struct PathSolver {
    pub entries: HashMap<String, Rc<dyn TypeSolver>>,
}

impl PathSolver {
    pub fn add_entry<S, I>(&mut self, ident: I, solver: Rc<S>)
    where
        S: TypeSolver + 'static,
        I: Into<String>,
    {
        self.entries.insert(ident.into(), solver);
    }
}

impl TypeSolver for PathSolver {
    fn solve_as_type(
        &self,
        solving_context: &ExporterContext,
        solver_info: &TypeInfo,
    ) -> SolverResult<TsType, TsExportError> {
        let TypeInfo { ty, .. } = solver_info;
        match ty {
            Type::Path(ty) => {
                let ident = DisplayPath(&ty.path).to_string();
                match self.entries.get(&ident) {
                    Some(solver) => solver.solve_as_type(solving_context, solver_info),
                    _ => SolverResult::Continue,
                }
            }
            _ => SolverResult::Continue,
        }
    }

    fn solve_as_member(
        &self,
        solving_context: &ExporterContext,
        solver_info: &MemberInfo,
    ) -> SolverResult<TypeMember, TsExportError> {
        match solver_info.ty {
            Type::Path(ty) => {
                let ident = DisplayPath(&ty.path).to_string();
                match self.entries.get(&ident) {
                    Some(solver) => solver.solve_as_member(solving_context, solver_info),
                    _ => SolverResult::Continue,
                }
            }
            _ => SolverResult::Continue,
        }
    }
}
