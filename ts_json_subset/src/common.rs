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

// TODO: Remove the string copy ?
// TODO: Check that input is escaped
impl<'a> From<&'a str> for StringLiteral {
    fn from(input: &str) -> Self {
        StringLiteral(input.to_string())
    }
}

// TODO: Check that input is escaped
impl From<String> for StringLiteral {
    fn from(input: String) -> Self {
        StringLiteral(input)
    }
}

#[derive(Debug, Clone, PartialEq, Display)]
#[display("{0}")]
/// A numeric literal, must have a numeric value (ie. no Infinity, no NaN)
pub struct NumericLiteral(f64);

// TODO: Check that input is a numeric value, use f64::is_finite
impl From<f64> for NumericLiteral {
    fn from(input: f64) -> Self {
        NumericLiteral(input)
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
