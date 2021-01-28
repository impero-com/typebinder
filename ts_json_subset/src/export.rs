use crate::declarations::{
    const_enum::ConstEnumDeclaration, interface::InterfaceDeclaration,
    type_alias::TypeAliasDeclaration,
};
use displaythis::Display;
use from_variants::FromVariants;

#[derive(Debug, Clone, PartialEq, FromVariants, Display)]
pub enum ExportStatement {
    #[display("export {0}")]
    InterfaceDeclaration(InterfaceDeclaration),
    #[display("export {0}")]
    TypeAliasDeclaration(TypeAliasDeclaration),
    #[display("export {0}")]
    ConstEnumDeclaration(ConstEnumDeclaration),
}
