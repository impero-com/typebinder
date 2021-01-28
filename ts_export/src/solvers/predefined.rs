use syn::Type;
use ts_json_subset::types::{
    PredefinedType, PrimaryType, PropertyName, PropertySignature, TsType, TypeMember,
};

use crate::type_solver::{SolverInfo, TypeSolver, TypeSolvingContext};

pub struct PredefinedSolver;

impl TypeSolver for PredefinedSolver {
    fn solve_newtype(
        &self,
        _solving_context: &TypeSolvingContext,
        solver_info: &SolverInfo,
    ) -> Option<ts_json_subset::types::TsType> {
        self.solve_inner_type(solver_info)
    }

    fn solve_tuple(
        &self,
        _solving_context: &TypeSolvingContext,
        solver_info: &SolverInfo,
    ) -> Option<ts_json_subset::types::TsType> {
        self.solve_inner_type(solver_info)
    }

    fn solve_struct_field(
        &self,
        _solving_context: &TypeSolvingContext,
        solver_info: &SolverInfo,
    ) -> Option<ts_json_subset::types::TypeMember> {
        let inner_type = self.solve_inner_type(solver_info)?;
        let member_name = solver_info.field.attrs.name().serialize_name();
        Some(TypeMember::PropertySignature(PropertySignature {
            inner_type,
            optional: false,
            name: PropertyName::Identifier(member_name),
        }))
    }
}

impl PredefinedSolver {
    fn solve_inner_type(&self, solver_info: &SolverInfo) -> Option<TsType> {
        match solver_info.field.ty {
            Type::Path(ty) => {
                let ty = ty.path.segments.last()?;
                let ident = ty.ident.to_string();
                match ident.as_str() {
                    "u8" => Some(PrimaryType::Predefined(PredefinedType::Number).into()),
                    "u16" => Some(PrimaryType::Predefined(PredefinedType::Number).into()),
                    "u32" => Some(PrimaryType::Predefined(PredefinedType::Number).into()),
                    "u64" => Some(PrimaryType::Predefined(PredefinedType::Number).into()),
                    "i8" => Some(PrimaryType::Predefined(PredefinedType::Number).into()),
                    "i16" => Some(PrimaryType::Predefined(PredefinedType::Number).into()),
                    "i32" => Some(PrimaryType::Predefined(PredefinedType::Number).into()),
                    "i64" => Some(PrimaryType::Predefined(PredefinedType::Number).into()),
                    "String" => Some(PrimaryType::Predefined(PredefinedType::String).into()),
                    _ => None,
                }
            }
            _ => None,
        }
    }
}
