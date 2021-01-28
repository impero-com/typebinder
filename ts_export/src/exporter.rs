use serde_derive_internals::{
    ast::{Container, Data, Field, Style},
    attr::TagType,
};
use syn::Generics;
use ts_json_subset::{
    declarations::interface::InterfaceDeclaration,
    export::ExportStatement,
    types::{ObjectType, TypeBody, TypeMember},
};

use crate::type_solver::{SolverInfo, TypeSolvingContext};

pub struct Exporter {
    pub solving_context: TypeSolvingContext,
}

impl Exporter {
    pub fn export_statements(&self, container: Container) -> Vec<ExportStatement> {
        let name = container.attrs.name().serialize_name();
        match container.data {
            Data::Enum(variants) => match container.attrs.tag() {
                TagType::External => vec![],
                TagType::Internal { tag } => vec![],
                TagType::Adjacent { tag, content } => vec![],
                TagType::None => vec![],
            },
            Data::Struct(style, fields) => match style {
                Style::Unit => vec![],
                Style::Newtype => vec![],
                Style::Tuple => vec![],
                Style::Struct => self.export_struct_struct(name, container.generics, fields),
            },
        }
    }

    fn export_struct_struct(
        &self,
        ident: String,
        generics: &Generics,
        fields: Vec<Field>,
    ) -> Vec<ExportStatement> {
        let members: Vec<TypeMember> = fields
            .into_iter()
            .filter_map(|field| {
                let solver_info = SolverInfo { generics, field };
                self.solving_context.solve_struct_field(&solver_info)
            })
            .collect();
        vec![ExportStatement::InterfaceDeclaration(
            InterfaceDeclaration {
                ident,
                extends_clause: None,
                type_params: None,
                obj_type: ObjectType {
                    body: Some(TypeBody { members }),
                },
            },
        )]
    }
}
