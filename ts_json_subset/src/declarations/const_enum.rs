use crate::common::StringLiteral;
use askama::Template;

#[derive(Debug, Clone, PartialEq, Template)]
#[template(source = "const enum {{ ident }} {{ body }}", ext = "txt")]
pub struct ConstEnumDeclaration {
    pub ident: String,
    pub body: ConstEnumBody,
}

#[derive(Debug, Clone, PartialEq, Template)]
#[template(source = "{ {{ variants|join(\", \") }} }", ext = "txt")]
pub struct ConstEnumBody {
    pub variants: Vec<ConstEnumVariant>,
}

#[derive(Debug, Clone, PartialEq, Template)]
#[template(source = "{{ ident }} = {{ value }}", ext = "txt")]
pub struct ConstEnumVariant {
    pub ident: String,
    pub value: StringLiteral,
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn display_const_enum_declaration() {
        assert_eq!(
            ConstEnumDeclaration {
                ident: "MyEnum".to_string(),
                body: ConstEnumBody {
                    variants: vec![
                        ConstEnumVariant {
                            ident: "One".to_string(),
                            value: StringLiteral::from("one"),
                        },
                        ConstEnumVariant {
                            ident: "Two".to_string(),
                            value: StringLiteral::from("two"),
                        },
                    ]
                }
            }
            .to_string(),
            "const enum MyEnum { One = \"one\", Two = \"two\" }",
        );
    }

    #[test]
    fn display_const_enum_body() {
        assert_eq!(
            ConstEnumBody {
                variants: vec![
                    ConstEnumVariant {
                        ident: "One".to_string(),
                        value: StringLiteral::from("one"),
                    },
                    ConstEnumVariant {
                        ident: "Two".to_string(),
                        value: StringLiteral::from("two"),
                    },
                ]
            }
            .to_string(),
            "{ One = \"one\", Two = \"two\" }"
        )
    }

    #[test]
    fn display_const_enum_variant() {
        assert_eq!(
            ConstEnumVariant {
                ident: "MyVariant".to_string(),
                value: StringLiteral::from("TheValue"),
            }
            .to_string(),
            "MyVariant = \"TheValue\"",
        );
    }
}