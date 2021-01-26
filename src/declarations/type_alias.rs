use crate::types::{TsType, TypeParameters};

/*
TypeAlias

TypeAliasDeclaration
    type Identifier TypeParametersopt = Type ;
*/

#[derive(Debug, Clone, PartialEq)]
pub struct TypeAliasDeclaration {
    pub ident: String,
    pub params: Option<TypeParameters>,
    pub inner_type: TsType,
}
