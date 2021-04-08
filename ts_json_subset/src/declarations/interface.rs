use crate::types::{ObjectType, TypeParameters, TypeReference};
use crate::{common::filters, ident::TSIdent};
use askama::Template;

#[derive(Debug, Clone, PartialEq, Template)]
#[template(source = r#"{{ identifiers|join(", ") }}"#, ext = "txt")]
pub struct InterfaceTypeList {
    pub identifiers: Vec<TypeReference>,
}

#[derive(Debug, Clone, PartialEq, Template)]
#[template(source = "extends {{ type_list }}", ext = "txt")]
/// An interface extend identifier list
pub struct InterfaceExtendsClause {
    // TODO: Inline InterfaceTypeList ?
    pub type_list: InterfaceTypeList,
}

#[derive(Debug, Clone, PartialEq, Template)]
#[template(
    source = r#"interface {{ ident }}{{ type_params|display_opt }} {{ extends_clause|display_opt }} {{- obj_type -}}"#,
    ext = "txt"
)]
/// An interface declaration,
/// supports generics parameters and extends
pub struct InterfaceDeclaration {
    pub ident: TSIdent,
    // TODO: Be consistent with TypeAliasDeclaration
    pub type_params: Option<TypeParameters>,
    pub extends_clause: Option<InterfaceExtendsClause>,
    pub obj_type: ObjectType,
}

#[cfg(test)]
pub mod tests {
    use std::str::FromStr;

    use crate::types::{
        PrimaryType, PropertyName, PropertySignature, TsType, TypeBody, TypeMember, TypeName,
    };

    use super::*;

    #[test]
    fn display_interface_type_list() {
        assert_eq!(
            InterfaceTypeList {
                identifiers: vec![
                    TypeReference {
                        name: TypeName {
                            ident: TSIdent::from_str("Test").unwrap(),
                            namespace: None,
                        },
                        args: None,
                    },
                    TypeReference {
                        name: TypeName {
                            ident: TSIdent::from_str("TestOther").unwrap(),
                            namespace: None,
                        },
                        args: None,
                    }
                ],
            }
            .to_string(),
            "Test, TestOther",
        );
    }

    #[test]
    fn display_interface_extends_clause() {
        assert_eq!(
            InterfaceExtendsClause {
                type_list: InterfaceTypeList {
                    identifiers: vec![
                        TypeReference {
                            name: TypeName {
                                ident: TSIdent::from_str("Test").unwrap(),
                                namespace: None,
                            },
                            args: None,
                        },
                        TypeReference {
                            name: TypeName {
                                ident: TSIdent::from_str("TestOther").unwrap(),
                                namespace: None,
                            },
                            args: None,
                        }
                    ],
                }
            }
            .to_string(),
            "extends Test, TestOther",
        );
    }

    #[test]
    fn display_interface_declaration() {
        assert_eq!(
            InterfaceDeclaration {
                ident: TSIdent::from_str("MyInterface").unwrap(),
                extends_clause: None,
                type_params: None,
                obj_type: ObjectType { body: None },
            }
            .to_string(),
            "interface MyInterface {\n\t\n}"
        );

        assert_eq!(
            InterfaceDeclaration {
                ident: TSIdent::from_str("MyInterface").unwrap(),
                extends_clause: None,
                type_params: None,
                obj_type: ObjectType {
                    body: Some(TypeBody {
                        members: vec![
                            TypeMember::PropertySignature(PropertySignature {
                                name: PropertyName::from("value".to_string()),
                                optional: false,
                                inner_type: TsType::PrimaryType(PrimaryType::Predefined(
                                    crate::types::PredefinedType::Number
                                )),
                            }),
                            TypeMember::PropertySignature(PropertySignature {
                                name: PropertyName::from("name".to_string()),
                                optional: true,
                                inner_type: TsType::PrimaryType(PrimaryType::Predefined(
                                    crate::types::PredefinedType::String
                                )),
                            })
                        ]
                    })
                },
            }
            .to_string(),
            "interface MyInterface {\n\tvalue: number,\n\tname?: string\n}"
        );
    }
}
