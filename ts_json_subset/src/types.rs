use crate::common::{filters, BooleanLiteral, NumericLiteral, StringLiteral};
use askama::Template;
use displaythis::Display;
use from_variants::FromVariants;

#[derive(Debug, Clone, PartialEq, Template)]
#[template(source = "{{ inner_type }}[]", ext = "txt")]
/// A generic TS array
pub struct ArrayType {
    pub inner_type: Box<PrimaryType>,
}

impl ArrayType {
    pub fn new(primary: PrimaryType) -> Self {
        ArrayType {
            inner_type: Box::new(primary),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Template)]
#[template(source = "<{{ identifiers|join(\", \") }}>", ext = "txt")]
/// A identifier list of generic parameters
pub struct TypeParameters {
    // TODO: Make an identifier type that checks TS constraints on identifiers
    pub identifiers: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Template)]
#[template(source = "[ {{ inner_types|join(\", \") }} ]", ext = "txt")]
/// A tuple represented as an array with positional types
pub struct TupleType {
    pub inner_types: Vec<TsType>,
}

#[derive(Debug, Clone, PartialEq, Display, FromVariants)]
/// A literal type, supports strings, numbers and booleans
pub enum LiteralType {
    #[display("{0}")]
    StringLiteral(StringLiteral),
    #[display("{0}")]
    NumericLiteral(NumericLiteral),
    #[display("{0}")]
    BooleanLiteral(BooleanLiteral),
}

#[derive(Debug, Clone, PartialEq, Template)]
#[template(
    source = "{% match namespace %}{% when Some with (namespace) %}{{ namespace }}.{% when None %}{% endmatch %}{{- ident -}}",
    ext = "txt"
)]
/// And optionally namespaced type identifier
pub struct TypeName {
    // TODO: Make an identifier type that checks TS constraints on identifiers
    pub ident: String,
    // TODO: Check if we want to keep it, it seems unused
    pub namespace: Option<Box<TypeName>>,
}

#[derive(Debug, Clone, PartialEq, Template)]
// TODO: remove space between chevron and types ?
#[template(source = "< {{ types|join(\", \") }} >", ext = "txt")]
/// A list of type arguments use in a generic parameter
pub struct TypeArguments {
    pub types: Vec<TsType>,
}

#[derive(Debug, Clone, PartialEq, Display, FromVariants)]
/// A TS combination of TS types, supports unions, intersections and parenthesis
pub enum TsType {
    #[display("{0}")]
    PrimaryType(PrimaryType),
    #[display("{0}")]
    // TODO: Inline UnionType ?
    UnionType(UnionType),
    #[display("{0}")]
    // TODO: Inline IntersectionType ?
    IntersectionType(IntersectionType),
    #[display("{0}")]
    // TODO: Inline ParenthesizedType ?
    ParenthesizedType(ParenthesizedType),
}

#[derive(Debug, Clone, PartialEq, Template)]
#[template(source = "{{ name }}{{ args|display_opt }}", ext = "txt")]
/// A type identifier with support for generic parameters
pub struct TypeReference {
    pub name: TypeName,
    pub args: Option<TypeArguments>,
}

#[derive(Debug, Clone, PartialEq, Template)]
#[template(source = "{{ types|join(\" | \") }}", ext = "txt")]
/// An union of multiple TS types
pub struct UnionType {
    pub types: Vec<TsType>,
}

#[derive(Debug, Clone, PartialEq, Template)]
#[template(source = "{{ types|join(\" & \") }}", ext = "txt")]
/// An intersection of multiple TS types
pub struct IntersectionType {
    pub types: Vec<TsType>,
}

#[derive(Debug, Clone, PartialEq, Template)]
#[template(source = "( {{ inner }} )", ext = "txt")]
/// A TS type surrounded by parenthesis
pub struct ParenthesizedType {
    pub inner: Box<TsType>,
}

#[derive(Debug, Clone, PartialEq, Template)]
#[template(source = "{\n\t{{ body|display_opt }}\n}", ext = "txt")]
/// A TS object type
pub struct ObjectType {
    // TODO: Remove the option and inline TypeBody ? None seems unused
    pub body: Option<TypeBody>,
}

#[derive(Debug, Clone, PartialEq, Template)]
#[template(source = "{{ members|join(\",\\n\t\") }}", ext = "txt")]
pub struct TypeBody {
    pub members: Vec<TypeMember>,
}

#[derive(Debug, Clone, PartialEq, Display, FromVariants)]
// TODO: Remove the enum and use PropertySignature directly ?
pub enum TypeMember {
    #[display("{0}")]
    PropertySignature(PropertySignature),
}

#[derive(Debug, Clone, PartialEq, Template)]
#[template(
    source = "{{ name }}{% if optional %}?{% endif %}: {{ inner_type }}",
    ext = "txt"
)]
/// An object property definition
pub struct PropertySignature {
    pub name: PropertyName,
    pub optional: bool,
    pub inner_type: TsType,
}

#[derive(Debug, Clone, PartialEq, Display, FromVariants)]
/// An object property identifier
// TODO: Impl a from str that check if this is a legal bare TS identifier -> PropertyName::Identifier else use the quoted version using PropertyName::StringLiteral
pub enum PropertyName {
    #[display("{0}")]
    // TODO: Make an identifier type that checks TS constraints on identifiers
    Identifier(String),
    #[display("{0}")]
    StringLiteral(StringLiteral),
}

#[derive(Debug, Clone, PartialEq, Display, FromVariants)]
/// A single TS type
pub enum PrimaryType {
    #[display("{0}")]
    Predefined(PredefinedType),
    #[display("{0}")]
    TypeReference(TypeReference),
    #[display("{0}")]
    ObjectType(ObjectType),
    #[display("{0}")]
    ArrayType(ArrayType),
    #[display("{0}")]
    TupleType(TupleType),
    #[display("{0}")]
    LiteralType(LiteralType),
}

#[derive(Debug, Clone, PartialEq, Display)]
/// A globally defined TS type
pub enum PredefinedType {
    #[display("any")]
    Any,
    #[display("number")]
    Number,
    #[display("boolean")]
    Boolean,
    #[display("string")]
    String,
    #[display("unknown")]
    Unknown,
    #[display("null")]
    Null,
    #[display("never")]
    Never,
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn display_primary_type() {
        assert_eq!(
            PrimaryType::Predefined(PredefinedType::Any).to_string(),
            "any"
        );

        assert_eq!(
            PrimaryType::TypeReference(TypeReference {
                args: None,
                name: TypeName {
                    ident: "MyType".to_string(),
                    namespace: None,
                }
            })
            .to_string(),
            "MyType"
        );
    }

    #[test]
    fn display_property_signature() {
        assert_eq!(
            PropertySignature {
                name: PropertyName::Identifier("test".to_string()),
                optional: false,
                inner_type: TsType::PrimaryType(PrimaryType::Predefined(PredefinedType::String))
            }
            .to_string(),
            "test: string"
        );

        assert_eq!(
            PropertySignature {
                name: PropertyName::Identifier("test".to_string()),
                optional: true,
                inner_type: TsType::PrimaryType(PrimaryType::Predefined(PredefinedType::Number))
            }
            .to_string(),
            "test?: number"
        );

        assert_eq!(
            PropertySignature {
                name: PropertyName::StringLiteral("test".into()),
                optional: true,
                inner_type: TsType::PrimaryType(PrimaryType::Predefined(PredefinedType::Number))
            }
            .to_string(),
            r#""test"?: number"#
        );
    }

    #[test]
    fn display_type_body() {
        assert_eq!(
            TypeBody {
                members: vec![
                    TypeMember::PropertySignature(PropertySignature {
                        name: PropertyName::Identifier("test".into()),
                        optional: false,
                        inner_type: TsType::PrimaryType(PrimaryType::Predefined(
                            PredefinedType::Number
                        ))
                    }),
                    TypeMember::PropertySignature(PropertySignature {
                        name: PropertyName::StringLiteral("test_other".into()),
                        optional: false,
                        inner_type: TsType::PrimaryType(PrimaryType::Predefined(
                            PredefinedType::Any
                        ))
                    }),
                ]
            }
            .to_string(),
            "test: number,\n\t\"test_other\": any",
        );
    }

    #[test]
    fn display_tuple_types() {
        assert_eq!(
            TupleType {
                inner_types: vec![
                    TsType::PrimaryType(PrimaryType::Predefined(PredefinedType::Any)),
                    TsType::PrimaryType(PrimaryType::Predefined(PredefinedType::Boolean)),
                ],
            }
            .to_string(),
            "[ any, boolean ]"
        );
    }

    #[test]
    fn display_array_type() {
        assert_eq!(
            ArrayType::new(PrimaryType::Predefined(PredefinedType::Any)).to_string(),
            "any[]"
        );
    }

    #[test]
    fn display_object_type() {
        assert_eq!(ObjectType { body: None }.to_string(), "{\n\t\n}",);
    }
}
