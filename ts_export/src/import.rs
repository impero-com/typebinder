use std::collections::HashMap;
use syn::{
    Ident, Item, Path, PathArguments, PathSegment, __private::Span, punctuated::Punctuated,
    token::Colon2, TypePath, UseTree,
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

impl ImportContext {
    pub fn parse_imported(&mut self, items: &Vec<Item>, crate_name: &str) {
        let import_list = parse_uses(items, crate_name);
        self.imported = import_list;
    }

    pub fn parse_scoped(&mut self, items: &Vec<Item>) {
        // TODO: Append current_path to all declarations
        let import_list = parse_declarations(items);
        self.scoped = import_list;
    }
}

impl Default for ImportContext {
    fn default() -> Self {
        let prelude = syn::parse_file(PRELUDE).expect("Failed to read Rust prelude");
        let prelude = parse_uses(&prelude.items, "crate");

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
    pub fn add_use_tree(
        &mut self,
        mut segments: Vec<PathSegment>,
        use_tree: &UseTree,
        crate_name: &str,
    ) {
        match use_tree {
            UseTree::Path(path) => {
                // TODO: check ident == self
                let new_segment = match path.ident.to_string().as_str() {
                    "crate" => PathSegment {
                        ident: Ident::new(crate_name, Span::call_site()),
                        arguments: PathArguments::None,
                    },
                    _ => PathSegment {
                        ident: path.ident.clone(),
                        arguments: PathArguments::None,
                    },
                };
                segments.push(new_segment);
                self.add_use_tree(segments.clone(), path.tree.as_ref(), crate_name)
            }
            UseTree::Name(name) => {
                self.0.insert(name.ident.clone(), segments);
            }
            UseTree::Rename(rename) => {
                self.0.insert(rename.rename.clone(), segments);
            }
            UseTree::Group(group) => {
                group
                    .items
                    .iter()
                    .for_each(|use_tree| self.add_use_tree(segments.clone(), use_tree, crate_name));
            }
            UseTree::Glob(_) => log::warn!("Glob imports are not supported by ts_export"),
        }
    }

    pub fn add_declaration(&mut self, ident: Ident) {
        self.0.insert(ident, Vec::new());
    }
}

pub fn parse_uses(items: &Vec<Item>, crate_name: &str) -> ImportList {
    let mut import_list = ImportList::default();
    for item_use in items.iter().filter_map(|item| match item {
        Item::Use(item) => Some(item),
        _ => None,
    }) {
        import_list.add_use_tree(Vec::new(), &item_use.tree, crate_name);
    }
    import_list
}

pub fn parse_declarations(items: &Vec<Item>) -> ImportList {
    let mut import_list = ImportList::default();
    items.iter().for_each(|item| match item {
        Item::Enum(item_enum) => import_list.add_declaration(item_enum.ident.clone()),
        Item::Struct(item_struct) => import_list.add_declaration(item_struct.ident.clone()),
        Item::Type(item_type) => import_list.add_declaration(item_type.ident.clone()),
        // TODO: Handle mod declarations
        _ => (),
    });
    import_list
}

impl ImportContext {
    pub fn solve_import(&self, ty_path: &TypePath) -> Option<syn::Type> {
        let segment = ty_path.path.segments.first().expect("Empty path");
        let ident = &segment.ident;
        let found_segments = self
            .imported
            .get(ident)
            .or_else(|| self.scoped.get(ident))
            .or_else(|| self.prelude.get(ident))?;

        let segments = found_segments
            .iter()
            .cloned()
            .chain(ty_path.path.segments.iter().cloned())
            .collect::<Punctuated<PathSegment, Colon2>>();

        let path = Path {
            leading_colon: None,
            segments,
        };

        Some(TypePath { qself: None, path }.into())
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
        let import_list = parse_uses(&src.items, "crate");

        let string = import_list
            .get(&Ident::new("String", Span::call_site()))
            .expect("Failed to get String");
        let path = Path {
            leading_colon: None,
            segments: string.clone().into_iter().collect(),
        };
        assert_eq!(DisplayPath(&path).to_string(), "std::string");
    }

    const EXAMPLE: &'static str = r#"
        struct A {}
        struct B;
        struct C<T> { _t: T }
    "#;

    #[test]
    fn test_import_scoped() {
        let src = syn::parse_file(EXAMPLE).expect("Failed to parse EXAMPLE");
        let import_list = parse_declarations(&src.items);

        let test_a = import_list
            .get(&Ident::new("A", Span::call_site()))
            .expect("Failed to parse A");
        let path = Path {
            leading_colon: None,
            segments: test_a.clone().into_iter().collect(),
        };
        assert_eq!(DisplayPath(&path).to_string(), "");

        let test_b = import_list
            .get(&Ident::new("B", Span::call_site()))
            .expect("Failed to parse B");
        let path = Path {
            leading_colon: None,
            segments: test_b.clone().into_iter().collect(),
        };
        assert_eq!(DisplayPath(&path).to_string(), "");

        let test_c = import_list
            .get(&Ident::new("C", Span::call_site()))
            .expect("Failed to parse C");
        let path = Path {
            leading_colon: None,
            segments: test_c.clone().into_iter().collect(),
        };
        assert_eq!(DisplayPath(&path).to_string(), "");
    }
}
