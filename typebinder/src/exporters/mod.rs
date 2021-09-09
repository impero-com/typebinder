//! How to output your bindings
use crate::{error::TsExportError, pipeline::module_step::ModuleStepResultData};

pub mod file;
pub mod stdout;

/// An abstraction that specifies the behaviour of how to handle a resulting process' data
pub trait Exporter {
    type Error: Into<TsExportError>;

    fn export_module(&mut self, process_result: ModuleStepResultData) -> Result<(), Self::Error>;
}
