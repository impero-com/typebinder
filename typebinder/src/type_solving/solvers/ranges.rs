use super::path::PathSolver;
use crate::{
    contexts::exporter::ExporterContext,
    error::TsExportError,
    type_solving::{
        fn_solver::AsFnSolver,
        result::{Solved, SolverResult},
        type_info::TypeInfo,
        TypeSolver, TypeSolverExt,
    },
    utils::inner_generic::solve_segment_generics,
};
use syn::Type;
use ts_json_subset::types::{
    ObjectType, PrimaryType, PropertyName, PropertySignature, TsType, TypeBody, TypeMember,
};

/// A solver that solves Ranges.
pub struct RangesSolver {
    inner: PathSolver,
}

fn solve_range(
    solving_context: &ExporterContext,
    solver_info: &TypeInfo,
) -> SolverResult<TsType, TsExportError> {
    let TypeInfo { generics, ty } = solver_info;
    match ty {
        Type::Path(ty) => {
            let segment = ty.path.segments.last().expect("Empty path");
            match solve_segment_generics(solving_context, generics, segment) {
                Ok(Solved {
                    mut inner,
                    import_entries,
                    generic_constraints,
                }) => {
                    let inner_type = inner.pop().expect("first generic");
                    SolverResult::Solved(Solved {
                        inner: TsType::PrimaryType(PrimaryType::ObjectType(ObjectType {
                            body: TypeBody {
                                members: vec![
                                    TypeMember::PropertySignature(PropertySignature {
                                        name: PropertyName::from("start".to_string()),
                                        optional: false,
                                        inner_type: inner_type.clone(),
                                    }),
                                    TypeMember::PropertySignature(PropertySignature {
                                        name: PropertyName::from("end".to_string()),
                                        optional: false,
                                        inner_type,
                                    }),
                                ],
                            },
                        })),
                        import_entries,
                        generic_constraints,
                    })
                }
                Err(e) => SolverResult::Error(e),
            }
        }
        _ => SolverResult::Continue,
    }
}

impl Default for RangesSolver {
    fn default() -> Self {
        let mut inner = PathSolver::default();
        let solver_range = solve_range.fn_solver().into_rc();

        inner.add_entry("std::ops::Range".to_string(), solver_range.clone());
        inner.add_entry("std::ops::RangeInclusive".to_string(), solver_range);

        RangesSolver { inner }
    }
}

impl TypeSolver for RangesSolver {
    fn solve_as_type(
        &self,
        solving_context: &ExporterContext,
        solver_info: &TypeInfo,
    ) -> SolverResult<TsType, TsExportError> {
        self.inner.solve_as_type(solving_context, solver_info)
    }
}
