use crate::{
    error::TsExportError,
    type_solver::{SolverResult, TypeInfo, TypeSolver, TypeSolvingContext},
};
use syn::{GenericArgument, PathArguments, Type};
use ts_json_subset::types::{PredefinedType, TsType, UnionType};

pub struct OptionSolver;

impl TypeSolver for OptionSolver {
    fn solve_as_type(
        &self,
        solving_context: &TypeSolvingContext,
        solver_info: &TypeInfo,
    ) -> SolverResult<TsType, TsExportError> {
        let TypeInfo { generics, ty } = solver_info;
        let inner_type = match ty {
            Type::Path(ty) => {
                let segment = ty.path.segments.last().expect("Empty path");
                let ident = segment.ident.to_string();
                match ident.as_str() {
                    "Option" => match &segment.arguments {
                        PathArguments::AngleBracketed(inner_generics) => {
                            if let Some(first_arg) = inner_generics.args.first() {
                                match first_arg {
                                    GenericArgument::Type(ty) => {
                                        solving_context.solve_type(&TypeInfo { generics, ty })
                                    }
                                    _ => {
                                        return SolverResult::Error(
                                            TsExportError::WrongGenericType(first_arg.clone()),
                                        )
                                    }
                                }
                            } else {
                                return SolverResult::Error(TsExportError::ExpectedGenerics);
                            }
                        }
                        _ => return SolverResult::Error(TsExportError::ExpectedGenerics),
                    },
                    _ => return SolverResult::Continue,
                }
            }
            _ => return SolverResult::Continue,
        };

        match inner_type {
            Ok(ts_ty) => SolverResult::Solved(TsType::UnionType(UnionType {
                types: vec![ts_ty, TsType::PrimaryType(PredefinedType::Null.into())],
            })),
            Err(e) => SolverResult::Error(e),
        }
    }
}
