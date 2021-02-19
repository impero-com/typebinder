use crate::error::TsExportError;
use crate::process::ProcessModuleResultData;

pub mod file;
pub mod stdout;

/// Specifies the behaviour of how to handle a resulting process' data
pub trait Exporter {
    type Error: Into<TsExportError>;

    fn export_module(&self, process_result: ProcessModuleResultData) -> Result<(), Self::Error>;
}
