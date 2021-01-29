// Tries to find the ident of the type in the generics of the parent type

use syn::{GenericParam, Type};
use ts_json_subset::types::{PrimaryType, TsType, TypeName, TypeReference};

use crate::type_solver::{TypeInfo, TypeSolver, TypeSolvingContext};

pub struct GenericsSolver;

impl TypeSolver for GenericsSolver {
    fn solve_as_type(
        &self,
        _solving_context: &TypeSolvingContext,
        solver_info: &TypeInfo,
    ) -> Option<TsType> {
        let TypeInfo { generics, ty } = solver_info;
        let ty = match ty {
            Type::Path(ty) => {
                let segment = ty.path.segments.last()?;
                generics
                    .params
                    .iter()
                    .filter_map(|generic_param| match generic_param {
                        GenericParam::Type(ty) if ty.ident == segment.ident => Some(ty),
                        _ => None,
                    })
                    .next()
            }
            _ => None,
        }?;

        Some(TsType::PrimaryType(PrimaryType::TypeReference(
            TypeReference {
                args: None,
                name: TypeName {
                    ident: ty.ident.to_string(),
                    namespace: None,
                },
            },
        )))
    }
}
