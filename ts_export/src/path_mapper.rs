use std::collections::HashMap;

#[derive(Debug, Default)]
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
}
