use crate::type_solver::{TypeInfo, TypeSolver, TypeSolvingContext};
use syn::{GenericArgument, PathArguments, Type};
use ts_json_subset::types::{PredefinedType, TsType, UnionType};

pub struct OptionSolver;

impl TypeSolver for OptionSolver {
    fn solve_as_type(
        &self,
        solving_context: &TypeSolvingContext,
        solver_info: &TypeInfo,
    ) -> Option<TsType> {
        let TypeInfo { generics, ty } = solver_info;
        let inner_type = match ty {
            Type::Path(ty) => {
                let segment = ty.path.segments.last()?;
                let ident = segment.ident.to_string();
                match ident.as_str() {
                    "Option" => match &segment.arguments {
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

        Some(TsType::UnionType(UnionType {
            types: vec![inner_type, TsType::PrimaryType(PredefinedType::Null.into())],
        }))
    }
}
