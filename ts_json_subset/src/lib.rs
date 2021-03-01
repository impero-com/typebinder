/// Implementation of a subset of the TypeScript language AST.
/// The only purpose of this crate is to output valid
/// * interfaces declarations,
/// * type aliases declarations,
/// * const enums declarations
///
/// This subset allows to represent types in TypeScript that can get deserialized from JSON.
///
/// Follows TypeScript grammar as defined in http://javascript.xgqfrms.xyz/pdfs/TypeScript%20Language%20Specification.pdf
///
/// Display implementation are provided by either :
/// * Askama when the implied logic is complex,
/// * Displaythis when we have an enum variant with an inner type that implements Display
///
pub mod common;
pub mod declarations;
pub mod export;
pub mod import;
pub mod types;
