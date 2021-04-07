use crate::{common::StringLiteral, ident::TSIdent};
use askama::Template;

#[derive(Debug, Clone, PartialEq, Template)]
#[template(source = "const enum {{ ident }} {{ body }}", ext = "txt")]
/// A const enum with string literals (TS numeric const enum are useless, use union types instead)
pub struct ConstEnumDeclaration {
    pub ident: TSIdent,
    // TODO: inline body ?
    pub body: ConstEnumBody,
}

#[derive(Debug, Clone, PartialEq, Template)]
#[template(source = r#"{ {{ variants|join(", ") }} }"#, ext = "txt")]
/// A description of all variants in a const enum with string literals, see `ConstEnumDeclaration`
pub struct ConstEnumBody {
    pub variants: Vec<ConstEnumVariant>,
}

#[derive(Debug, Clone, PartialEq, Template)]
#[template(source = "{{ ident }} = {{ value }}", ext = "txt")]
/// A const enum variant with string literal
pub struct ConstEnumVariant {
    // TODO: Make an identifier type that checks TS constraints on identifiers
    pub ident: TSIdent,
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
                    ident: TSIdent::from_str("One").unwrap(),
                    value: StringLiteral::from("one"),
                },
                ConstEnumVariant {
                    ident: TSIdent::from_str("Two").unwrap(),
                    value: StringLiteral::from("two"),
                },
            ],
        }
    }

    #[test]
    fn display_const_enum_declaration() {
        assert_eq!(
            ConstEnumDeclaration {
                ident: TSIdent::from_str("MyEnum").unwrap(),
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
                ident: TSIdent::from_str("MyVariant").unwrap(),
                value: StringLiteral::from("TheValue"),
            }
            .to_string(),
            r#"MyVariant = "TheValue""#,
        );
    }
}
