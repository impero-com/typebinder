use askama::Template;
use displaythis::Display;

#[derive(Debug, Clone, PartialEq, Template)]
#[template(source = "import {{ import_kind }} from {{ path }};", ext = "txt")]
pub struct ImportStatement {
    pub import_kind: ImportKind,
    // TODO: Might need stronger typing here
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Template)]
#[template(source = "{{ items|join(\", \") }}", ext = "txt")]
pub struct ImportList {
    pub items: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum ImportKind {
    #[display("{0}")]
    Identifier(String),
    #[display("* as {0}")]
    GlobAsIdentifier(String),
    #[display("{{ {0} }}")]
    ImportList(ImportList),
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn display_import_list() {
        assert_eq!(
            ImportList {
                items: vec!["Test".to_string(), "TestOther".to_string(),],
            }
            .to_string(),
            "Test, TestOther",
        );
    }

    #[test]
    fn display_import_statement() {
        assert_eq!(
            ImportStatement {
                import_kind: ImportKind::Identifier("Test".to_string()),
                path: "\"types/users\"".to_string(),
            }
            .to_string(),
            "import Test from \"types/users\";"
        );

        assert_eq!(
            ImportStatement {
                import_kind: ImportKind::GlobAsIdentifier("Test".to_string()),
                path: "\"types/users\"".to_string(),
            }
            .to_string(),
            "import * as Test from \"types/users\";"
        );

        assert_eq!(
            ImportStatement {
                import_kind: ImportKind::ImportList(ImportList {
                    items: vec!["Test".to_string(), "TestOther".to_string()]
                }),
                path: "\"types/users\"".to_string(),
            }
            .to_string(),
            "import { Test, TestOther } from \"types/users\";"
        );
    }
}
