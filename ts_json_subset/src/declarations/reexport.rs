use askama::Template;

use crate::ident::StrictTSIdent;

#[derive(Debug, Clone, PartialEq, Eq, Template)]
#[template(source = r#"{ {{ reexports|join(", ") }} }"#, ext = "txt")]
pub struct ReexportDeclaration {
    pub reexports: Vec<ReexportClause>,
}

#[derive(Debug, Clone, PartialEq, Eq, Template)]
#[template(source = r#"{{ scope }} as {{ export_as }}"#, ext = "txt")]
pub struct ReexportClause {
    pub scope: StrictTSIdent,
    pub export_as: StrictTSIdent,
}

#[cfg(test)]
pub mod tests {
    use std::str::FromStr;

    use super::*;

    pub fn should_reexport() {
        assert_eq!(
            ReexportClause {
                scope: StrictTSIdent::from_str("ThisType").unwrap(),
                export_as: StrictTSIdent::from_str("ThatType").unwrap(),
            }
            .to_string(),
            "ThisType as ThatType",
        );
    }
}
