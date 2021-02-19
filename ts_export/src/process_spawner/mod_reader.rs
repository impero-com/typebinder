use std::path::PathBuf;

use syn::Path;

use crate::{display_path::DisplayPath, error::TsExportError, process::ProcessModule};

use super::ProcessSpawner;

pub struct RustModuleReader {
    root_path: PathBuf,
    root_module_name: String,
    crate_name: String,
}

impl RustModuleReader {
    /// Path is the path to the root module
    pub fn try_new(path: PathBuf) -> Result<Self, TsExportError> {
        if path.is_dir() {
            return Err(TsExportError::DirectoryGiven(path));
        }
        let root_module_name = path
            .file_stem()
            .expect("Path should be a file")
            .to_string_lossy()
            .to_string();
        let root_path = path
            .canonicalize()?
            .parent()
            .ok_or_else(|| TsExportError::WrongPath(path))?
            .to_path_buf();

        // TODO: Set crate_name
        Ok(RustModuleReader {
            root_path,
            root_module_name,
            crate_name: "my_crate".to_string(),
        })
    }
}

impl ProcessSpawner for RustModuleReader {
    fn create_process(&self, path: Path) -> Option<ProcessModule> {
        log::info!("Creating process for Rust module : {}", DisplayPath(&path));
        let file_path: PathBuf = if path.segments.is_empty() {
            self.root_module_name.clone().into()
        } else {
            path.segments
                .iter()
                .map(|seg| seg.ident.to_string())
                .collect()
        };
        let mut full_path = self.root_path.clone();
        full_path.push(file_path);
        full_path.set_extension("rs");

        log::info!("Reading module from path {:?}", full_path);
        // TODO: ProcessSpawners should be fallible for cleaner error handling
        let contents = std::fs::read_to_string(&full_path).expect("Failed to read module");
        let ast = syn::parse_file(&contents).expect("Failed to parse file");

        Some(ProcessModule::new(path, ast.items, &self.crate_name))
    }
}
