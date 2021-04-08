use crate::declarations::{
    const_enum::ConstEnumDeclaration, interface::InterfaceDeclaration,
    reexport::ReexportDeclaration, type_alias::TypeAliasDeclaration,
};
use displaythis::Display;
use from_variants::FromVariants;

#[derive(Debug, Clone, PartialEq, FromVariants, Display)]
/// An export statement, with support for exporting interfaces, types, const enum, and reexports
pub enum ExportStatement {
    #[display("export {0}")]
    InterfaceDeclaration(InterfaceDeclaration),
    #[display("export {0}")]
    TypeAliasDeclaration(TypeAliasDeclaration),
    #[display("export {0}")]
    ConstEnumDeclaration(ConstEnumDeclaration),
    #[display("export {0}")]
    ReexportDeclaration(ReexportDeclaration),
}
