use crate::common::filters;
use crate::types::{ObjectType, TypeParameters, TypeReference};
use askama::Template;

#[derive(Debug, Clone, PartialEq, Template)]
#[template(source = "{{ identifiers|join(\", \") }}", ext = "txt")]
pub struct InterfaceTypeList {
    pub identifiers: Vec<TypeReference>,
}

#[derive(Debug, Clone, PartialEq, Template)]
#[template(source = "extends {{ type_list }}", ext = "txt")]
pub struct InterfaceExtendsClause {
    pub type_list: InterfaceTypeList,
}

#[derive(Debug, Clone, PartialEq, Template)]
#[template(
    source = r#"interface {{ ident }}{{ type_params|display_opt }} {{ extends_clause|display_opt }} {{- obj_type -}}"#,
    ext = "txt"
)]
pub struct InterfaceDeclaration {
    pub ident: String,
    pub type_params: Option<TypeParameters>,
    pub extends_clause: Option<InterfaceExtendsClause>,
    pub obj_type: ObjectType,
}

#[cfg(test)]
pub mod tests {
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
                            ident: "Test".to_string(),
                            namespace: None,
                        },
                        args: None,
                    },
                    TypeReference {
                        name: TypeName {
                            ident: "TestOther".to_string(),
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
                                ident: "Test".to_string(),
                                namespace: None,
                            },
                            args: None,
                        },
                        TypeReference {
                            name: TypeName {
                                ident: "TestOther".to_string(),
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
                ident: "MyInterface".to_string(),
                extends_clause: None,
                type_params: None,
                obj_type: ObjectType { body: None },
            }
            .to_string(),
            "interface MyInterface {\n\t\n}"
        );

        assert_eq!(
            InterfaceDeclaration {
                ident: "MyInterface".to_string(),
                extends_clause: None,
                type_params: None,
                obj_type: ObjectType {
                    body: Some(TypeBody {
                        members: vec![
                            TypeMember::PropertySignature(PropertySignature {
                                name: PropertyName::Identifier("value".to_string()),
                                optional: false,
                                inner_type: TsType::PrimaryType(PrimaryType::Predefined(
                                    crate::types::PredefinedType::Number
                                )),
                            }),
                            TypeMember::PropertySignature(PropertySignature {
                                name: PropertyName::Identifier("name".to_string()),
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
