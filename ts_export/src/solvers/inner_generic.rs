use syn::{GenericArgument, Generics, PathArguments, PathSegment};
use ts_json_subset::types::TsType;

/// Helper to solve a type by getting an info on its generics
use crate::{
    error::TsExportError,
    exporter_context::ExporterContext,
    type_solver::{ImportEntry, TypeInfo},
};

pub fn solve_segment_generics(
    solving_context: &ExporterContext,
    generics: &Generics,
    segment: &PathSegment,
) -> Result<(Vec<TsType>, Vec<ImportEntry>), TsExportError> {
    match &segment.arguments {
        PathArguments::AngleBracketed(inner_generics) => {
            let mut imports: Vec<ImportEntry> = Vec::new();
            let inner_types = inner_generics
                .args
                .iter()
                .filter_map(|arg| match arg {
                    GenericArgument::Type(ty) => {
                        Some(solving_context.solve_type(&TypeInfo { generics, ty }))
                    }
                    _ => None,
                })
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .map(|(ts_ty, mut entries)| {
                    imports.append(&mut entries);
                    ts_ty
                })
                .collect();
            Ok((inner_types, imports))
        }
        _ => return Err(TsExportError::ExpectedGenerics),
    }
}
