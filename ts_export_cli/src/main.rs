use std::{fs::File, io::Read, path::PathBuf};

use structopt::StructOpt;
use ts_export::{error::TsExportError, type_solver::TypeSolvingContextBuilder};
use ts_export::{
    exporters::stdout::StdoutExport, process::Process,
    process_spawner::discard::BypassProcessSpawner,
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
}

fn main() -> Result<(), TsExportError> {
    let options = Options::from_args();
    main_process(options)
}

fn main_process(options: Options) -> Result<(), TsExportError> {
    let content: String = match options.input {
        Some(path) => {
            let mut file = File::open(path)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            content
        }
        None => {
            let mut stdin = std::io::stdin();
            let mut content = String::new();
            stdin.read_to_string(&mut content)?;
            content
        }
    };

    let solving_context = TypeSolvingContextBuilder::default()
        .add_default_solvers()
        .finish();

    match options.output {
        Some(_path) => {
            /*
            let mut file = File::open(path)?;
            file.write(output.as_bytes())?;
            */
            todo!()
        }
        None => {
            Process {
                content,
                process_spawner: BypassProcessSpawner,
                exporter: StdoutExport,
            }
            .launch(&solving_context)?;
        }
    }

    Ok(())
}
