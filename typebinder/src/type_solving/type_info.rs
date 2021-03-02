use syn::{Generics, Type};

/// The information required to solve a type
#[derive(Debug)]
pub struct TypeInfo<'a> {
    /// Generics of the parent type
    pub generics: &'a Generics,
    /// Type to solve
    pub ty: &'a Type,
}
