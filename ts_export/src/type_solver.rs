use serde_derive_internals::ast::Field;
use syn::Generics;
use ts_json_subset::types::{TsType, TypeMember};

pub struct SolverInfo<'a> {
    pub generics: &'a Generics,
    pub field: Field<'a>,
}

pub trait TypeSolver {
    fn solve_newtype(
        &self,
        _solving_context: &TypeSolvingContext,
        _solver_info: &SolverInfo,
    ) -> Option<TsType> {
        None
    }

    fn solve_tuple(
        &self,
        _solving_context: &TypeSolvingContext,
        _solver_info: &SolverInfo,
    ) -> Option<TsType> {
        None
    }

    fn solve_struct_field(
        &self,
        _solving_context: &TypeSolvingContext,
        _solver_info: &SolverInfo,
    ) -> Option<TypeMember> {
        None
    }
}

#[derive(Default)]
pub struct TypeSolvingContext {
    solvers: Vec<Box<dyn TypeSolver>>,
}

impl TypeSolvingContext {
    pub fn add_solver<S: TypeSolver + 'static>(&mut self, solver: S) {
        self.solvers.push(Box::new(solver));
    }

    pub fn solve_type(&self, solver_info: &SolverInfo) -> Option<TsType> {
        self.solvers
            .iter()
            .filter_map(|solver| solver.as_ref().solve_newtype(&self, solver_info))
            .next()
    }

    pub fn solve_tuple(&self, solver_info: &SolverInfo) -> Option<TsType> {
        self.solvers
            .iter()
            .filter_map(|solver| solver.as_ref().solve_tuple(&self, solver_info))
            .next()
    }

    pub fn solve_struct_field(&self, solver_info: &SolverInfo) -> Option<TypeMember> {
        self.solvers
            .iter()
            .filter_map(|solver| solver.as_ref().solve_struct_field(&self, solver_info))
            .next()
    }
}
