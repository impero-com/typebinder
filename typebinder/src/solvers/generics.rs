// Tries to find the ident of the type in the generics of the parent type

use syn::{GenericParam, Type};
use ts_json_subset::types::{PrimaryType, TsType, TypeName, TypeReference};

use crate::{
    error::TsExportError,
    exporter_context::ExporterContext,
    type_solver::{SolverResult, TypeInfo, TypeSolver},
};

pub struct GenericsSolver;

impl TypeSolver for GenericsSolver {
    fn solve_as_type(
        &self,
        _solving_context: &ExporterContext,
        solver_info: &TypeInfo,
    ) -> SolverResult<TsType, TsExportError> {
        let TypeInfo { generics, ty } = solver_info;
        let ty = match ty {
            Type::Path(ty) => {
                // Probably needs an explicit error
                let segment = ty.path.segments.last().expect("Empty path");
                generics
                    .params
                    .iter()
                    .filter_map(|generic_param| match generic_param {
                        GenericParam::Type(ty) if ty.ident == segment.ident => Some(ty),
                        _ => None,
                    })
                    .next()
            }
            _ => {
                return SolverResult::Continue;
            }
        };

        match ty {
            Some(ty) => SolverResult::Solved(
                TsType::PrimaryType(PrimaryType::TypeReference(TypeReference {
                    args: None,
                    name: TypeName {
                        ident: ty.ident.to_string(),
                        namespace: None,
                    },
                })),
                Vec::new(),
            ),
            _ => SolverResult::Continue,
        }
    }
}
