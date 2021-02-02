use syn::GenericArgument;
use thiserror::Error;
use ts_json_subset::types::TsType;

#[derive(Debug, Error)]
pub enum TsExportError {
    #[error("IO Error {0}")]
    IoError(#[from] std::io::Error),
    #[error("Syn Parse Error {0}")]
    SynError(#[from] syn::parse::Error),
    #[error("Could not resolve type {:?}", _0)]
    UnsolvedType(syn::Type),
    #[error("Could not resolve field {:?}", _0)]
    UnsolvedField(syn::Field),
    #[error("Unexpected type {:?}", _0)]
    UnexpectedType(TsType),
    #[error("Expected generics")]
    ExpectedGenerics,
    #[error("Wrong generic type {:?}", _0)]
    WrongGenericType(GenericArgument),
}
