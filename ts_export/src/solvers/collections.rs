/// Solver for :
/// * Vec<T>
/// * VecDeque<T>
/// * HashSet<T>
use crate::type_solver::{TypeInfo, TypeSolver, TypeSolvingContext};
use syn::{GenericArgument, PathArguments, Type};
use ts_json_subset::types::{ArrayType, PrimaryType, TsType};

pub struct CollectionsSolver;

impl TypeSolver for CollectionsSolver {
    fn solve_as_type(
        &self,
        solving_context: &TypeSolvingContext,
        solver_info: &TypeInfo,
    ) -> Option<ts_json_subset::types::TsType> {
        let TypeInfo { generics, ty } = solver_info;
        let ty = match ty {
            Type::Path(ty) => {
                let segment = ty.path.segments.last()?;
                let ident = segment.ident.to_string();
                match ident.as_str() {
                    "Vec" | "VecDeque" | "HashSet" => match &segment.arguments {
                        PathArguments::AngleBracketed(inner_generics) => {
                            let first_arg = inner_generics.args.first()?;
                            match first_arg {
                                GenericArgument::Type(ty) => {
                                    solving_context.solve_type(&TypeInfo { generics, ty })
                                }
                                _ => None,
                            }
                        }
                        _ => None,
                    },
                    _ => None,
                }
            }
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
