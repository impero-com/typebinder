use super::Exporter;
use crate::{display_path::DisplayPath, process::ProcessModuleResultData};
use std::{io::Write, path::PathBuf};

pub struct FileExporter {
    root_path: PathBuf,
}

impl Default for FileExporter {
    fn default() -> Self {
        let root_path = std::env::current_dir().expect("Failed to find current dir");
        FileExporter { root_path }
    }
}

impl FileExporter {
    pub fn new(path: PathBuf) -> Self {
        FileExporter { root_path: path }
    }

    pub fn set_root_path(&mut self, path: PathBuf) {
        self.root_path = path;
    }
}

impl Exporter for FileExporter {
    fn export_module(&self, process_result: ProcessModuleResultData) {
        log::info!("Exporting module {}", DisplayPath(&process_result.path));
        let mut file_path: PathBuf = if process_result.path.segments.is_empty() {
            "index".to_string().into()
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

        let file_contents: String = process_result
            .imports
            .into_iter()
            .map(|statement| format!("{}\n", statement))
            .chain(
                process_result
                    .exports
                    .into_iter()
                    .map(|stm| stm.to_string()),
            )
            .collect();

        let mut file = std::fs::File::create(path).expect("Failed to open file");
        file.write_all(file_contents.as_bytes())
            .expect("Failed to write");
    }
}
