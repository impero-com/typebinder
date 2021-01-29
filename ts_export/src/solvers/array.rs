/// Solver for the Array type variant
use crate::type_solver::{TypeInfo, TypeSolver, TypeSolvingContext};
use syn::Type;
use ts_json_subset::types::{ArrayType, PrimaryType, TsType};

pub struct ArraySolver;

impl TypeSolver for ArraySolver {
    fn solve_as_type(
        &self,
        solving_context: &TypeSolvingContext,
        solver_info: &TypeInfo,
    ) -> Option<ts_json_subset::types::TsType> {
        let ty = match solver_info.ty {
            Type::Array(ty) => solving_context.solve_type(&TypeInfo {
                generics: solver_info.generics,
                ty: ty.elem.as_ref(),
            }),
            Type::Slice(ty) => solving_context.solve_type(&TypeInfo {
                generics: solver_info.generics,
                ty: ty.elem.as_ref(),
            }),
            _ => None,
        }?;

        match ty {
            TsType::PrimaryType(primary) => Some(TsType::PrimaryType(PrimaryType::ArrayType(
                ArrayType::new(primary),
            ))),
            _ => None,
        }
    }
}
