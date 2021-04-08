use askama::Template;

use crate::ident::TSIdent;

#[derive(Debug, Clone, PartialEq, Eq, Template)]
#[template(source = r#"{ {{ reexports|join(", ") }} }"#, ext = "txt")]
pub struct ReexportDeclaration {
    pub reexports: Vec<ReexportClause>,
}

#[derive(Debug, Clone, PartialEq, Eq, Template)]
#[template(source = r#"{{ scope }} as {{ export_as }}"#, ext = "txt")]
pub struct ReexportClause {
    pub scope: TSIdent,
    pub export_as: TSIdent,
}

#[cfg(test)]
pub mod tests {
    use std::str::FromStr;

    use super::*;

    pub fn should_reexport() {
        assert_eq!(
            ReexportClause {
                scope: TSIdent::from_str("ThisType").unwrap(),
                export_as: TSIdent::from_str("ThatType").unwrap(),
            }
            .to_string(),
            "ThisType as ThatType",
        );
    }
}
