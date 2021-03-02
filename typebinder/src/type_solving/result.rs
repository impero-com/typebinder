use ts_json_subset::types::{TsType, TypeMember};

use crate::error::TsExportError;

use super::ImportEntry;

/// The result of a TypeSolver
pub enum SolverResult<T, E> {
    /// The solver could not process the given type info
    Continue,
    /// The solver correctly processed the input type
    Solved(T, Vec<ImportEntry>),
    /// The solver tried to process the input type, but it failed to do so
    Error(E),
}

impl From<Result<TsType, TsExportError>> for SolverResult<TsType, TsExportError> {
    fn from(result: Result<TsType, TsExportError>) -> Self {
        match result {
            Ok(ty) => SolverResult::Solved(ty, Vec::new()),
            Err(e) => SolverResult::Error(e),
        }
    }
}

impl From<Result<TypeMember, TsExportError>> for SolverResult<TypeMember, TsExportError> {
    fn from(result: Result<TypeMember, TsExportError>) -> Self {
        match result {
            Ok(ty) => SolverResult::Solved(ty, Vec::new()),
            Err(e) => SolverResult::Error(e),
        }
    }
}
