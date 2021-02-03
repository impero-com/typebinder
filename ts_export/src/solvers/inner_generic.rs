use syn::{GenericArgument, Generics, PathArguments, PathSegment};
use ts_json_subset::types::TsType;

/// Helper to solve a type by getting an info on its generics
use crate::{error::TsExportError, exporter::ExporterContext, type_solver::TypeInfo};

pub fn solve_segment_generics(
    solving_context: &ExporterContext,
    generics: &Generics,
    segment: &PathSegment,
) -> Result<Vec<TsType>, TsExportError> {
    match &segment.arguments {
        PathArguments::AngleBracketed(inner_generics) => {
            return inner_generics
                .args
                .iter()
                .filter_map(|arg| match arg {
                    GenericArgument::Type(ty) => {
                        Some(solving_context.solve_type(&TypeInfo { generics, ty }))
                    }
                    _ => None,
                })
                .collect::<Result<_, _>>();
        }
        _ => return Err(TsExportError::ExpectedGenerics),
    }
}
