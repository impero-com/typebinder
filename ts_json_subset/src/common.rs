use displaythis::Display;

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
pub struct StringLiteral(pub String);

impl<'a> From<&'a str> for StringLiteral {
    fn from(input: &str) -> Self {
        StringLiteral(input.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Display)]
#[display("{0}")]
pub struct NumericLiteral(pub f64);

impl From<f64> for NumericLiteral {
    fn from(input: f64) -> Self {
        NumericLiteral(input)
    }
}

#[derive(Debug, Clone, PartialEq, Display)]
#[display("{0}")]
pub struct BooleanLiteral(pub bool);

impl From<bool> for BooleanLiteral {
    fn from(input: bool) -> Self {
        BooleanLiteral(input)
    }
}
