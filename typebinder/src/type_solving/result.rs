use ts_json_subset::types::{TsType, TypeMember};

use crate::error::TsExportError;

use super::ImportEntry;

/// The result of a TypeSolver. See the explanation there.
pub enum SolverResult<T, E> {
    /// The solver could not process the given type info
    Continue,
    /// The solver correctly processed the input type
    Solved(T, Vec<ImportEntry>),
    /// The solver tried to process the input type, but it failed to do so
    Error(E),
}

impl From<Result<(TsType, Vec<ImportEntry>), TsExportError>>
    for SolverResult<TsType, TsExportError>
{
    fn from(result: Result<(TsType, Vec<ImportEntry>), TsExportError>) -> Self {
        match result {
            Ok((ty, imports)) => SolverResult::Solved(ty, imports),
            Err(e) => SolverResult::Error(e),
        }
    }
}

impl From<Result<(TypeMember, Vec<ImportEntry>), TsExportError>>
    for SolverResult<TypeMember, TsExportError>
{
    fn from(result: Result<(TypeMember, Vec<ImportEntry>), TsExportError>) -> Self {
        match result {
            Ok((ty, imports)) => SolverResult::Solved(ty, imports),
            Err(e) => SolverResult::Error(e),
        }
    }
}
