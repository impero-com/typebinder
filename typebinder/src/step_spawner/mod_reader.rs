use std::path::PathBuf;

use syn::Path;

use crate::{
    error::TsExportError, pipeline::module_step::ModuleStep, utils::display_path::DisplayPath,
};

use super::PipelineStepSpawner;

/// A strategy that reads Rust Modules from file, following the typical Rust 2018 edition module architecture
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
        let crate_name = crate::utils::cargo::fetch_crate_name_for_source_file(&path)?;
        let root_module_name = path
            .file_stem()
            .expect("Path should be a file")
            .to_string_lossy()
            .to_string();
        let root_path = path
            .canonicalize()?
            .parent()
            .ok_or(TsExportError::WrongPath(path))?
            .to_path_buf();

        Ok(RustModuleReader {
            root_path,
            root_module_name,
            crate_name,
        })
    }
}

impl PipelineStepSpawner for RustModuleReader {
    type Error = TsExportError;

    fn create_process(&self, path: Path) -> Result<Option<ModuleStep>, TsExportError> {
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

        // Case 1: <path>/file_path/mod.rs a.k.a <full_path>/mod.rs
        if full_path.is_dir() {
            let mut full_path_file = full_path.clone();
            full_path_file.push("mod");
            full_path_file.set_extension("rs");
            create_process_from_path(full_path_file, path, &self.crate_name)
        } else {
            // Case 2: <path>/file_path.rs a.k.a. <full_path>.rs
            let mut full_path_file = full_path.clone();
            full_path_file.set_extension("rs");
            create_process_from_path(full_path_file, path, &self.crate_name)
        }
    }
}

fn create_process_from_path<P: AsRef<std::path::Path> + std::fmt::Debug>(
    full_path: P,
    path: Path,
    crate_name: &str,
) -> Result<Option<ModuleStep>, TsExportError> {
    log::info!("Reading module from path {:?}", full_path);
    let contents = std::fs::read_to_string(&full_path)?;
    let ast = syn::parse_file(&contents)?;

    let process_module = ModuleStep::new(path, ast.items, crate_name);
    Ok(Some(process_module))
}
