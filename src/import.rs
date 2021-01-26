/*
Import

ImportStatement
    import ImportKind from Path;
ImportKind
    Identifier
    * as Identifier
    { ImportList }
ImportList
    ImportListItem
    ImportListItem, ImportList
ImportListItem
    Identifier
    Identifier as Identifier
*/

#[derive(Debug, Clone, PartialEq)]
pub struct ImportStatement {
    pub import_kind: ImportKind,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImportList {
    pub items: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImportKind {
    Identifier(String),
    GlobAsIdentifier(String),
    ImportList(ImportList),
}
