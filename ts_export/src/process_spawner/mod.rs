use syn::Path;

use crate::error::TsExportError;
use crate::process::ProcessModule;

pub mod discard;
pub mod mod_reader;

/// Creates a ProcessModule from a Path
pub trait ProcessSpawner {
    type Error: Into<TsExportError>;
    fn create_process(&self, path: Path) -> Result<Option<ProcessModule>, Self::Error>;
}
