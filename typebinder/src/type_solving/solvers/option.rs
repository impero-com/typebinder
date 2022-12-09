use crate::{
    contexts::exporter::ExporterContext,
    error::TsExportError,
    type_solving::fn_solver::AsFnSolver,
    type_solving::{SolverResult, TypeInfo, TypeSolver, TypeSolverExt},
    utils::inner_generic::solve_segment_generics,
};
use syn::Type;
use ts_json_subset::types::{PredefinedType, TsType, UnionType};

use super::path::PathSolver;

/// Solver for for Option<T>
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
                        Ok(solved) => {
                            if !solved.inner.is_empty() {
                                SolverResult::Solved(solved.map(|types| match types.first() {
                                    Some(ts_ty) => TsType::UnionType(UnionType {
                                        types: vec![
                                            ts_ty.clone(),
                                            TsType::PrimaryType(PredefinedType::Null.into()),
                                        ],
                                    }),
                                    None => panic!("Solved types must have at least one element"),
                                }))
                            } else {
                                SolverResult::Error(TsExportError::EmptyGenerics)
                            }
                        }
                        Err(e) => SolverResult::Error(e),
                    }
                }
                _ => unreachable!(),
            }
        })
        .fn_solver()
        .into_rc();

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
