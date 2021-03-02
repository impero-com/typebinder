use syn::{Attribute, ItemMacro, Macro};
use ts_json_subset::export::ExportStatement;

use crate::{error::TsExportError, type_solving::result::SolverResult};

pub mod context;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MacroInfo {
    pub mac: Macro,
    pub attrs: Vec<Attribute>,
}

impl From<ItemMacro> for MacroInfo {
    fn from(item: ItemMacro) -> Self {
        MacroInfo {
            attrs: item.attrs,
            mac: item.mac,
        }
    }
}

/// The MacroSolver is an abstraction that generates ExportStatements from a macro invocation.
/// It is meant as a placeholder while we figure out a proper way to expand the macro invocations.
pub trait MacroSolver {
    fn solve_macro(&self, macro_info: &MacroInfo) -> SolverResult<ExportStatement, TsExportError>;
}
