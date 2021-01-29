use serde_derive_internals::{
    ast::{Container, Data, Field, Style},
    attr::TagType,
};
use syn::Generics;
use ts_json_subset::{
    declarations::{interface::InterfaceDeclaration, type_alias::TypeAliasDeclaration},
    export::ExportStatement,
    types::{ObjectType, PrimaryType, TsType, TupleType, TypeBody, TypeMember},
};

use crate::type_solver::{MemberInfo, TypeInfo, TypeSolvingContext};

pub struct Exporter {
    pub solving_context: TypeSolvingContext,
}

impl Exporter {
    pub fn export_statements(&self, container: Container) -> Vec<ExportStatement> {
        let name = container.attrs.name().serialize_name();
        match container.data {
            Data::Enum(_variants) => match container.attrs.tag() {
                TagType::External => vec![],
                TagType::Internal { tag: _tag } => vec![],
                TagType::Adjacent {
                    tag: _tag,
                    content: _content,
                } => vec![],
                TagType::None => vec![],
            },
            Data::Struct(style, fields) => match style {
                Style::Unit => vec![], // Unit structs are a no-op because they dont have a TS representation
                Style::Newtype => self.export_struct_newtype(name, container.generics, fields),
                Style::Tuple => self.export_struct_tuple(name, container.generics, fields),
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
                // TODO: Handle skip_serializing_if. Concept: mark TypeMember as optional if skip_seriazing_if is `Some`
                if field.attrs.skip_serializing() {
                    return None;
                }
                let solver_info = MemberInfo { generics, field };
                self.solving_context.solve_member(&solver_info)
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

    fn export_struct_newtype(
        &self,
        ident: String,
        generics: &Generics,
        fields: Vec<Field>,
    ) -> Vec<ExportStatement> {
        let field = &fields[0];
        let solver_info = TypeInfo {
            generics,
            ty: field.ty,
        };
        self.solving_context
            .solve_type(&solver_info)
            .map(|inner_type| {
                TypeAliasDeclaration {
                    ident,
                    inner_type,
                    params: None,
                }
                .into()
            })
            .into_iter()
            .collect()
    }

    fn export_struct_tuple(
        &self,
        ident: String,
        generics: &Generics,
        fields: Vec<Field>,
    ) -> Vec<ExportStatement> {
        let inner_types: Vec<TsType> = fields
            .into_iter()
            .filter_map(|field| {
                let solver_info = TypeInfo {
                    generics,
                    ty: field.ty,
                };
                self.solving_context.solve_type(&solver_info)
            })
            .collect();
        let inner_type = TsType::PrimaryType(PrimaryType::TupleType(TupleType { inner_types }));

        vec![TypeAliasDeclaration {
            ident,
            inner_type,
            params: None,
        }
        .into()]
    }
}
