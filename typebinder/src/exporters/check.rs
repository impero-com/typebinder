use crate::exporters::utils::{get_file_contents, get_output_file_path};
use crate::{
    error::TsExportError,
    exporters::{Exporter, HeaderComment},
    pipeline::module_step::ModuleStepResultData,
};
use displaythis::Display;
use std::collections::HashMap;
use std::io::Read;
use std::path::PathBuf;

pub struct CheckExport {
    root_path: PathBuf,
    default_module_name: Option<String>,
    header_comment: HeaderComment,
    patches: HashMap<PathBuf, DiffPatch>,
}

impl CheckExport {
    pub fn new(root_path: PathBuf) -> Self {
        CheckExport {
            root_path,
            default_module_name: None,
            header_comment: HeaderComment::Standard,
            patches: HashMap::default(),
        }
    }
}

#[derive(Debug, Display)]
pub enum DiffKind {
    #[display("+{0}")]
    Added(String),
    #[display("-{0}")]
    Removed(String),
}

#[derive(Debug, Display)]
#[display("{line}: {kind}")]
pub struct DiffChange {
    line: usize,
    kind: DiffKind,
}

#[derive(Debug)]
pub enum DiffPatch {
    NewFile(String),
    Changes(Vec<DiffChange>),
}

struct FileChange {
    path: PathBuf,
    diff: DiffPatch,
}

impl std::fmt::Display for FileChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        // This could maybe use a bit of a cleanup to display the change in a prettier way
        let path_str = self.path.to_str().expect("Path was not valid UTF8");
        match &self.diff {
            DiffPatch::NewFile(contents) => {
                write!(
                    f,
                    "NEW MODULE {} DOES NOT EXIST IN THE BINDINGS YET:\n{}",
                    path_str, &contents
                )?;
            }
            DiffPatch::Changes(changes) => {
                write!(f, "MODULE {} IS NOT UP TO DATE:", path_str)?;
                changes
                    .iter()
                    .map(|c| write!(f, "{}", c))
                    .collect::<Result<(), _>>()?;
            }
        }
        Ok(())
    }
}

impl Exporter for CheckExport {
    type Error = TsExportError;

    fn export_module(&mut self, process_result: ModuleStepResultData) -> Result<(), TsExportError> {
        let path =
            get_output_file_path(&process_result, &self.default_module_name, &self.root_path);
        log::info!("Comparing module at {:?}", path);

        let generated_file_contents = get_file_contents(process_result, &self.header_comment);

        match std::fs::File::open(&path) {
            Ok(mut file) => {
                let mut file_contents = String::new();
                file.read_to_string(&mut file_contents)?;
                let file_diff = diff::lines(&generated_file_contents, &file_contents);
                let changes: Vec<DiffChange> = file_diff
                    .into_iter()
                    .enumerate()
                    .filter_map(|(line, dif)| match dif {
                        diff::Result::Both(_, _) => None,
                        diff::Result::Left(l) => Some(DiffChange {
                            line,
                            kind: DiffKind::Added(l.to_string()),
                        }),
                        diff::Result::Right(r) => Some(DiffChange {
                            line,
                            kind: DiffKind::Removed(r.to_string()),
                        }),
                    })
                    .collect();
                if !changes.is_empty() {
                    self.patches.insert(path, DiffPatch::Changes(changes));
                }

                Ok(())
            }
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => {
                    self.patches
                        .insert(path, DiffPatch::NewFile(generated_file_contents));
                    Ok(())
                }
                _ => Err(e.into()),
            },
        }
    }

    fn finish(self) {
        let is_ok = self.patches.is_empty();
        self.patches
            .into_iter()
            .for_each(|(path, diff)| log::error!("{}", FileChange { path, diff }));

        if !is_ok {
            std::process::exit(1);
        }
    }
}
