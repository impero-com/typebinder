use askama::Template;
use displaythis::Display;

use crate::common::{BooleanLiteral, NumericLiteral, StringLiteral};

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayType {
    pub inner_type: Box<PrimaryType>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeParameters {
    pub identifiers: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TupleType {
    pub inner_types: Vec<TsType>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralType {
    StringLiteral(StringLiteral),
    NumericLiteral(NumericLiteral),
    BooleanLiteral(BooleanLiteral),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeName {
    pub ident: String,
    pub namespace: Option<Box<TypeName>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeArguments {
    pub types: Vec<TsType>,
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum TsType {
    #[display("Primary")]
    PrimaryType(PrimaryType),
    #[display("Union")]
    UnionType(UnionType),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeReference {
    pub name: TypeName,
    pub args: Option<TypeArguments>,
}

#[derive(Debug, Clone, PartialEq, Template)]
#[template(source = "{{ types|join(\" | \") }}", ext = "txt")]
pub struct UnionType {
    pub types: Vec<TsType>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectType {
    pub body: Option<TypeBody>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeBody {
    pub members: Vec<TypeMember>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeMember {
    PropertySignature(PropertySignature),
}

#[derive(Debug, Clone, PartialEq)]
pub struct PropertySignature {
    pub name: PropertyName,
    pub optional: bool,
    pub inner_type: TsType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PropertyName {
    Identifier(String),
    StringLiteral(StringLiteral),
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrimaryType {
    Predefined(PredefinedType),
    TypeReference(TypeReference),
    ObjectType(ObjectType),
    ArrayType(ArrayType),
    TupleType(TupleType),
    LiteralType(LiteralType),
}

#[derive(Debug, Clone, PartialEq, Display)]
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
