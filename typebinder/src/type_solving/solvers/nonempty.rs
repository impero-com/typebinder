use syn::Type;
use ts_json_subset::{
    common::StringLiteral,
    types::{
        ArrayType, IntersectionType, ObjectType, PrimaryType, PropertyName, PropertySignature,
        TsType, TypeBody, TypeMember,
    },
};

use crate::{
    contexts::exporter::ExporterContext,
    error::TsExportError,
    type_solving::{
        fn_solver::AsFnSolver, result::SolverResult, type_info::TypeInfo, TypeSolver, TypeSolverExt,
    },
    utils::inner_generic::solve_segment_generics,
};

use super::path::PathSolver;

/// Solver for NonEmpty<T>
pub struct NonEmptySolver {
    inner: PathSolver,
}

impl Default for NonEmptySolver {
    fn default() -> Self {
        let non_empty_solver = (|solving_context: &ExporterContext, solver_info: &TypeInfo| {
            let TypeInfo { generics, ty } = solver_info;
            match ty {
                Type::Path(ty) => {
                    let segment = ty.path.segments.last().expect("Empty path");
                    match solve_segment_generics(solving_context, generics, segment) {
                        Ok(solved) => {
                            if !solved.inner.is_empty() {
                                SolverResult::Solved(solved.map(|types| match types.first() {
                                    Some(ts_ty) => TsType::IntersectionType(IntersectionType {
                                        types: vec![
                                            TsType::PrimaryType(PrimaryType::ArrayType(
                                                ArrayType::new(ts_ty.clone()),
                                            )),
                                            TsType::PrimaryType(PrimaryType::ObjectType(
                                                ObjectType {
                                                    body: TypeBody {
                                                        members: vec![
                                                            TypeMember::PropertySignature(
                                                                PropertySignature {
                                                                    name:
                                                                        PropertyName::StringLiteral(
                                                                            StringLiteral::from_raw(
                                                                                "0",
                                                                            ),
                                                                        ),
                                                                    optional: false,
                                                                    inner_type: ts_ty.clone(),
                                                                },
                                                            ),
                                                        ],
                                                    },
                                                },
                                            )),
                                        ],
                                    }),
                                    None => {
                                        panic!("Solved types must have at least one element")
                                    }
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
        inner.add_entry("nonempty::NonEmpty".to_string(), non_empty_solver);
        NonEmptySolver { inner }
    }
}

impl TypeSolver for NonEmptySolver {
    fn solve_as_type(
        &self,
        solving_context: &ExporterContext,
        solver_info: &TypeInfo,
    ) -> SolverResult<TsType, TsExportError> {
        self.inner.solve_as_type(solving_context, solver_info)
    }
}
