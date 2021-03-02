//! `typebinder_cli` is a CLI tool to launch `typebinder` on a Rust file, providing `TypeScript` bindings for your Rust source code.
//!
//! Example usage :
//!
//! Given the following project structure
//!
//! ```
//! + my_crate
//! |
//! +- Cargo.toml
//! +- src
//!    |
//!    +- lib.rs
//!    +- models.rs
//! ```
//!
//! `typebinder_cli src/models.rs -o types`
//!
//! -->
//!
//! ```
//! + my_crate
//! |
//! +- Cargo.toml
//! +- src
//! |  |
//! |  +- lib.rs
//! |  +- models.rs
//! +- types
//!    |
//!    +- index.ts
//! ```
//!
use std::path::PathBuf;

use structopt::StructOpt;
use typebinder::{
    contexts::type_solving::TypeSolvingContextBuilder,
    error::TsExportError,
    exporters::{file::FileExporter, stdout::StdoutExport},
    macros::context::MacroSolvingContext,
    path_mapper::PathMapper,
    pipeline::Pipeline,
    step_spawner::mod_reader::RustModuleReader,
};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "typebinder_cli",
    about = "Exports TS definitions from a Rust module"
)]
/// CLI arguments
struct Options {
    #[structopt(parse(from_os_str))]
    /// Rust module to generate the bindings for
    input: PathBuf,
    #[structopt(short, parse(from_os_str))]
    /// Output file, will use stdout if no file is specified
    output: Option<PathBuf>,
    #[structopt(short, parse(from_os_str))]
    /// Path to the PathMapper definition
    path_mapper_file: Option<PathBuf>,
}

fn main() -> Result<(), TsExportError> {
    pretty_env_logger::init();
    let options = Options::from_args();
    main_process(options)
}

fn main_process(options: Options) -> Result<(), TsExportError> {
    let Options {
        input,
        output,
        path_mapper_file,
    } = options;

    let pipeline_step_spawner = RustModuleReader::try_new(input)?;

    let solving_context = TypeSolvingContextBuilder::default()
        .add_default_solvers()
        .finish();

    let macro_context = MacroSolvingContext::default();

    let path_mapper = if let Some(path) = path_mapper_file {
        PathMapper::load_from(path)?
    } else {
        PathMapper::default()
    };

    match output {
        Some(out_path) => {
            Pipeline {
                pipeline_step_spawner,
                exporter: FileExporter::new(out_path),
                path_mapper,
            }
            .launch(&solving_context, &macro_context)?;
        }
        None => {
            Pipeline {
                pipeline_step_spawner,
                exporter: StdoutExport,
                path_mapper,
            }
            .launch(&solving_context, &macro_context)?;
        }
    }

    Ok(())
}
