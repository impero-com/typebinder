use serde_derive_internals::ast::Field;
use syn::{Generics, Type};

use super::type_info::TypeInfo;

pub struct MemberInfo<'a> {
    pub generics: &'a Generics,
    pub ty: &'a Type,
    pub name: String,
    pub field: &'a syn::Field,
}

impl<'a> MemberInfo<'a> {
    pub fn from_generics_and_field(generics: &'a Generics, field: Field<'a>) -> Self {
        Self {
            generics,
            field: field.original,
            ty: field.ty,
            name: field.attrs.name().serialize_name(),
        }
    }

    pub fn as_type_info(&self) -> TypeInfo<'a> {
        let MemberInfo { generics, ty, .. } = self;
        TypeInfo { generics, ty }
    }
}
