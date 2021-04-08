use std::convert::TryFrom;

use displaythis::Display;

/// Askama filters
pub mod filters {
    pub fn display_opt<T: std::fmt::Display>(value: &Option<T>) -> askama::Result<String> {
        match value {
            Some(val) => Ok(val.to_string()),
            None => Ok("".to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Display)]
#[display("\"{0}\"")]
/// An escaped string literal, surrounded by double quotes.
pub struct StringLiteral(String);

impl StringLiteral {
    pub fn from_raw(input: &str) -> Self {
        StringLiteral::from(input.to_string())
    }
}

impl From<String> for StringLiteral {
    fn from(input: String) -> Self {
        StringLiteral(sanitize_string_literal(&input))
    }
}

pub fn sanitize_string_literal(input: &str) -> String {
    input.escape_default().collect()
}

#[derive(Debug, Clone, PartialEq, Display)]
#[display("{0}")]
/// A numeric literal, must have a numeric value (ie. no Infinity, no NaN)
pub struct NumericLiteral(f64);

#[derive(Debug, Clone, PartialEq, Display)]
#[display("{0} is not a valid numeric literal")]
pub struct WrongNumericLiteral(f64);

impl TryFrom<f64> for NumericLiteral {
    type Error = WrongNumericLiteral;

    fn try_from(input: f64) -> Result<Self, Self::Error> {
        if input.is_finite() {
            return Ok(NumericLiteral(input));
        }
        return Err(WrongNumericLiteral(input));
    }
}

#[derive(Debug, Clone, PartialEq, Display)]
#[display("{0}")]
/// A boolean literal
pub struct BooleanLiteral(bool);

impl From<bool> for BooleanLiteral {
    fn from(input: bool) -> Self {
        BooleanLiteral(input)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn should_escape_string_literal() {
        assert_eq!(
            StringLiteral::from_raw("my \"string\" literal"),
            StringLiteral("my \\\"string\\\" literal".to_string()),
        );
    }

    #[test]
    pub fn should_validate_numeric_literal() {
        assert_eq!(NumericLiteral::try_from(1.2), Ok(NumericLiteral(1.2)),);
        assert!(matches!(
            NumericLiteral::try_from(f64::INFINITY),
            Err(WrongNumericLiteral(_)),
        ));
        assert!(matches!(
            NumericLiteral::try_from(f64::NAN),
            Err(WrongNumericLiteral(_)),
        ));
    }
}
