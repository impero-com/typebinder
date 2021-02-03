use crate::error::TsExportError;
use std::collections::HashMap;
use syn::{
    punctuated::Punctuated, token::Colon2, File, Ident, Item, Path, PathArguments, PathSegment,
    TypePath, UseTree,
};

/// All imports of interest from Rust's prelude (not importing Traits, functions and macros)
const PRELUDE: &'static str = r#"
    pub use std::option::Option;
    pub use std::result::Result;
    pub use std::boxed::Box;
    pub use std::string::String;
    pub use std::vec::Vec;"#;

pub struct ImportContext {
    imported: ImportList,
    scoped: ImportList,
    // TODO: Maybe remove, this should probably be static ?
    prelude: ImportList,
}

impl Default for ImportContext {
    fn default() -> Self {
        let prelude = syn::parse_file(PRELUDE).expect("Failed to read Rust prelude");
        let prelude = import_list_from_ast(&prelude);

        ImportContext {
            imported: Default::default(),
            scoped: Default::default(),
            prelude,
        }
    }
}

#[derive(Debug, Default)]
/// An ImportList matches the last segment to the rest of
pub struct ImportList(HashMap<Ident, Vec<PathSegment>>);

impl std::ops::Deref for ImportList {
    type Target = HashMap<Ident, Vec<PathSegment>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ImportList {
    // TODO: maybe fix the space-complexity of this function that clones PathSegments all the way
    pub fn add_use_tree(&mut self, mut segments: Vec<PathSegment>, use_tree: &UseTree) {
        match use_tree {
            UseTree::Path(path) => {
                let new_segment = PathSegment {
                    ident: path.ident.clone(),
                    arguments: PathArguments::None,
                };
                segments.push(new_segment);
                self.add_use_tree(segments.clone(), path.tree.as_ref())
            }
            UseTree::Name(name) => {
                // TODO: check ident == self
                self.0.insert(name.ident.clone(), segments);
            }
            UseTree::Rename(rename) => {
                // TODO: check ident == self
                self.0.insert(rename.rename.clone(), segments);
            }
            UseTree::Group(group) => {
                group
                    .items
                    .iter()
                    .for_each(|use_tree| self.add_use_tree(segments.clone(), use_tree));
            }
            UseTree::Glob(_) => (),
        }
    }
}

pub fn import_list_from_ast(file: &File) -> ImportList {
    let mut import_list = ImportList::default();
    for item_use in file.items.iter().filter_map(|item| match item {
        Item::Use(item) => Some(item),
        _ => None,
    }) {
        import_list.add_use_tree(Vec::new(), &item_use.tree);
    }
    import_list
}

impl ImportContext {
    pub fn solve_import(&self, ty_path: TypePath) -> Result<syn::Type, TsExportError> {
        let segment = ty_path.path.segments.first().expect("Empty path");
        let ident = &segment.ident;
        let found_segments = self
            .imported
            .get(ident)
            .or_else(|| self.scoped.get(ident))
            .or_else(|| self.prelude.get(ident))
            .ok_or_else(|| TsExportError::UnsolvedType(ty_path.clone().into()))?;

        let segments = found_segments
            .iter()
            .cloned()
            .chain(ty_path.path.segments.into_iter())
            .collect::<Punctuated<PathSegment, Colon2>>();

        let path = Path {
            leading_colon: None,
            segments,
        };

        Ok(TypePath { qself: None, path }.into())
    }
}

#[cfg(test)]
pub mod tests {
    use crate::display_path::DisplayPath;
    use syn::__private::Span;

    use super::*;

    #[test]
    fn test_import_prelude() {
        let src = syn::parse_file(PRELUDE).expect("Failed to parse PRELUDE");
        let import_list = import_list_from_ast(&src);

        let string = import_list
            .get(&Ident::new("String", Span::call_site()))
            .expect("Failed to get String");
        let path = Path {
            leading_colon: None,
            segments: string.clone().into_iter().collect(),
        };
        assert_eq!(DisplayPath(&path).to_string(), "std::string");
    }
}
