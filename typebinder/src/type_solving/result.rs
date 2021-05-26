use super::{generic_constraints::GenericConstraints, ImportEntry};

#[derive(Default)]
pub struct Solved<T> {
    pub inner: T,
    pub import_entries: Vec<ImportEntry>,
    pub generic_constraints: GenericConstraints,
}

impl<T> Solved<T> {
    pub fn new(inner: T) -> Self {
        Solved {
            inner,
            import_entries: Vec::new(),
            generic_constraints: GenericConstraints::default(),
        }
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, mapper: F) -> Solved<U> {
        let Solved {
            inner,
            import_entries,
            generic_constraints,
        } = self;
        Solved {
            inner: mapper(inner),
            import_entries,
            generic_constraints,
        }
    }
}

/// The result of a TypeSolver. See the explanation on each variants.
pub enum SolverResult<T, E> {
    /// The solver could not process the given type info
    Continue,
    /// The solver correctly processed the input type
    Solved(Solved<T>),
    /// The solver tried to process the input type, but it failed to do so
    Error(E),
}

/*
impl From<Result<(TsType, Vec<ImportEntry>), TsExportError>>
    for SolverResult<TsType, TsExportError>
{
    fn from(result: Result<(TsType, Vec<ImportEntry>), TsExportError>) -> Self {
        match result {
            Ok((ty, import_entries)) => SolverResult::Solved(Solved { inner: ty, import_entriess),
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
*/
