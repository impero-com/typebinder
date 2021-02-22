use std::collections::{HashMap, HashSet};

use crate::{
    error::TsExportError, exporters::Exporter, import::ImportContext, path_mapper::PathMapper,
    process_spawner::ProcessSpawner, type_solver::ImportEntry,
};
use crate::{exporter_context::ExporterContext, type_solver::TypeSolvingContext};
use result::prelude::*;
use serde_derive_internals::{ast::Container, Ctxt, Derive};
use syn::{
    punctuated::Punctuated, DeriveInput, Item, ItemMod, ItemType, Path, PathArguments, PathSegment,
};
use ts_json_subset::{
    export::ExportStatement,
    import::{ImportKind, ImportList, ImportStatement},
};

// TODO: Rename. This is not a process, system-wise
// Pipeline ?
pub struct Process<PS, E> {
    pub process_spawner: PS,
    pub exporter: E,
    pub path_mapper: PathMapper,
}

pub struct ProcessModule {
    current_path: Path,
    items: Vec<Item>,
    import_context: ImportContext,
}

pub struct ProcessModuleResultData {
    pub exports: Vec<ExportStatement>,
    pub imports: Vec<ImportStatement>,
    pub path: Path,
}

pub struct ProcessModuleResult {
    pub data: ProcessModuleResultData,
    pub children: Vec<ProcessModuleResult>,
}

impl ProcessModule {
    pub fn new(current_path: syn::Path, items: Vec<Item>, crate_name: &str) -> Self {
        let mut import_context = ImportContext::default();
        import_context.parse_imported(&items, crate_name);
        import_context.parse_scoped(&items);

        ProcessModule {
            current_path,
            items,
            import_context,
        }
    }

    pub fn launch<PS: ProcessSpawner>(
        self,
        process_spawner: &PS,
        solving_context: &TypeSolvingContext,
        path_mapper: &PathMapper,
    ) -> Result<ProcessModuleResult, TsExportError> {
        let ProcessModule {
            current_path,
            import_context,
            items,
        } = self;

        let mut derive_inputs: Vec<(usize, DeriveInput)> = Vec::new();
        let mut type_aliases: Vec<(usize, ItemType)> = Vec::new();
        let mut mod_declarations: Vec<ItemMod> = Vec::new();

        items
            .into_iter()
            .enumerate()
            .for_each(|(index, item)| match item {
                Item::Enum(item) => derive_inputs.push((index, DeriveInput::from(item))),
                Item::Struct(item) => derive_inputs.push((index, DeriveInput::from(item))),
                Item::Type(item) => {
                    type_aliases.push((index, item));
                }
                Item::Mod(item) => {
                    mod_declarations.push(item);
                }
                _ => {}
            });

        let children: Vec<ProcessModuleResult> = mod_declarations
            .into_iter()
            .filter_map(|item_mod| {
                let ident = item_mod.ident;
                let mut path = current_path.clone();
                path.segments.push(PathSegment {
                    ident,
                    arguments: PathArguments::None,
                });
                match item_mod.content {
                    Some((_, items)) => Some(Ok(ProcessModule::new(path, items, "crate"))),
                    _ => process_spawner
                        .create_process(path)
                        .map_err(|e| e.into())
                        .invert(),
                }
            })
            .map(|process_module_result| {
                process_module_result.and_then(|process_module| {
                    process_module.launch(process_spawner, solving_context, path_mapper)
                })
            })
            .collect::<Result<_, _>>()?;

        let ctxt = Ctxt::default();
        let containers: Vec<(usize, Container)> = derive_inputs
            .iter()
            .filter_map(|(index, derive_input)| {
                Container::from_ast(&ctxt, &derive_input, Derive::Serialize)
                    .map(|container| (*index, container))
            })
            .collect();

        let exporter = ExporterContext {
            solving_context: &solving_context,
            import_context,
        };

        let type_export_statements = type_aliases.into_iter().map(|(index, item)| {
            exporter
                .export_statements_from_type_alias(item)
                .map(|statements| (index, statements))
        });
        let container_statements = containers.into_iter().map(|(index, container)| {
            exporter
                .export_statements_from_container(container)
                .map(|statements| (index, statements))
        });

        let mut imports: Vec<ImportEntry> = Vec::new();

        let mut statements: Vec<(usize, Vec<ExportStatement>)> = type_export_statements
            .chain(container_statements)
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|(index, (exports, mut entries))| {
                imports.append(&mut entries);
                (index, exports)
            })
            .collect();

        let mut all_imports: HashMap<String, HashSet<String>> = HashMap::default();
        imports.into_iter().for_each(|entry| {
            let hm_entry = all_imports.entry(entry.path).or_default();
            hm_entry.insert(entry.ident);
        });

        let imports: Vec<ImportStatement> = all_imports
            .into_iter()
            .filter_map(|(path, items)| {
                let items: Vec<String> = items.into_iter().collect();
                let path = path_mapper.map(&path).unwrap_or(path);
                if path.is_empty() {
                    None
                } else {
                    Some(ImportStatement {
                        path,
                        import_kind: ImportKind::ImportList(ImportList { items }),
                    })
                }
            })
            .collect();

        statements.sort_by_key(|(index, _)| *index);

        let exports: Vec<ExportStatement> = statements
            .into_iter()
            .flat_map(|(_, statements)| statements.into_iter())
            .collect();

        Ok(ProcessModuleResult {
            data: ProcessModuleResultData {
                exports,
                imports,
                path: current_path,
            },
            children,
        })
    }
}

fn extractor(all: &mut Vec<ProcessModuleResultData>, iter: ProcessModuleResult) {
    iter.children
        .into_iter()
        .for_each(|child| extractor(all, child));
    all.push(iter.data);
}

impl<PS, E> Process<PS, E>
where
    PS: ProcessSpawner,
    E: Exporter,
    TsExportError: From<PS::Error> + From<E::Error>,
{
    pub fn launch(&self, solving_context: &TypeSolvingContext) -> Result<(), TsExportError> {
        let path = Path {
            leading_colon: None,
            segments: Punctuated::default(),
        };

        let res = self
            .process_spawner
            .create_process(path)?
            .ok_or(TsExportError::FailedToLaunch)?
            .launch(&self.process_spawner, solving_context, &self.path_mapper)?;
        let mut all_results: Vec<ProcessModuleResultData> = Vec::new();
        extractor(&mut all_results, res);

        for result_data in all_results.into_iter() {
            if result_data.imports.is_empty() && result_data.exports.is_empty() {
                return Ok(());
            }
            self.exporter.export_module(result_data)?;
        }

        Ok(())
    }
}
