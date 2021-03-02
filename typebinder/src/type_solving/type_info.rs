use syn::{Generics, Type};

#[derive(Debug)]
pub struct TypeInfo<'a> {
    pub generics: &'a Generics,
    pub ty: &'a Type,
}
