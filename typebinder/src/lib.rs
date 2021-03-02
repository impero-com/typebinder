/// Typebinder is an engine that translates your Rust source code to TypeScript interfaces declarations.
///
/// It is meant to be modular, as opposed to hardcoding the translation between a Rust type and a TypeScript one.
///
/// Typebinder works by starting a Pipeline, and customizing how you want the
/// pipeline to treat Rust "input" modules, and how to output the modules.  
///
/// Typebinder resolves Rust types to their TypeScript definition by using the abstraction called TypeSolver.
/// A bunch of default solvers are already implemented and cover the types from the standard library. For special purposes, you can also implement your own.
///
use contexts::type_solving::TypeSolvingContextBuilder;
use error::TsExportError;
use exporters::stdout::StdoutExport;
use macros::context::MacroSolvingContext;
use path_mapper::PathMapper;
use pipeline::Pipeline;
use step_spawner::mod_reader::RustModuleReader;

pub mod contexts;
pub mod error;
pub mod exporters;
pub mod macros;
pub mod path_mapper;
pub mod pipeline;
pub mod step_spawner;
pub mod type_solving;
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
        pipeline_step_spawner: RustModuleReader::try_new(path.as_ref().to_path_buf())?,
        exporter: StdoutExport,
        path_mapper: PathMapper::default(),
    }
    .launch(&solving_context, &macro_context)?;

    Ok(())
}
