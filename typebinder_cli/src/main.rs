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
use typebinder::exporters::check::CheckExport;

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
    /// Path to the PathMapper definition
    path_mapper_file: Option<PathBuf>,
    #[structopt(subcommand)]
    command: TypebinderCommand,
}

#[derive(Debug, StructOpt)]
enum TypebinderCommand {
    /// Generates the bindings for your Rust files
    Generate {
        #[structopt(short, parse(from_os_str))]
        /// Output path, will use stdout if no path is specified
        output: Option<PathBuf>,
    },
    /// Runs typebinder in "check" mode : no files will be produced.
    ///
    /// This will just compare the existing files to the typebinder output, in order to make
    /// sure that the bindings are up to date.
    ///
    /// This mode is useful when you want to run typebinder in your CI pipeline.
    Check {
        #[structopt(parse(from_os_str))]
        /// Output path where the bindings that we are checking against are stored
        output: PathBuf,
    },
}

fn main() -> Result<(), TsExportError> {
    pretty_env_logger::init();
    let options = Options::from_args();
    main_process(options)
}

fn main_process(options: Options) -> Result<(), TsExportError> {
    let Options {
        input,
        path_mapper_file,
        command,
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
    match command {
        TypebinderCommand::Check { output } => {
            log::info!("Launching Typebinder in check mode");
            Pipeline {
                pipeline_step_spawner,
                exporter: CheckExport::new(output),
                path_mapper,
            }
            .launch(&solving_context, &macro_context)?;
        }
        TypebinderCommand::Generate { output } => match output {
            Some(out_path) => {
                log::info!("Launching Typebinder in FileExporter mode");
                Pipeline {
                    pipeline_step_spawner,
                    exporter: FileExporter::new(out_path),
                    path_mapper,
                }
                .launch(&solving_context, &macro_context)?;
            }
            None => {
                log::info!("Launching Typebinder in StdoutExport mode");
                Pipeline {
                    pipeline_step_spawner,
                    exporter: StdoutExport,
                    path_mapper,
                }
                .launch(&solving_context, &macro_context)?;
            }
        },
    }

    Ok(())
}
