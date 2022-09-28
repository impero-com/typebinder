use crate::type_solving::{solvers::skip_serialize_if::SkipSerializeIf, TypeSolver, TypeSolverExt};

/// The context that contains all TypeSolver implementors for this pipeline.
///
/// This can only be built from a TypeSolvingContextBuilder, because the feature of "looking for a type in the imports"
/// is implemented as a TypeSolver. This means that the ImportSolver *must* be added to the TypeSolvingContext.
/// Also, for the ImportSolver to work correctly, it needs to be the last in the list of solvers.
///
/// To statically ensure that this is the case, we force the usage of the Builder.
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
    primitives::PrimitivesSolver, ranges::RangesSolver, reference::ReferenceSolver,
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
            .add_solver(RangesSolver::default())
            .add_solver(ChronoSolver::default())
            .add_solver(SerdeJsonValueSolver::default())
            .add_solver(SkipSerializeIf)
    }

    pub fn finish(self) -> TypeSolvingContext {
        let builder = self.add_solver(ImportSolver);
        TypeSolvingContext {
            solvers: builder.solvers,
        }
    }
}
