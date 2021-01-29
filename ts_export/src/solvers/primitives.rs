use crate::type_solver::{TypeInfo, TypeSolver, TypeSolvingContext};
use syn::Type;
use ts_json_subset::types::{PredefinedType, PrimaryType};

pub struct PrimitivesSolver;

impl TypeSolver for PrimitivesSolver {
    fn solve_as_type(
        &self,
        _solving_context: &TypeSolvingContext,
        solver_info: &TypeInfo,
    ) -> Option<ts_json_subset::types::TsType> {
        match solver_info.ty {
            Type::Path(ty) => {
                let ty = ty.path.segments.last()?;
                let ident = ty.ident.to_string();
                match ident.as_str() {
                    "u8" => Some(PrimaryType::Predefined(PredefinedType::Number).into()),
                    "u16" => Some(PrimaryType::Predefined(PredefinedType::Number).into()),
                    "u32" => Some(PrimaryType::Predefined(PredefinedType::Number).into()),
                    "u64" => Some(PrimaryType::Predefined(PredefinedType::Number).into()),
                    "usize" => Some(PrimaryType::Predefined(PredefinedType::Number).into()),
                    "i8" => Some(PrimaryType::Predefined(PredefinedType::Number).into()),
                    "i16" => Some(PrimaryType::Predefined(PredefinedType::Number).into()),
                    "i32" => Some(PrimaryType::Predefined(PredefinedType::Number).into()),
                    "i64" => Some(PrimaryType::Predefined(PredefinedType::Number).into()),
                    "isize" => Some(PrimaryType::Predefined(PredefinedType::Number).into()),
                    "f32" => Some(PrimaryType::Predefined(PredefinedType::Number).into()),
                    "f64" => Some(PrimaryType::Predefined(PredefinedType::Number).into()),
                    "String" => Some(PrimaryType::Predefined(PredefinedType::String).into()),
                    "str" => Some(PrimaryType::Predefined(PredefinedType::String).into()),
                    "char" => Some(PrimaryType::Predefined(PredefinedType::String).into()),
                    "bool" => Some(PrimaryType::Predefined(PredefinedType::Boolean).into()),
                    _ => None,
                }
            }
            _ => None,
        }
    }
}
