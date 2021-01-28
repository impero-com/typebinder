use thiserror::Error;

#[derive(Debug, Error)]
pub enum TsExportError {
    #[error("IO Error {0}")]
    IoError(#[from] std::io::Error),
    #[error("Syn Parse Error {0}")]
    SynError(#[from] syn::parse::Error),
}
