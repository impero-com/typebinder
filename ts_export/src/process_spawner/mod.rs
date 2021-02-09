use syn::Path;

use crate::process::ProcessModule;

pub mod discard;

/// Creates a ProcessModule from a Path
pub trait ProcessSpawner {
    fn create_process(&self, path: Path) -> Option<ProcessModule>;
}
