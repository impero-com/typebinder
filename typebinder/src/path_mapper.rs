use std::{collections::HashMap, path::Path};

use crate::error::TsExportError;

/*
*/

#[derive(Debug, Default)]
/// A tool that maps a punctuated path (Rust) to a TypeScript module path.
/// Useful when you have a complex codebase that requires multiple `typebinder` passes to generate the bindings.
///
/// The input path is a syn punctuated path with colons, e.g. : `my_crate::models::user`
/// The output is a TS import path, e.g: `types/user`
///
/// The PathMapper is implemented as a tree. For example, given those mappings :
/// * a::api -> types/a
/// * a::models -> types/models/a
/// * b::api -> types/b
///
/// ... the PathMapper will take the following shape:
/// (where `m` means "mapped_ident = ")
///
/// ```text
///  ____x___  -> root
/// /        \
/// x -> "a"  x -> "b"
/// |\___      \____
/// x    x          x
/// ^    ^          ^
/// |    |          |
/// api  models    api
/// ^    ^         ^
/// |    |         m
/// |    m         +--- types/b
/// m    |
/// |    +--- types/models/a
/// |
/// types/a
/// ```
///
/// And when trying to get the resource for a Rust path :
/// * a::api -> types/a
/// * a::api::my::module -> types/a/my/module
/// * a::models::my::module -> types/models/a/my/module
/// * b::api -> types/b
///
/// See the tests for more information
pub struct PathMapper {
    root: PathMapperNode,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct PathMapperNode {
    mapped_ident: String,
    children: HashMap<String, PathMapperNode>,
}

impl PathMapper {
    pub fn add_mapping<AS: AsRef<str>, S: Into<String>>(&mut self, path: AS, output: S) {
        let path = path.as_ref();
        self.root.add_mapping(path, output.into())
    }

    pub fn get(&self, path: &str) -> Option<String> {
        self.root.get(path)
    }

    pub fn load_from<P: AsRef<Path>>(path: P) -> Result<Self, TsExportError> {
        let content = std::fs::read_to_string(path)?;
        Self::load_from_string(&content)
    }

    fn load_from_string(input: &str) -> Result<Self, TsExportError> {
        let map: HashMap<String, String> = serde_json::from_str(input)?;
        let mut root = PathMapperNode::default();
        map.into_iter().for_each(|(k, v)| root.add_mapping(&k, v));
        Ok(PathMapper { root })
    }
}

impl PathMapperNode {
    pub fn get(&self, path: &str) -> Option<String> {
        let mut splitted_path = path.split("::");
        self.get_inner(&mut splitted_path)
    }

    fn get_inner<'a, I: Iterator<Item = &'a str>>(&self, path_iter: &mut I) -> Option<String> {
        let path = path_iter.next();
        if let Some(path) = path {
            if let Some(child) = self.children.get(&path.to_string()) {
                child.get_inner(path_iter)
            } else {
                let rest: Vec<&str> = Some(path).into_iter().chain(path_iter).collect();
                let rest: String = rest.join("/");
                Some(format!("{}/{}", self.mapped_ident, rest))
            }
        } else {
            Some(self.mapped_ident.clone())
        }
    }

    pub fn add_mapping(&mut self, path: &str, output: String) {
        let mut splitted_path = path.split("::");
        self.add_mapping_inner(&mut splitted_path, output)
    }

    fn add_mapping_inner<'a, I: Iterator<Item = &'a str>>(
        &mut self,
        path_iter: &mut I,
        output: String,
    ) {
        let path = path_iter.next();
        if let Some(path) = path {
            let entry = self.children.entry(path.to_string()).or_default();
            entry.add_mapping_inner(path_iter, output)
        } else {
            self.mapped_ident = output;
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::PathMapper;

    #[test]
    fn should_add_mapping() {
        let mut mapper = PathMapper::default();
        mapper.add_mapping("a::b", "types/a/b");
        mapper.add_mapping("a::b::c", "types/a/b/models/c");
        mapper.add_mapping("b::a", "types/b/a");
        mapper.add_mapping("b::b", "types/b/b");

        assert_eq!(mapper.get("a::b"), Some("types/a/b".to_string()));
        assert_eq!(
            mapper.get("a::b::c"),
            Some("types/a/b/models/c".to_string())
        );
        assert_eq!(mapper.get("a::b::d"), Some("types/a/b/d".to_string()));
    }

    const INPUT: &'static str = r#"{
        "a::b": "types/a/b",
        "a::b::c": "types/a/b/models/c",
        "b::a": "types/b/a",
        "b::b": "types/a/b"
    }"#;

    #[test]
    fn should_load_from_json() {
        let mapper = PathMapper::load_from_string(&INPUT).expect("Failed to read PathMapper");
        assert_eq!(mapper.get("a::b"), Some("types/a/b".to_string()));
        assert_eq!(
            mapper.get("a::b::c"),
            Some("types/a/b/models/c".to_string())
        );
        assert_eq!(mapper.get("a::b::d"), Some("types/a/b/d".to_string()));
    }
}
