use syn::Type;
use ts_json_subset::types::{PropertyName, PropertySignature, TsType, TypeMember};

use crate::{
    contexts::exporter::ExporterContext,
    error::TsExportError,
    type_solving::member_info::MemberInfo,
    type_solving::{result::SolverResult, type_info::TypeInfo, TypeSolver},
    utils::display_path::DisplayPath,
    utils::inner_generic::solve_segment_generics,
};

/// A solver to handle `serde(skip_serialize_if = "...")`
pub struct SkipSerializeIf;

impl TypeSolver for SkipSerializeIf {
    fn solve_as_type(
        &self,
        _solving_context: &ExporterContext,
        _solver_info: &TypeInfo,
    ) -> SolverResult<TsType, TsExportError> {
        SolverResult::Continue
    }

    fn solve_as_member(
        &self,
        solving_context: &ExporterContext,
        solver_info: &MemberInfo,
    ) -> SolverResult<TypeMember, TsExportError> {
        if let Some(skip_serializing_if) = solver_info.serde_field.skip_serializing_if() {
            if let Type::Path(ty_path) = solver_info.ty {
                let ty_name = DisplayPath(&ty_path.path).to_string();
                match ty_name.as_str() {
                    "Option" => {
                        let skip_serializing_if =
                            DisplayPath(&skip_serializing_if.path).to_string();
                        match skip_serializing_if.as_str() {
                            "Option::is_none" => {
                                // Special case: the type is Option and skip_serialize_if's function is Option::is_none
                                // Solution: inner type of Option, field as optional
                                let generics = solver_info.generics;
                                let segment = ty_path.path.segments.last().expect("Empty path");
                                match solve_segment_generics(solving_context, generics, segment) {
                                    Ok((types, imports)) => {
                                        let inner_type = types[0].clone();
                                        let member =
                                            TypeMember::PropertySignature(PropertySignature {
                                                inner_type,
                                                name: PropertyName::from(
                                                    solver_info.name.to_string(),
                                                ),
                                                optional: true,
                                            });
                                        return SolverResult::Solved(member, imports);
                                    }
                                    Err(e) => return SolverResult::Error(e),
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            // General case the type is not an Option
            let type_info = solver_info.as_type_info();
            match solving_context.solve_type(&type_info).into() {
                SolverResult::Solved(inner_type, imports) => {
                    let member = TypeMember::PropertySignature(PropertySignature {
                        inner_type,
                        name: PropertyName::from(solver_info.name.to_string()),
                        optional: true,
                    });
                    return SolverResult::Solved(member, imports);
                }
                SolverResult::Error(e) => return SolverResult::Error(e),
                SolverResult::Continue => return SolverResult::Continue,
            }
        }
        SolverResult::Continue
    }
}
