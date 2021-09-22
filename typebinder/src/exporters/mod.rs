//! How to output your bindings
use crate::{
    error::TsExportError, pipeline::module_step::ModuleStepResultData,
    utils::display_path::DisplayPath,
};

pub mod check;
pub mod file;
pub mod stdout;
pub mod utils;

/// An abstraction that specifies the behaviour of how to handle a resulting process' data
pub trait Exporter {
    type Error: Into<TsExportError>;

    /// Consumes the process result to do something with it
    fn export_module(&mut self, process_result: ModuleStepResultData) -> Result<(), Self::Error>;

    /// Called when the exporter's process is done
    fn finish(self)
    where
        Self: Sized,
    {
    }
}

pub enum HeaderComment {
    Standard,
    Custom(String),
    None,
}

impl HeaderComment {
    pub fn render(&self, rust_module_path: &syn::Path) -> Option<String> {
        match &self {
            HeaderComment::None => None,
            HeaderComment::Custom(comment) => Some(format!("/* {} */", comment)),
            HeaderComment::Standard => {
                let header = format!(
                    "// This file was auto-generated with typebinder from Rust source code. Do not change this file manually.\n\
                     // Change the Rust source code instead and regenerate with typebinder.\n\
                     // Rust source module: {}",
                     DisplayPath(&rust_module_path)
                );
                Some(header)
            }
        }
    }
}
