use crate::declarations::{
    const_enum::ConstEnumDeclaration, interface::InterfaceDeclaration,
    type_alias::TypeAliasDeclaration,
};

/*
Export

ExportStatement
    export InterfaceDeclaration
    export TypeAliasDeclaration
    export ConstEnumDeclaration
*/

#[derive(Debug, Clone, PartialEq)]
pub enum ExportStatement {
    InterfaceDeclaration(InterfaceDeclaration),
    TypeAliasDeclaration(TypeAliasDeclaration),
    ConstEnumDeclaration(ConstEnumDeclaration),
}
