use crate::types::{TsType, TypeParameters};
use crate::{common::filters, ident::TSIdent};
use askama::Template;

#[derive(Debug, Clone, PartialEq, Template)]
#[template(
    source = "type {{ ident }} {{- type_params|display_opt }} = {{ inner_type }};",
    ext = "txt"
)]
/// A type alias declaration,
/// supports generics parameters
pub struct TypeAliasDeclaration {
    pub ident: TSIdent,
    pub type_params: Option<TypeParameters>,
    pub inner_type: TsType,
}

#[cfg(test)]
pub mod tests {
    use std::str::FromStr;

    use crate::types::{PredefinedType, PrimaryType};

    use super::*;

    #[test]
    fn display_type_alias_declaration() {
        assert_eq!(
            TypeAliasDeclaration {
                ident: TSIdent::from_str("MyType").unwrap(),
                type_params: None,
                inner_type: TsType::PrimaryType(PrimaryType::Predefined(PredefinedType::Any)),
            }
            .to_string(),
            "type MyType = any;",
        )
    }
}
