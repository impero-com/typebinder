use crate::common::filters;
use crate::types::{TsType, TypeParameters};
use askama::Template;

#[derive(Debug, Clone, PartialEq, Template)]
#[template(
    source = "type {{ ident }} {{- params|display_opt }} = {{ inner_type }};",
    ext = "txt"
)]
/// A type alias declaration,
/// supports generics parameters
pub struct TypeAliasDeclaration {
    // TODO: Make an identifier type that checks TS constraints on identifiers
    pub ident: String,
    // TODO: Be consistent with InterfaceDeclaration
    pub params: Option<TypeParameters>,
    pub inner_type: TsType,
}

#[cfg(test)]
pub mod tests {
    use crate::types::{PredefinedType, PrimaryType};

    use super::*;

    #[test]
    fn display_type_alias_declaration() {
        assert_eq!(
            TypeAliasDeclaration {
                ident: "MyType".to_string(),
                params: None,
                inner_type: TsType::PrimaryType(PrimaryType::Predefined(PredefinedType::Any)),
            }
            .to_string(),
            "type MyType = any;",
        )
    }
}
