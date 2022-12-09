use crate::{common::StringLiteral, ident::StrictTSIdent};
use askama::Template;

#[derive(Debug, Clone, Eq, PartialEq, Template)]
#[template(source = "const enum {{ ident }} {{ body }}", ext = "txt")]
/// A const enum with string literals (TS numeric const enum offer no advantage, consider using union types instead)
pub struct ConstEnumDeclaration {
    pub ident: StrictTSIdent,
    pub body: ConstEnumBody,
}

#[derive(Debug, Clone, Eq, PartialEq, Template)]
#[template(source = r#"{ {{ variants|join(", ") }} }"#, ext = "txt")]
/// A description of all variants in a const enum with string literals, see `ConstEnumDeclaration`
pub struct ConstEnumBody {
    pub variants: Vec<ConstEnumVariant>,
}

#[derive(Debug, Clone, Eq, PartialEq, Template)]
#[template(source = "{{ ident }} = {{ value }}", ext = "txt")]
/// A const enum variant with string literal
pub struct ConstEnumVariant {
    pub ident: StrictTSIdent,
    pub value: StringLiteral,
}

#[cfg(test)]
pub mod tests {
    use std::str::FromStr;

    use super::*;

    fn build_dummy_enum_body() -> ConstEnumBody {
        ConstEnumBody {
            variants: vec![
                ConstEnumVariant {
                    ident: StrictTSIdent::from_str("One").unwrap(),
                    value: StringLiteral::from_raw("one"),
                },
                ConstEnumVariant {
                    ident: StrictTSIdent::from_str("Two").unwrap(),
                    value: StringLiteral::from_raw("two"),
                },
            ],
        }
    }

    #[test]
    fn display_const_enum_declaration() {
        assert_eq!(
            ConstEnumDeclaration {
                ident: StrictTSIdent::from_str("MyEnum").unwrap(),
                body: build_dummy_enum_body()
            }
            .to_string(),
            r#"const enum MyEnum { One = "one", Two = "two" }"#,
        );
    }

    #[test]
    fn display_const_enum_body() {
        assert_eq!(
            build_dummy_enum_body().to_string(),
            r#"{ One = "one", Two = "two" }"#
        )
    }

    #[test]
    fn display_const_enum_variant() {
        assert_eq!(
            ConstEnumVariant {
                ident: StrictTSIdent::from_str("MyVariant").unwrap(),
                value: StringLiteral::from_raw("TheValue"),
            }
            .to_string(),
            r#"MyVariant = "TheValue""#,
        );
    }
}
