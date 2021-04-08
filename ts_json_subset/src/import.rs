use askama::Template;
use displaythis::Display;

use crate::ident::TSIdent;

#[derive(Debug, Clone, PartialEq, Template)]
#[template(source = "import {{ import_kind }} from {{ path }};", ext = "txt")]
/// An import statement, supporting multiple imports from a file
pub struct ImportStatement {
    pub import_kind: ImportKind,
    // TODO: Might need stronger typing here
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Template)]
#[template(source = "{{ items|join(\", \") }}", ext = "txt")]
/// An list of imported identifiers
pub struct ImportList {
    pub items: Vec<TSIdent>,
}

#[derive(Debug, Clone, PartialEq, Display)]
/// The identifiers fragment of an import statement
pub enum ImportKind {
    #[display("{0}")]
    Identifier(TSIdent),
    #[display("* as {0}")]
    GlobAsIdentifier(TSIdent),
    #[display("{{ {0} }}")]
    ImportList(ImportList),
}

#[cfg(test)]
pub mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn display_import_list() {
        assert_eq!(
            ImportList {
                items: vec![
                    TSIdent::from_str("Test").unwrap(),
                    TSIdent::from_str("TestOther").unwrap()
                ],
            }
            .to_string(),
            "Test, TestOther",
        );
    }

    #[test]
    fn display_import_statement() {
        assert_eq!(
            ImportStatement {
                import_kind: ImportKind::Identifier(TSIdent::from_str("Test").unwrap()),
                path: r#""types/users""#.to_string(),
            }
            .to_string(),
            r#"import Test from "types/users";"#
        );

        assert_eq!(
            ImportStatement {
                import_kind: ImportKind::GlobAsIdentifier(TSIdent::from_str("Test").unwrap()),
                path: r#""types/users""#.to_string(),
            }
            .to_string(),
            r#"import * as Test from "types/users";"#
        );

        assert_eq!(
            ImportStatement {
                import_kind: ImportKind::ImportList(ImportList {
                    items: vec![
                        TSIdent::from_str("Test").unwrap(),
                        TSIdent::from_str("TestOther").unwrap()
                    ]
                }),
                path: r#""types/users""#.to_string(),
            }
            .to_string(),
            r#"import { Test, TestOther } from "types/users";"#
        );
    }
}
