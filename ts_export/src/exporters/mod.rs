use crate::process::ProcessModuleResultData;

pub mod stdout;

/// Specifies the behaviour of how to handle a resulting process' data
pub trait Exporter {
    fn export_module(&self, process_result: ProcessModuleResultData);
}
