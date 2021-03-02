//! How to load Rust input modules

use syn::Path;

use crate::error::TsExportError;
use crate::pipeline::module_step::ModuleStep;

pub mod discard;
pub mod mod_reader;

/// An abstraction that specifies how to create a Step of the pipeline.
///
/// When a Rust `module` is referenced in a file, this Process
pub trait PipelineStepSpawner {
    type Error: Into<TsExportError>;
    fn create_process(&self, path: Path) -> Result<Option<ModuleStep>, Self::Error>;
}
