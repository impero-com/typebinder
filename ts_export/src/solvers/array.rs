use crate::type_solver::{SolverInfo, TypeSolver, TypeSolvingContext};
use ts_json_subset::types::{
    ArrayType, PrimaryType, PropertyName, PropertySignature, TsType, TypeMember,
};

pub struct ArraySolver;

impl TypeSolver for ArraySolver {
    fn solve_newtype(
        &self,
        solving_context: &TypeSolvingContext,
        solver_info: &SolverInfo,
    ) -> Option<ts_json_subset::types::TsType> {
        let ty = self.solve_inner_type(solving_context, solver_info)?;
        match ty {
            TsType::PrimaryType(primary) => Some(TsType::PrimaryType(PrimaryType::ArrayType(
                ArrayType::new(primary),
            ))),
            _ => None,
        }
    }

    fn solve_tuple(
        &self,
        solving_context: &TypeSolvingContext,
        solver_info: &SolverInfo,
    ) -> Option<ts_json_subset::types::TsType> {
        let ty = self.solve_inner_type(solving_context, solver_info)?;
        match ty {
            TsType::PrimaryType(primary) => Some(TsType::PrimaryType(PrimaryType::ArrayType(
                ArrayType::new(primary),
            ))),
            _ => None,
        }
    }

    fn solve_struct_field(
        &self,
        solving_context: &TypeSolvingContext,
        solver_info: &SolverInfo,
    ) -> Option<ts_json_subset::types::TypeMember> {
        let ty = self.solve_inner_type(solving_context, solver_info)?;
        match ty {
            TsType::PrimaryType(primary) => {
                let inner_type = PrimaryType::ArrayType(ArrayType::new(primary)).into();
                Some(TypeMember::PropertySignature(PropertySignature {
                    inner_type,
                    optional: false,
                    name: PropertyName::Identifier(solver_info.field.attrs.name().serialize_name()),
                }))
            }
            _ => None,
        }
    }
}

impl ArraySolver {
    pub fn solve_inner_type(
        &self,
        _solving_context: &TypeSolvingContext,
        _solver_info: &SolverInfo,
    ) -> Option<TsType> {
        /*
        let member = solver_info.field.member;
        let attrs = solver_info.field.attrs;
        let original = solver_info.field.original;
        match solver_info.field.ty {
            Type::Array(ty) => {
                let new_field = Field {
                    ty: ty.elem.as_ref(),
                    ..solver_info.field
                };
                solving_context.solve_newtype(&SolverInfo {
                    generics: solver_info.generics,
                    field: new_field,
                })
            }
            _ => None,
        }
        */
        None
    }
}
