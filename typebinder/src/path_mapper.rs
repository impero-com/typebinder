use std::{collections::HashMap, path::Path};

use crate::error::TsExportError;

#[derive(Debug, Default)]
/// A tool that maps a punctuated path to another.
///
/// The input path is a syn punctuated path with colons, e.g. : std::collections::Vec
/// The output is a TS import path, e.g: types/user
pub struct PathMapper {
    map: HashMap<String, String>,
}

impl PathMapper {
    pub fn add_mapping<S: Into<String>>(&mut self, path: S, output: S) {
        self.map.insert(path.into(), output.into());
    }

    pub fn map(&self, path: &str) -> Option<String> {
        self.map.get(path).cloned()
    }

    pub fn load_from<P: AsRef<Path>>(path: P) -> Result<Self, TsExportError> {
        let content = std::fs::read_to_string(path)?;
        let map: HashMap<String, String> = serde_json::from_str(&content)?;
        Ok(PathMapper { map })
    }
}
