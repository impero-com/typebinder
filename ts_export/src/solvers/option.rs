use crate::{
    error::TsExportError,
    exporter_context::ExporterContext,
    solvers::fn_solver::AsFnSolver,
    type_solver::{SolverResult, TypeInfo, TypeSolver, TypeSolverExt},
};
use syn::Type;
use ts_json_subset::types::{PredefinedType, TsType, UnionType};

use super::{inner_generic::solve_segment_generics, path::PathSolver};

pub struct OptionSolver {
    inner: PathSolver,
}

impl Default for OptionSolver {
    fn default() -> Self {
        let option_solver = (|solving_context: &ExporterContext, solver_info: &TypeInfo| {
            let TypeInfo { generics, ty } = solver_info;
            match ty {
                Type::Path(ty) => {
                    let segment = ty.path.segments.last().expect("Empty path");
                    match solve_segment_generics(solving_context, generics, segment) {
                        Ok((types, entries)) => match types.first() {
                            Some(ts_ty) => {
                                return SolverResult::Solved(
                                    TsType::UnionType(UnionType {
                                        types: vec![
                                            ts_ty.clone(),
                                            TsType::PrimaryType(PredefinedType::Null.into()),
                                        ],
                                    }),
                                    entries,
                                )
                            }
                            None => return SolverResult::Error(TsExportError::EmptyGenerics),
                        },
                        Err(e) => return SolverResult::Error(e),
                    }
                }
                _ => unreachable!(),
            }
        })
        .as_fn_solver()
        .as_rc();

        let mut inner = PathSolver::default();
        inner.add_entry("std::option::Option".to_string(), option_solver);
        OptionSolver { inner }
    }
}

impl TypeSolver for OptionSolver {
    fn solve_as_type(
        &self,
        solving_context: &ExporterContext,
        solver_info: &TypeInfo,
    ) -> SolverResult<TsType, TsExportError> {
        self.inner.solve_as_type(solving_context, solver_info)
    }
}
