use crate::type_solving::{TypeSolver, TypeSolverExt};

pub struct TypeSolvingContext {
    solvers: Vec<Box<dyn TypeSolver>>,
}

impl TypeSolvingContext {
    pub fn solvers(&self) -> &Vec<Box<dyn TypeSolver>> {
        &self.solvers
    }
}

use crate::type_solving::solvers::{
    array::ArraySolver, chrono::ChronoSolver, collections::CollectionsSolver,
    generics::GenericsSolver, import::ImportSolver, option::OptionSolver,
    primitives::PrimitivesSolver, reference::ReferenceSolver,
    serde_json_value::SerdeJsonValueSolver, tuple::TupleSolver,
};

#[derive(Default)]
pub struct TypeSolvingContextBuilder {
    solvers: Vec<Box<dyn TypeSolver>>,
}

impl TypeSolvingContextBuilder {
    pub fn add_solver<S: TypeSolver + 'static>(mut self, solver: S) -> Self {
        self.solvers.push(solver.boxed());
        self
    }

    pub fn add_default_solvers(self) -> Self {
        self.add_solver(TupleSolver)
            .add_solver(ReferenceSolver)
            .add_solver(ArraySolver)
            .add_solver(CollectionsSolver::default())
            .add_solver(PrimitivesSolver::default())
            .add_solver(OptionSolver::default())
            .add_solver(GenericsSolver)
            .add_solver(ChronoSolver::default())
            .add_solver(SerdeJsonValueSolver::default())
    }

    pub fn finish(self) -> TypeSolvingContext {
        let builder = self.add_solver(ImportSolver);
        TypeSolvingContext {
            solvers: builder.solvers,
        }
    }
}
