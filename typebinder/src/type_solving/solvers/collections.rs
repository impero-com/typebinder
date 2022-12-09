use std::str::FromStr;

use crate::{
    contexts::exporter::ExporterContext,
    error::TsExportError,
    type_solving::{fn_solver::AsFnSolver, result::Solved},
    type_solving::{SolverResult, TypeInfo, TypeSolver, TypeSolverExt},
    utils::inner_generic::solve_segment_generics,
};
use syn::Type;
use ts_json_subset::{
    ident::{StrictTSIdent, TSIdent},
    types::{ArrayType, PredefinedType, PrimaryType, TsType, TypeArguments, TypeReference},
};

use super::path::PathSolver;

/// Solves all collections (vectors, arrays, trees, maps) from the standard library
/// The list of types supported by this solver is from serde's implementation.
/// See serde's `seq_impl!` and `map_impl!`.
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
                Ok(Solved {
                    inner,
                    import_entries,
                    generic_constraints,
                }) => SolverResult::Solved(Solved {
                    inner: TsType::PrimaryType(PrimaryType::ArrayType(ArrayType::new(
                        inner[0].clone(),
                    ))),
                    import_entries,
                    generic_constraints,
                }),
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
                Ok(solved) => {
                    let first = solved.inner[0].clone();
                    let mut solved = match first {
                        // Special case where the key of the map is a string or a number
                        // output is Record<K, V>
                        TsType::PrimaryType(PrimaryType::Predefined(
                            PredefinedType::String | PredefinedType::Number,
                        )) => solved.map(|inner| {
                            TsType::PrimaryType(PrimaryType::TypeReference(TypeReference {
                                name: StrictTSIdent::from_str("Record").unwrap(),
                                args: Some(TypeArguments {
                                    types: vec![inner[0].clone(), inner[1].clone()],
                                }),
                            }))
                        }),
                        // For other types, the correct representation is a Partial<Record<K, V>>
                        _ => solved.map(|inner| {
                            let record =
                                TsType::PrimaryType(PrimaryType::TypeReference(TypeReference {
                                    name: StrictTSIdent::from_str("Record").unwrap(),
                                    args: Some(TypeArguments {
                                        types: vec![inner[0].clone(), inner[1].clone()],
                                    }),
                                }));
                            TsType::PrimaryType(PrimaryType::TypeReference(TypeReference {
                                name: StrictTSIdent::from_str("Partial").unwrap(),
                                args: Some(TypeArguments {
                                    types: vec![record],
                                }),
                            }))
                        }),
                    };
                    solved.generic_constraints.add_extends_constraint(
                        TSIdent::from_str(&format!("{}", first)).unwrap(),
                        TsType::PrimaryType(PrimaryType::Predefined(PredefinedType::String)),
                    );
                    SolverResult::Solved(solved)
                }
                Err(e) => SolverResult::Error(e),
            }
        }
        _ => SolverResult::Continue,
    }
}

impl Default for CollectionsSolver {
    fn default() -> Self {
        let mut inner = PathSolver::default();
        let solver_seq = solve_seq.fn_solver().into_rc();
        let solver_map = solve_map.fn_solver().into_rc();

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
