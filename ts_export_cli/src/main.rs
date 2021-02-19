use std::path::PathBuf;

use structopt::StructOpt;
use ts_export::{
    error::TsExportError,
    exporters::{file::FileExporter, stdout::StdoutExport},
    path_mapper::PathMapper,
    process::Process,
    process_spawner::mod_reader::RustModuleReader,
    type_solver::TypeSolvingContextBuilder,
};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "ts_export_cli",
    about = "Exports TS definitions from a Rust module"
)]
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

    let process_spawner = RustModuleReader::try_new(input)?;

    let solving_context = TypeSolvingContextBuilder::default()
        .add_default_solvers()
        .finish();

    let path_mapper = if let Some(path) = path_mapper_file {
        PathMapper::load_from(path)?
    } else {
        PathMapper::default()
    };

    match output {
        Some(out_path) => {
            Process {
                process_spawner,
                exporter: FileExporter::new(out_path),
                path_mapper,
            }
            .launch(&solving_context)?;
        }
        None => {
            Process {
                process_spawner,
                exporter: StdoutExport,
                path_mapper,
            }
            .launch(&solving_context)?;
        }
    }

    Ok(())
}
