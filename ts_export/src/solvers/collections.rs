/// Solver for :
/// * Vec<T>
/// * VecDeque<T>
/// * HashSet<T>
use crate::type_solver::{TypeInfo, TypeSolver, TypeSolvingContext};
use syn::{GenericArgument, PathArguments, Type};
use ts_json_subset::types::{
    ArrayType, PrimaryType, PropertyName, PropertySignature, TsType, TypeMember,
};

pub struct CollectionsSolver;

impl TypeSolver for CollectionsSolver {
    fn solve_as_type(
        &self,
        solving_context: &crate::type_solver::TypeSolvingContext,
        solver_info: &crate::type_solver::TypeInfo,
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
        solving_context: &crate::type_solver::TypeSolvingContext,
        solver_info: &crate::type_solver::MemberInfo,
    ) -> Option<TypeMember> {
        let ty = self.solve_inner_type(solving_context, &solver_info.as_type_info())?;
        match ty {
            TsType::PrimaryType(primary) => {
                let member_name = solver_info.field.attrs.name().serialize_name();
                let inner_type = PrimaryType::ArrayType(ArrayType::new(primary)).into();
                Some(TypeMember::PropertySignature(PropertySignature {
                    inner_type,
                    optional: false,
                    name: PropertyName::Identifier(member_name),
                }))
            }
            _ => None,
        }
    }
}

impl CollectionsSolver {
    pub fn solve_inner_type(
        &self,
        solving_context: &TypeSolvingContext,
        solver_info: &TypeInfo,
    ) -> Option<TsType> {
        let TypeInfo { generics, ty } = solver_info;
        match ty {
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
        }
    }
}
