use syn::Path;

use crate::process::ProcessModule;
use crate::error::TsExportError;
use std::fmt::Debug;

pub mod discard;
pub mod mod_reader;

/// Creates a ProcessModule from a Path
pub trait ProcessSpawner {
    type Error: Debug + Into<TsExportError>;
    fn create_process(&self, path: Path) -> Result<Option<ProcessModule>, Self::Error>;
}
