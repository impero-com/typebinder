use crate::common::StringLiteral;

/*
ConstEnumDeclaration

ConstEnumDeclaration
    const enum Identifier ConstEnumBody
ConstEnumBody
    { ConstEnumVariantList }
ConstEnumVariantList
    Identifier = StringLiteralValue ,opt
    Identifier = StringLiteralValue , ConstEnumVariantList
*/

#[derive(Debug, Clone, PartialEq)]
pub struct ConstEnumDeclaration {
    pub ident: String,
    pub body: ConstEnumBody,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstEnumBody {
    pub variants: ConstEnumVariant,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstEnumVariant {
    pub identifier: String,
    pub value: StringLiteral,
}
