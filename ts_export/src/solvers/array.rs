use crate::type_solver::{MemberInfo, TypeInfo, TypeSolver, TypeSolvingContext};
use syn::Type;
use ts_json_subset::types::{
    ArrayType, PrimaryType, PropertyName, PropertySignature, TsType, TypeMember,
};

pub struct ArraySolver;

impl TypeSolver for ArraySolver {
    fn solve_as_type(
        &self,
        solving_context: &TypeSolvingContext,
        solver_info: &TypeInfo,
    ) -> Option<ts_json_subset::types::TsType> {
        let ty = self.solve_inner_type(solving_context, solver_info)?;
        match ty {
            TsType::PrimaryType(primary) => Some(TsType::PrimaryType(PrimaryType::ArrayType(
                ArrayType::new(primary),
            ))),
            _ => None,
        }
    }

    fn solve_as_member(
        &self,
        solving_context: &TypeSolvingContext,
        solver_info: &MemberInfo,
    ) -> Option<ts_json_subset::types::TypeMember> {
        let ty = self.solve_inner_type(solving_context, &solver_info.as_type_info())?;
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
        solving_context: &TypeSolvingContext,
        solver_info: &TypeInfo,
    ) -> Option<TsType> {
        match solver_info.ty {
            Type::Array(ty) => solving_context.solve_type(&TypeInfo {
                generics: solver_info.generics,
                ty: ty.elem.as_ref(),
            }),
            _ => None,
        }
    }
}
