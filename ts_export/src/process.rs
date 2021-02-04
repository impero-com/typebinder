use crate::solvers::{
    array::ArraySolver, collections::CollectionsSolver, generics::GenericsSolver,
    import::ImportSolver, option::OptionSolver, primitives::PrimitivesSolver,
    reference::ReferenceSolver, tuple::TupleSolver,
};
use crate::{error::TsExportError, import::ImportContext};
use crate::{exporter::ExporterContext, type_solver::TypeSolvingContext};
use serde_derive_internals::{ast::Container, Ctxt, Derive};
use syn::{punctuated::Punctuated, DeriveInput, File, Item, ItemType, Path};
use ts_json_subset::export::ExportStatement;

pub struct Process {
    pub content: String,
}

pub struct ProcessModule {
    current_path: Path,
    items: Vec<Item>,
    import_context: ImportContext,
}

pub struct ProcessModuleResultData {
    pub statements: Vec<ExportStatement>,
    pub path: Path,
}

pub struct ProcessModuleResult {
    pub data: ProcessModuleResultData,
    pub children: Vec<ProcessModuleResult>,
}

impl ProcessModule {
    pub fn new(current_path: syn::Path, ast: File) -> Self {
        let mut import_context = ImportContext::default();
        import_context.parse_imported(&ast);
        import_context.parse_scoped(&ast);

        ProcessModule {
            current_path,
            items: ast.items,
            import_context,
        }
    }

    pub fn launch(self) -> Result<ProcessModuleResult, TsExportError> {
        let mut derive_inputs: Vec<DeriveInput> = Vec::new();
        let mut type_aliases: Vec<ItemType> = Vec::new();

        self.items.into_iter().for_each(|item| match item {
            Item::Enum(item) => derive_inputs.push(DeriveInput::from(item)),
            Item::Struct(item) => derive_inputs.push(DeriveInput::from(item)),
            Item::Type(item) => {
                type_aliases.push(item);
            }
            // When importing a module, append current_path::module to the ImportContext
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
        solving_context.add_solver(PrimitivesSolver::default());
        solving_context.add_solver(OptionSolver::default());
        solving_context.add_solver(GenericsSolver);
        solving_context.add_solver(ImportSolver);

        let exporter = ExporterContext {
            solving_context,
            import_context: self.import_context,
        };

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

        Ok(ProcessModuleResult {
            data: ProcessModuleResultData {
                statements,
                path: self.current_path,
            },
            children: Vec::new(),
        })
    }
}

pub fn extractor(all: &mut Vec<ProcessModuleResultData>, iter: ProcessModuleResult) {
    iter.children
        .into_iter()
        .for_each(|child| extractor(all, child));
    all.push(iter.data);
}

impl Process {
    pub fn launch(&self) -> Result<String, TsExportError> {
        let ast = syn::parse_file(&self.content)?;

        let path = Path {
            leading_colon: None,
            segments: Punctuated::default(),
        };

        let res = ProcessModule::new(path, ast).launch()?;
        let mut all_statements: Vec<ProcessModuleResultData> = Vec::new();
        extractor(&mut all_statements, res);

        Ok(all_statements
            .into_iter()
            .flat_map(|statement| statement.statements)
            .map(|statement| format!("{}\n", statement))
            .collect())
    }
}
