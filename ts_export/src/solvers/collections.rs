/// Solver for :
/// * Vec<T>
/// * VecDeque<T>
/// * HashSet<T>
use crate::{
    error::TsExportError,
    exporter_context::ExporterContext,
    type_solver::{SolverResult, TypeInfo, TypeSolver, TypeSolverExt},
};
use syn::Type;
use ts_json_subset::types::{
    ArrayType, PrimaryType, TsType, TypeArguments, TypeName, TypeReference,
};

use super::{fn_solver::AsFnSolver, inner_generic::solve_segment_generics, path::PathSolver};

pub struct CollectionsSolver {
    inner: PathSolver,
}

fn solve_seq(
    solving_context: &ExporterContext,
    solver_info: &TypeInfo,
) -> SolverResult<TsType, TsExportError> {
    let TypeInfo { generics, ty } = solver_info;
    match ty {
        Type::Path(ty) => {
            let segment = ty.path.segments.last().expect("Empty path");
            match solve_segment_generics(solving_context, generics, segment) {
                Ok(types) => match &types[0] {
                    TsType::PrimaryType(prim) => SolverResult::Solved(TsType::PrimaryType(
                        PrimaryType::ArrayType(ArrayType::new(prim.clone())),
                    )),
                    _ => SolverResult::Error(TsExportError::UnexpectedType(types[0].clone())),
                },
                Err(e) => SolverResult::Error(e),
            }
        }
        _ => SolverResult::Continue,
    }
}

fn solve_map(
    solving_context: &ExporterContext,
    solver_info: &TypeInfo,
) -> SolverResult<TsType, TsExportError> {
    let TypeInfo { generics, ty } = solver_info;
    match ty {
        Type::Path(ty) => {
            let segment = ty.path.segments.last().expect("Empty path");
            match solve_segment_generics(solving_context, generics, segment) {
                Ok(types) => match (&types[0], &types[1]) {
                    (TsType::PrimaryType(key), TsType::PrimaryType(value)) => SolverResult::Solved(
                        TsType::PrimaryType(PrimaryType::TypeReference(TypeReference {
                            name: TypeName {
                                namespace: None,
                                ident: "Record".to_string(),
                            },
                            args: Some(TypeArguments {
                                types: vec![key.clone().into(), value.clone().into()],
                            }),
                        })),
                    ),
                    _ => SolverResult::Error(TsExportError::UnexpectedType(types[0].clone())),
                },
                Err(e) => SolverResult::Error(e),
            }
        }
        _ => SolverResult::Continue,
    }
}

impl Default for CollectionsSolver {
    fn default() -> Self {
        let mut inner = PathSolver::default();
        let solver_seq = solve_seq.as_fn_solver().as_rc();
        let solver_map = solve_map.as_fn_solver().as_rc();

        inner.add_entry("std::vec::Vec".to_string(), solver_seq.clone());
        inner.add_entry("std::collections::VecDeque".to_string(), solver_seq.clone());
        inner.add_entry("std::collections::HashSet".to_string(), solver_seq.clone());
        inner.add_entry(
            "std::collections::LinkedList".to_string(),
            solver_seq.clone(),
        );
        inner.add_entry("std::collections::BTreeSet".to_string(), solver_seq.clone());
        inner.add_entry("std::collections::BinaryHeap".to_string(), solver_seq);
        inner.add_entry("std::collections::HashMap".to_string(), solver_map.clone());
        inner.add_entry("std::collections::BTreeMap".to_string(), solver_map);

        CollectionsSolver { inner }
    }
}

impl TypeSolver for CollectionsSolver {
    fn solve_as_type(
        &self,
        solving_context: &ExporterContext,
        solver_info: &TypeInfo,
    ) -> SolverResult<TsType, TsExportError> {
        self.inner.solve_as_type(solving_context, solver_info)
    }
}
