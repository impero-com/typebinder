use super::Exporter;
use crate::error::TsExportError;
use crate::exporters::utils::{get_file_contents, get_output_file_path};
use crate::exporters::HeaderComment;
use crate::{pipeline::module_step::ModuleStepResultData, utils::display_path::DisplayPath};
use std::{
    io::Write,
    path::{Path, PathBuf},
};

/// A strategy that will output a TS file given its path
pub struct FileExporter {
    root_path: PathBuf,
    default_module_name: Option<String>,
    header_comment: HeaderComment,
}

impl Default for FileExporter {
    fn default() -> Self {
        let root_path = std::env::current_dir().expect("Failed to find current dir");
        FileExporter {
            root_path,
            default_module_name: None,
            header_comment: HeaderComment::Standard,
        }
    }
}

impl FileExporter {
    pub fn new(path: PathBuf) -> Self {
        FileExporter {
            root_path: path,
            default_module_name: None,
            header_comment: HeaderComment::Standard,
        }
    }

    pub fn set_root_path(&mut self, path: PathBuf) {
        self.root_path = path;
    }

    pub fn set_default_module_name(&mut self, default_module_path: &Path) {
        self.default_module_name = default_module_path.file_name().map(|os_str| {
            let os_string = os_str.to_os_string();
            os_string
                .into_string()
                .expect("Invalid UTF-8 name for module")
        });
    }

    pub fn set_header_comment(&mut self, header_comment: HeaderComment) {
        self.header_comment = header_comment;
    }
}

impl Exporter for FileExporter {
    type Error = TsExportError;

    fn export_module(&mut self, process_result: ModuleStepResultData) -> Result<(), TsExportError> {
        log::info!("Exporting module {}", DisplayPath(&process_result.path));
        let path =
            get_output_file_path(&process_result, &self.default_module_name, &self.root_path);

        log::info!("Outputting module at {:?}", path);
        let file_contents = get_file_contents(process_result, &self.header_comment);

        if let Err(e) =
            std::fs::create_dir_all(path.parent().expect("Failed to get dir of output module"))
        {
            match e.kind() {
                std::io::ErrorKind::AlreadyExists => (),
                _ => panic!("{}", e),
            }
        }

        let mut file = std::fs::File::create(&path).expect("Failed to open file");
        file.write_all(file_contents.as_bytes())
            .expect("Failed to write");

        Ok(())
    }
}
