use crate::error::TsExportError;
use crate::exporter::Exporter;
use crate::solvers::{
    array::ArraySolver, collections::CollectionsSolver, generics::GenericsSolver,
    option::OptionSolver, primitives::PrimitivesSolver, reference::ReferenceSolver,
    tuple::TupleSolver,
};
use crate::type_solver::TypeSolvingContext;
use serde_derive_internals::{ast::Container, Ctxt, Derive};
use syn::{DeriveInput, Item, ItemType};
use ts_json_subset::export::ExportStatement;

pub struct Process {
    pub content: String,
}

impl Process {
    pub fn launch(&self) -> Result<String, TsExportError> {
        let ast = syn::parse_file(&self.content)?;

        let mut derive_inputs: Vec<DeriveInput> = Vec::new();
        let mut type_aliases: Vec<ItemType> = Vec::new();

        ast.items.into_iter().for_each(|item| match item {
            Item::Enum(item) => derive_inputs.push(DeriveInput::from(item)),
            Item::Struct(item) => derive_inputs.push(DeriveInput::from(item)),
            Item::Type(item) => {
                type_aliases.push(item);
            }
            _ => {}
        });

        let ctxt = Ctxt::default();
        let containers: Vec<Container> = derive_inputs
            .iter()
            .filter_map(|derive_input| Container::from_ast(&ctxt, derive_input, Derive::Serialize))
            .collect();

        let mut solving_context = TypeSolvingContext::default();
        solving_context.add_solver(TupleSolver);
        solving_context.add_solver(ReferenceSolver);
        solving_context.add_solver(ArraySolver);
        solving_context.add_solver(CollectionsSolver);
        solving_context.add_solver(PrimitivesSolver);
        solving_context.add_solver(OptionSolver);
        solving_context.add_solver(GenericsSolver);
        let exporter = Exporter { solving_context };

        let type_export_statements = type_aliases
            .into_iter()
            .map(|item| exporter.export_statements_from_type_alias(item));
        let container_statements = containers
            .into_iter()
            .map(|container| exporter.export_statements_from_container(container));

        let statements: Vec<ExportStatement> = type_export_statements
            .chain(container_statements)
            .collect::<Result<Vec<Vec<_>>, _>>()?
            .into_iter()
            .flat_map(|x| x)
            .collect();

        Ok(statements
            .into_iter()
            .map(|statement| format!("{}\n", statement))
            .collect())
    }
}
