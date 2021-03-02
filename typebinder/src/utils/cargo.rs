/// Various utilities to get the name of a crate from a path
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to canonicalize path {0} : {1}")]
    PathCanonicalizeError(PathBuf, std::io::Error),
    #[error("Cargo.toml file can not be found source file {0}")]
    CargoTomlNotFound(PathBuf),
    #[error("Cannot read Cargo.toml: {0}")]
    CargoTomlReadFailed(#[from] cargo_toml::Error),
    #[error("package section not found in {0}")]
    NoPackageSection(PathBuf),
}

pub fn fetch_crate_name_for_source_file(source_file_path: &Path) -> Result<String, Error> {
    let cargo_toml_path = find_cargo_toml_for_source_file(source_file_path)?;
    let manifest = cargo_toml::Manifest::from_path(&cargo_toml_path)?;
    let package = manifest
        .package
        .ok_or(Error::NoPackageSection(cargo_toml_path))?;
    Ok(package.name)
}

fn find_cargo_toml_for_source_file(source_file_path: &Path) -> Result<PathBuf, Error> {
    let path = source_file_path
        .canonicalize()
        .map_err(|io_err| Error::PathCanonicalizeError(source_file_path.to_path_buf(), io_err))?;
    let mut opt_dir = path.parent();

    while let Some(dir) = opt_dir {
        let cargo_toml_path = dir.join("Cargo.toml");
        if cargo_toml_path.is_file() {
            return Ok(cargo_toml_path);
        }
        opt_dir = dir.parent();
    }

    Err(Error::CargoTomlNotFound(source_file_path.to_path_buf()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_crate_name_for_source_file() {
        let current_file = format!("../{}", file!());
        let path = PathBuf::from(current_file);
        let crate_name = fetch_crate_name_for_source_file(&path).unwrap();
        assert_eq!(crate_name, "typebinder")
    }
}
