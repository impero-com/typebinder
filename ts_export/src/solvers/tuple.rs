use crate::type_solver::{TypeInfo, TypeSolver, TypeSolvingContext};
use syn::Type;
use ts_json_subset::types::{PrimaryType, TsType, TupleType};

pub struct TupleSolver;

impl TypeSolver for TupleSolver {
    fn solve_as_type(
        &self,
        solving_context: &TypeSolvingContext,
        solver_info: &TypeInfo,
    ) -> Option<ts_json_subset::types::TsType> {
        let inner_types = self.solve_inner_types(solving_context, solver_info)?;
        Some(TsType::PrimaryType(PrimaryType::TupleType(TupleType {
            inner_types,
        })))
    }
}

impl TupleSolver {
    fn solve_inner_types(
        &self,
        solving_context: &TypeSolvingContext,
        solver_info: &TypeInfo,
    ) -> Option<Vec<TsType>> {
        let TypeInfo { generics, ty } = solver_info;
        match ty {
            Type::Tuple(ty) => Some(
                ty.elems
                    .iter()
                    .filter_map(|ty| solving_context.solve_type(&TypeInfo { generics, ty }))
                    .collect(),
            ),
            _ => None,
        }
    }
}
