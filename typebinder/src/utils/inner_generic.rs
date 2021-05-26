/// Helper to solve a type by getting an info on its generics
///
use syn::{GenericArgument, Generics, PathArguments, PathSegment};
use ts_json_subset::types::TsType;

use crate::{
    contexts::exporter::ExporterContext,
    error::TsExportError,
    type_solving::{
        generic_constraints::GenericConstraints, result::Solved, type_info::TypeInfo, ImportEntry,
    },
};

/// Helper that solves all the generics at the end of a segment.
///
/// e.g. : MyType<u32, String>
/// Will return a vector containing [ number, string ], and the associated ImportEntry
pub fn solve_segment_generics(
    solving_context: &ExporterContext,
    generics: &Generics,
    segment: &PathSegment,
) -> Result<Solved<Vec<TsType>>, TsExportError> {
    match &segment.arguments {
        PathArguments::AngleBracketed(inner_generics) => {
            let mut imports: Vec<ImportEntry> = Vec::new();
            let mut constraints = GenericConstraints::default();
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
                .map(
                    |Solved {
                         inner: ts_ty,
                         import_entries: mut entries,
                         generic_constraints,
                     }| {
                        imports.append(&mut entries);
                        constraints.merge(generic_constraints);
                        ts_ty
                    },
                )
                .collect();
            Ok(Solved {
                inner: inner_types,
                import_entries: imports,
                generic_constraints: constraints,
            })
        }
        _ => Err(TsExportError::ExpectedGenerics),
    }
}
