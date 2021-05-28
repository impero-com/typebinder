use crate::{
    contexts::exporter::ExporterContext,
    error::TsExportError,
    type_solving::{generic_constraints::GenericConstraints, result::Solved, ImportEntry},
    type_solving::{SolverResult, TypeInfo, TypeSolver},
};
use syn::Type;
use ts_json_subset::types::{PrimaryType, TsType, TupleType};

/// A solver that solves Tuples.
/// It recurses for all the inner types
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
                    return SolverResult::Solved(Solved {
                        inner: TsType::PrimaryType(PrimaryType::Predefined(
                            ts_json_subset::types::PredefinedType::Null,
                        )),
                        import_entries: Vec::new(),
                        generic_constraints: GenericConstraints::default(),
                    });
                }

                let inner_types = ty
                    .elems
                    .iter()
                    .map(|ty| solving_context.solve_type(&TypeInfo { generics, ty }))
                    .collect::<Result<Vec<_>, _>>();
                match inner_types {
                    Ok(inner) => {
                        let mut imports: Vec<ImportEntry> = Vec::new();
                        let mut constraints = GenericConstraints::default();
                        let inner_types: Vec<TsType> = inner
                            .into_iter()
                            .map(
                                |Solved {
                                     inner,
                                     mut import_entries,
                                     generic_constraints,
                                 }| {
                                    imports.append(&mut import_entries);
                                    constraints.merge(generic_constraints);
                                    inner
                                },
                            )
                            .collect();
                        SolverResult::Solved(Solved {
                            inner: TsType::PrimaryType(PrimaryType::TupleType(TupleType {
                                inner_types,
                            })),
                            import_entries: imports,
                            generic_constraints: constraints,
                        })
                    }
                    Err(e) => SolverResult::Error(e),
                }
            }
            _ => SolverResult::Continue,
        }
    }
}
