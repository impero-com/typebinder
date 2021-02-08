use display_path::DisplayPath;
use error::TsExportError;
use process::{Exporter, Process, ProcessSpawner};
use type_solver::TypeSolvingContextBuilder;

pub mod display_path;
pub mod error;
pub mod exporter;
pub mod import;
pub mod process;
pub mod solvers;
pub mod type_solver;

use std::{fs::File, io::Read, path::Path};

/// Helper function for demo
pub fn process_file<P: AsRef<Path>>(path: P) -> Result<(), TsExportError> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let solving_context = TypeSolvingContextBuilder::default()
        .add_default_solvers()
        .finish();

    Process {
        content,
        process_spawner: BypassProcessSpawner,
        exporter: StdoutExport,
    }
    .launch(&solving_context)?;

    Ok(())
}

pub struct StdoutExport;

impl Exporter for StdoutExport {
    fn export_module(&self, process_result: process::ProcessModuleResultData) {
        println!("------");
        let mut display_path = DisplayPath(&process_result.path).to_string();
        if display_path.is_empty() {
            display_path = "Default module".to_string();
        }
        println!("{}", display_path);
        println!("------");
        let output: String = process_result
            .statements
            .into_iter()
            .map(|statement| format!("{}\n", statement))
            .collect();
        println!("{}", output);
    }
}

/// Strategy that discards any external module
pub struct BypassProcessSpawner;

impl ProcessSpawner for BypassProcessSpawner {
    fn create_process(&self, _path: syn::Path) -> Option<process::ProcessModule> {
        None
    }
}
