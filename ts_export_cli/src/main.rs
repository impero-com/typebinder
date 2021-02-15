use std::{io::Read, path::PathBuf};

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
    #[structopt(short, parse(from_os_str))]
    /// Input file, will use stdin if no file is specified
    input: Option<PathBuf>,
    #[structopt(short, parse(from_os_str))]
    /// Output file, will use stdout if no file is specified
    output: Option<PathBuf>,
    #[structopt(short, parse(from_os_str))]
    /// Path to the PathMapper definition
    path_mapper_file: Option<PathBuf>,
}

fn main() -> Result<(), TsExportError> {
    let options = Options::from_args();
    main_process(options)
}

fn main_process(options: Options) -> Result<(), TsExportError> {
    let (content, path) = match options.input {
        Some(path) => {
            let content = std::fs::read_to_string(&path)?;
            let path = path.parent().map(std::path::Path::to_owned);
            (content, path)
        }
        None => {
            let mut stdin = std::io::stdin();
            let mut content = String::new();
            stdin.read_to_string(&mut content)?;
            (content, None)
        }
    };

    let process_spawner = path
        .map(|path| RustModuleReader::new(path))
        .unwrap_or_default();

    let solving_context = TypeSolvingContextBuilder::default()
        .add_default_solvers()
        .finish();

    let path_mapper = if let Some(path) = options.path_mapper_file {
        PathMapper::load_from(path)?
    } else {
        PathMapper::default()
    };

    match options.output {
        Some(out_path) => {
            Process {
                content,
                process_spawner,
                exporter: FileExporter::new(out_path),
                path_mapper,
            }
            .launch(&solving_context)?;
        }
        None => {
            Process {
                content,
                process_spawner,
                exporter: StdoutExport,
                path_mapper,
            }
            .launch(&solving_context)?;
        }
    }

    Ok(())
}
