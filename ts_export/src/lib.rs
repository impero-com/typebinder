use error::TsExportError;
use process::Process;

pub mod display_path;
pub mod error;
pub mod exporter;
pub mod process;
pub mod solvers;
pub mod type_solver;

use std::{fs::File, io::Read, path::Path};

/// Helper function for demo
pub fn process_file<P: AsRef<Path>>(path: P) -> Result<(), TsExportError> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    println!("{}", Process { content }.launch()?);

    Ok(())
}
