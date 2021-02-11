use std::path::PathBuf;

use syn::Path;

use crate::{display_path::DisplayPath, process::ProcessModule};

use super::ProcessSpawner;

pub struct RustModuleReader {
    root_path: PathBuf,
}

impl Default for RustModuleReader {
    fn default() -> Self {
        let root_path = std::env::current_dir().expect("Failed to find current dir");
        RustModuleReader { root_path }
    }
}

impl RustModuleReader {
    pub fn new(path: PathBuf) -> Self {
        RustModuleReader { root_path: path }
    }
}

impl ProcessSpawner for RustModuleReader {
    fn create_process(&self, path: Path) -> Option<ProcessModule> {
        log::info!("Parsing module {}", DisplayPath(&path));
        let file_path: PathBuf = path
            .segments
            .iter()
            .map(|seg| seg.ident.to_string())
            .collect();
        let mut full_path = self.root_path.clone();
        full_path.push(file_path);
        full_path.set_extension("rs");

        let contents = std::fs::read_to_string(&full_path).expect("Failed to read module");
        let ast = syn::parse_file(&contents).expect("Failed to parse file");

        Some(ProcessModule::new(path, ast.items))
    }
}
