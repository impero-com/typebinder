use crate::{
    contexts::exporter::ExporterContext,
    error::TsExportError,
    type_solving::ImportEntry,
    type_solving::{SolverResult, TypeInfo, TypeSolver},
};
use syn::Type;
use ts_json_subset::types::{PrimaryType, TsType, TupleType};

pub struct TupleSolver;

impl TypeSolver for TupleSolver {
    fn solve_as_type(
        &self,
        solving_context: &ExporterContext,
        solver_info: &TypeInfo,
    ) -> SolverResult<TsType, TsExportError> {
        let TypeInfo { generics, ty } = solver_info;
        match ty {
            Type::Tuple(ty) => {
                // Empty tuples are unit types, "()". Those get serialized as null.
                if ty.elems.is_empty() {
                    return SolverResult::Solved(
                        TsType::PrimaryType(PrimaryType::Predefined(
                            ts_json_subset::types::PredefinedType::Null,
                        )),
                        Vec::new(),
                    );
                }

                let inner_types = ty
                    .elems
                    .iter()
                    .map(|ty| solving_context.solve_type(&TypeInfo { generics, ty }))
                    .collect::<Result<Vec<_>, _>>();
                match inner_types {
                    Ok(inner) => {
                        let mut imports: Vec<ImportEntry> = Vec::new();
                        let inner_types: Vec<TsType> = inner
                            .into_iter()
                            .map(|(ty, mut entries)| {
                                imports.append(&mut entries);
                                ty
                            })
                            .collect();
                        SolverResult::Solved(
                            TsType::PrimaryType(PrimaryType::TupleType(TupleType { inner_types })),
                            imports,
                        )
                    }
                    Err(e) => SolverResult::Error(e),
                }
            }
            _ => SolverResult::Continue,
        }
    }
}
