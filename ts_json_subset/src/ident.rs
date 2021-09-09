use displaythis::Display;
use std::str::FromStr;
use thiserror::Error;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq, Display, Hash)]
#[display("{0}")]
// A valid TS identifier
pub struct TSIdent(String);

#[derive(Debug, Clone, PartialEq, Eq, Display, Hash)]
#[display("{0}")]
/// A TS identifier that is also checked for reserved keywords
pub struct StrictTSIdent(TSIdent);


lazy_static! {
    static ref REGEX_TS_IDENT: Regex = Regex::new("^[a-zA-Z_$]+[a-zA-Z1-9_$]*$").unwrap();
    static ref RESERVED: [&'static str; 36] = [
        "break",
        "case",
        "catch",
        "class",
        "const",
        "continue",
        "debugger",
        "default",
        "delete",
        "do",
        "else",
        "enum",
        "export",
        "extends",
        "false",
        "finally",
        "for",
        "function",
        "if",
        "import",
        "in",
        "instanceOf",
        "new",
        "null",
        "return",
        "super",
        "switch",
        "this",
        "throw",
        "true",
        "try",
        "typeOf",
        "var",
        "void",
        "while",
        "with",
    ];
}

#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum IdentError {
    #[error("The identifier {} is a reserved keyword", _0)]
    ReservedKeyword(String),
    #[error("The identifier {} is incorrect", _0)]
    InvalidIdent(String),
}

impl FromStr for TSIdent {
    type Err = IdentError;
    fn from_str(input: &str) -> Result<Self, IdentError> {
        if !REGEX_TS_IDENT.is_match(input) {
            return Err(IdentError::InvalidIdent(input.to_string()));
        }

        Ok(TSIdent(input.to_string()))
    }
}

impl FromStr for StrictTSIdent {
    type Err = IdentError;

    fn from_str(input: &str) -> Result<Self, IdentError> {
        if RESERVED.contains(&input.to_lowercase().as_str()) {
            return Err(IdentError::ReservedKeyword(input.to_string()));
        }

        Ok(StrictTSIdent(TSIdent::from_str(input)?))
    }
}

impl From<StrictTSIdent> for TSIdent {
    fn from(source: StrictTSIdent) -> Self {
        source.0
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn should_work_when_ident_is_reserved_keyword() {
        assert_eq!(
            TSIdent::from_str("void"),
            Ok(TSIdent("void".to_string()))
        );
        assert_eq!(
            TSIdent::from_str("break"),
            Ok(TSIdent("break".to_string()))
        );
    }

    #[test]
    pub fn should_fail_when_strict_ident_is_reserved_keyword() {
        assert_eq!(
            StrictTSIdent::from_str("void"),
            Err(IdentError::ReservedKeyword("void".to_string()))
        );
        assert_eq!(
            StrictTSIdent::from_str("break"),
            Err(IdentError::ReservedKeyword("break".to_string()))
        );
    }

    #[test]
    pub fn should_fail_when_ident_is_invalid() {
        assert_eq!(
            TSIdent::from_str("2my_invalid_ident"),
            Err(IdentError::InvalidIdent("2my_invalid_ident".to_string()))
        );
        assert_eq!(
            StrictTSIdent::from_str("break"),
            Err(IdentError::ReservedKeyword("break".to_string()))
        );
    }

    #[test]
    pub fn valid_ident() {
        assert_eq!(
            TSIdent::from_str("MyValidIdent"),
            Ok(TSIdent("MyValidIdent".to_string())),
        );
        assert_eq!(
            TSIdent::from_str("my_valid_ident"),
            Ok(TSIdent("my_valid_ident".to_string())),
        );
        assert_eq!(
            TSIdent::from_str("_my_valid_ident"),
            Ok(TSIdent("_my_valid_ident".to_string())),
        );
        assert_eq!(
            TSIdent::from_str("$my_ident"),
            Ok(TSIdent("$my_ident".to_string())),
        );
        assert_eq!(
            TSIdent::from_str("$my_2nd_ident"),
            Ok(TSIdent("$my_2nd_ident".to_string())),
        );
    }
}
