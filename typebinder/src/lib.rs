use error::TsExportError;
use exporters::stdout::StdoutExport;
use macros::context::MacroSolvingContext;
use path_mapper::PathMapper;
use pipeline::Pipeline;
use process_spawner::mod_reader::RustModuleReader;
use type_solver::TypeSolvingContextBuilder;

pub mod display_path;
pub mod error;
pub mod exporter_context;
pub mod exporters;
pub mod import;
pub mod macros;
pub mod path_mapper;
pub mod pipeline;
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

    let macro_context = MacroSolvingContext::default();

    Pipeline {
        process_spawner: RustModuleReader::try_new(path.as_ref().to_path_buf())?,
        exporter: StdoutExport,
        path_mapper: PathMapper::default(),
    }
    .launch(&solving_context, &macro_context)?;

    Ok(())
}
