use crate::{
    error::TsExportError,
    type_solver::{SolverResult, TypeInfo, TypeSolver, TypeSolvingContext},
};
use syn::Type;
use ts_json_subset::types::{PrimaryType, TsType, TupleType};

pub struct TupleSolver;

impl TypeSolver for TupleSolver {
    fn solve_as_type(
        &self,
        solving_context: &TypeSolvingContext,
        solver_info: &TypeInfo,
    ) -> SolverResult<TsType, TsExportError> {
        let TypeInfo { generics, ty } = solver_info;
        match ty {
            Type::Tuple(ty) => {
                // Empty tuples are unit types, "()". Those get serialized as null.
                if ty.elems.is_empty() {
                    return SolverResult::Solved(TsType::PrimaryType(PrimaryType::Predefined(
                        ts_json_subset::types::PredefinedType::Null,
                    )));
                }

                let inner_types = ty
                    .elems
                    .iter()
                    .map(|ty| solving_context.solve_type(&TypeInfo { generics, ty }))
                    .collect::<Result<_, _>>();
                match inner_types {
                    Ok(inner_types) => SolverResult::Solved(TsType::PrimaryType(
                        PrimaryType::TupleType(TupleType { inner_types }),
                    )),
                    Err(e) => SolverResult::Error(e),
                }
            }
            _ => SolverResult::Continue,
        }
    }
}
