/// When serializing, serde will treat references as a no-op
use crate::type_solver::{TypeInfo, TypeSolver, TypeSolvingContext};
use syn::Type;
use ts_json_subset::types::TsType;

pub struct ReferenceSolver;

impl TypeSolver for ReferenceSolver {
    fn solve_as_type(
        &self,
        solving_context: &TypeSolvingContext,
        solver_info: &TypeInfo,
    ) -> Option<TsType> {
        let TypeInfo { generics, ty } = solver_info;
        match ty {
            Type::Reference(ty) => {
                let ty = ty.elem.as_ref();
                solving_context.solve_type(&TypeInfo { generics, ty })
            }
            _ => None,
        }
    }
}
