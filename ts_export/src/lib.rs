use exporter::Exporter;
use serde_derive_internals::{ast::Container, Ctxt, Derive};
use solvers::predefined::PredefinedSolver;
use syn::{DeriveInput, Item};
use ts_json_subset::export::ExportStatement;
use type_solver::TypeSolvingContext;

use crate::error::TsExportError;
use std::{fs::File, io::Read, path::Path};

pub mod error;
pub mod exporter;
pub mod solvers;
pub mod type_solver;

use crate::solvers::array::ArraySolver;

pub fn do_it<P: AsRef<Path>>(path: P) -> Result<(), TsExportError> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let ast = syn::parse_file(&content)?;

    let derive_inputs: Vec<DeriveInput> = ast
        .items
        .into_iter()
        .filter_map(|item| match item {
            Item::Enum(item) => Some(DeriveInput::from(item)),
            Item::Struct(item) => Some(DeriveInput::from(item)),
            _ => None,
        })
        .collect();

    let ctxt = Ctxt::default();
    let containers: Vec<Container> = derive_inputs
        .iter()
        .filter_map(|derive_input| Container::from_ast(&ctxt, derive_input, Derive::Serialize))
        .collect();

    let mut solving_context = TypeSolvingContext::default();
    solving_context.add_solver(ArraySolver);
    solving_context.add_solver(PredefinedSolver);
    let exporter = Exporter { solving_context };

    let statements: Vec<ExportStatement> = containers
        .into_iter()
        .flat_map(|container| exporter.export_statements(container))
        .collect();

    for stm in statements {
        println!("{}", stm);
    }

    Ok(())
}
