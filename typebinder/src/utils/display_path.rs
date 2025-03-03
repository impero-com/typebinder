use std::fmt::Display;
use std::string::ToString;

use syn::Path;

/// Wrapper around syn::Path that implements Display and Debug
#[derive(Debug)]
pub struct DisplayPath<'a>(pub &'a Path);

impl Display for DisplayPath<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .segments
                .iter()
                .map(|segment| segment.ident.to_string())
                .collect::<Vec<String>>()
                .join("::")
        )
    }
}
