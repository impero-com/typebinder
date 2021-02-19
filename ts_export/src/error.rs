use std::path::PathBuf;

use syn::GenericArgument;
use thiserror::Error;
use ts_json_subset::types::TsType;

#[derive(Debug, Error)]
pub enum TsExportError {
    #[error("Failed to launch the main process")]
    FailedToLaunch,
    #[error("Wrong path {:?}", _0)]
    WrongPath(PathBuf),
    #[error("Expected a Rust module, found a directory at {:?}", _0)]
    DirectoryGiven(PathBuf),
    #[error("IO Error {0}")]
    IoError(#[from] std::io::Error),
    #[error("Syn Parse Error {0}")]
    SynError(#[from] syn::parse::Error),
    #[error("JSON error {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("Could not resolve type {:?}", _0)]
    UnsolvedType(syn::Type),
    #[error("Could not resolve field {:?}", _0)]
    UnsolvedField(String),
    #[error("Unexpected TS type {:?}", _0)]
    UnexpectedType(TsType),
    #[error("Expected generics")]
    ExpectedGenerics,
    #[error("Empty generics")]
    EmptyGenerics,
    #[error("Wrong generic type {:?}", _0)]
    WrongGenericType(GenericArgument),
    #[error("Cargo.toml error {0}")]
    CargoTomlError(#[from] crate::utils::cargo::Error),
}
