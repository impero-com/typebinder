use super::Exporter;
use crate::error::TsExportError;
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

    fn render_header_comment(&self, rust_module_path: &syn::Path) -> Option<String> {
        match &self.header_comment {
            HeaderComment::None => None,
            HeaderComment::Custom(comment) => Some(format!("/* {} */", comment)),
            HeaderComment::Standard => {
                let header = format!(
                    "// This file was auto-generated with typebinder from Rust source code. Do not change this file manually.\n\
                     // Change the Rust source code instead and regenerate with typebinder.\n\
                     // Rust source module: {}",
                     DisplayPath(&rust_module_path)
                );
                Some(header)
            }
        }
    }
}

impl Exporter for FileExporter {
    type Error = TsExportError;

    fn export_module(&mut self, process_result: ModuleStepResultData) -> Result<(), TsExportError> {
        log::info!("Exporting module {}", DisplayPath(&process_result.path));

        let header = self.render_header_comment(&process_result.path);
        let mut file_path: PathBuf = if process_result.path.segments.is_empty() {
            self.default_module_name
                .clone()
                .unwrap_or_else(|| "index".to_string())
                .into()
        } else {
            process_result
                .path
                .segments
                .into_iter()
                .map(|segm| segm.ident.to_string())
                .collect()
        };
        file_path.set_extension("ts");
        let mut path = self.root_path.clone();
        path.push(file_path);

        let main_content: String = process_result
            .imports
            .into_iter()
            .map(|statement| format!("{}\n", statement))
            .chain(
                process_result
                    .exports
                    .into_iter()
                    .map(|stm| format!("{}\n", stm.to_string())),
            )
            .collect();

        let file_contents = match header {
            None => main_content,
            Some(comment) => format!("{}\n\n{}", comment, main_content),
        };

        log::info!("Outputting module at {:?}", path);
        if let Err(e) =
            std::fs::create_dir_all(&path.parent().expect("Failed to get dir of output module"))
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

pub enum HeaderComment {
    Standard,
    Custom(String),
    None,
}
