use super::MacroSolver;

#[derive(Default)]
/// Contains all the MacroSolver implementors
pub struct MacroSolvingContext {
    solvers: Vec<Box<dyn MacroSolver>>,
}

impl MacroSolvingContext {
    pub fn add_solver<MS: MacroSolver + 'static>(mut self, macro_solver: MS) -> Self {
        self.solvers.push(Box::new(macro_solver));
        self
    }

    pub fn solvers(&self) -> &[Box<dyn MacroSolver>] {
        &self.solvers
    }
}
