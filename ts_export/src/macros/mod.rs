use syn::{Attribute, ItemMacro, Macro};
use ts_json_subset::export::ExportStatement;

use crate::{error::TsExportError, type_solver::SolverResult};

pub mod context;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MacroInfo {
    mac: Macro,
    attrs: Vec<Attribute>,
}

impl From<ItemMacro> for MacroInfo {
    fn from(item: ItemMacro) -> Self {
        MacroInfo {
            attrs: item.attrs,
            mac: item.mac,
        }
    }
}

pub trait MacroSolver {
    fn solve_macro(&self, macro_info: &MacroInfo) -> SolverResult<ExportStatement, TsExportError>;
}
