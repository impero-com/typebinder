use crate::types::{ObjectType, TypeParameters, TypeReference};

/*
Interface

InterfaceDeclaration
    interface Identifier TypeParametersopt InterfaceExtendsClauseopt ObjectType
InterfaceExtendsClause
    extends InterfaceTypeList
InterfaceTypeList
    Identifier
    Identifier, InterfaceTypeList
*/

#[derive(Debug, Clone, PartialEq)]
pub struct InterfaceTypeList {
    pub identifiers: Vec<TypeReference>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InterfaceExtendsClause {
    pub type_list: InterfaceTypeList,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InterfaceDeclaration {
    pub ident: String,
    pub type_params: Option<TypeParameters>,
    pub extends_clause: Option<InterfaceExtendsClause>,
    pub obj_type: ObjectType,
}
