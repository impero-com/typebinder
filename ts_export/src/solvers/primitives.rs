use crate::{
    display_path::DisplayPath,
    error::TsExportError,
    type_solver::{SolverResult, TypeInfo, TypeSolver, TypeSolvingContext},
};
use syn::Type;
use ts_json_subset::types::{PredefinedType, PrimaryType, TsType};

pub struct PrimitivesSolver;

impl TypeSolver for PrimitivesSolver {
    fn solve_as_type(
        &self,
        _solving_context: &TypeSolvingContext,
        solver_info: &TypeInfo,
    ) -> SolverResult<TsType, TsExportError> {
        match solver_info.ty {
            Type::Path(ty) => {
                let ident = DisplayPath(&ty.path).to_string();
                match ident.as_str() {
                    "u8" => {
                        SolverResult::Solved(PrimaryType::Predefined(PredefinedType::Number).into())
                    }
                    "u16" => {
                        SolverResult::Solved(PrimaryType::Predefined(PredefinedType::Number).into())
                    }
                    "u32" => {
                        SolverResult::Solved(PrimaryType::Predefined(PredefinedType::Number).into())
                    }
                    "u64" => {
                        SolverResult::Solved(PrimaryType::Predefined(PredefinedType::Number).into())
                    }
                    "usize" => {
                        SolverResult::Solved(PrimaryType::Predefined(PredefinedType::Number).into())
                    }
                    "i8" => {
                        SolverResult::Solved(PrimaryType::Predefined(PredefinedType::Number).into())
                    }
                    "i16" => {
                        SolverResult::Solved(PrimaryType::Predefined(PredefinedType::Number).into())
                    }
                    "i32" => {
                        SolverResult::Solved(PrimaryType::Predefined(PredefinedType::Number).into())
                    }
                    "i64" => {
                        SolverResult::Solved(PrimaryType::Predefined(PredefinedType::Number).into())
                    }
                    "isize" => {
                        SolverResult::Solved(PrimaryType::Predefined(PredefinedType::Number).into())
                    }
                    "f32" => {
                        SolverResult::Solved(PrimaryType::Predefined(PredefinedType::Number).into())
                    }
                    "f64" => {
                        SolverResult::Solved(PrimaryType::Predefined(PredefinedType::Number).into())
                    }
                    "String" => {
                        SolverResult::Solved(PrimaryType::Predefined(PredefinedType::String).into())
                    }
                    "str" => {
                        SolverResult::Solved(PrimaryType::Predefined(PredefinedType::String).into())
                    }
                    "char" => {
                        SolverResult::Solved(PrimaryType::Predefined(PredefinedType::String).into())
                    }
                    "bool" => SolverResult::Solved(
                        PrimaryType::Predefined(PredefinedType::Boolean).into(),
                    ),
                    _ => SolverResult::Continue,
                }
            }
            _ => SolverResult::Continue,
        }
    }
}
