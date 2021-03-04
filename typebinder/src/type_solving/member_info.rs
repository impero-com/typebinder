use serde_derive_internals::ast::Field;
use syn::{Generics, Type};

use super::type_info::TypeInfo;

/// Stores the information of a member of a struct
/// The stored generics are the generics of the parent structure.
pub struct MemberInfo<'a> {
    pub generics: &'a Generics,
    pub ty: &'a Type,
    pub name: String,
    pub field: &'a syn::Field,
    pub serde_field: &'a serde_derive_internals::attr::Field,
}

impl<'a> MemberInfo<'a> {
    pub fn from_generics_and_field(generics: &'a Generics, field: &'a Field<'a>) -> Self {
        let name = field.attrs.name().serialize_name();
        Self {
            generics,
            field: field.original,
            ty: field.ty,
            name,
            serde_field: &field.attrs,
        }
    }

    pub fn as_type_info(&self) -> TypeInfo<'a> {
        let MemberInfo { generics, ty, .. } = self;
        TypeInfo { generics, ty }
    }
}
