use error::TsExportError;
use exporters::stdout::StdoutExport;
use path_mapper::PathMapper;
use process::Process;
use process_spawner::mod_reader::RustModuleReader;
use type_solver::TypeSolvingContextBuilder;

pub mod display_path;
pub mod error;
pub mod exporter_context;
pub mod exporters;
pub mod import;
pub mod path_mapper;
pub mod process;
pub mod process_spawner;
pub mod solvers;
pub mod type_solver;
pub mod utils;

pub use syn;
pub use ts_json_subset as ts;

use std::path::Path;

/// Helper function for demo
pub fn process_file<P: AsRef<Path>>(path: P) -> Result<(), TsExportError> {
    let solving_context = TypeSolvingContextBuilder::default()
        .add_default_solvers()
        .finish();

    Process {
        process_spawner: RustModuleReader::try_new(path.as_ref().to_path_buf())?,
        exporter: StdoutExport,
        path_mapper: PathMapper::default(),
    }
    .launch(&solving_context)?;

    Ok(())
}
