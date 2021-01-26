mod common;
mod declarations;
mod export;
mod import;
pub mod types;

/*
#[derive(Debug, Clone, PartialEq, Eq, Template)]
#[template(
    source = r#"interface {{ ident }}
    {% match parameters %}
        {% when Some with (params) %}
            Some
        {% when None %}
            None
    {% endmatch %}
    {% match extends_clause %}
        {% when Some with (extends) %}
            Some
        {% when None %}
            None
    {% endmatch %} {
    {{ obj_type }}
}"#,
    ext = "txt"
)]
pub struct InterfaceDeclaration {
    pub ident: BindingIdentifier,
    pub parameters: Option<TypeParameters>,
    pub extends_clause: Option<InterfaceExtendsClause>,
    pub obj_type: ObjectType,
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
#[display("{}", 0)]
pub struct BindingIdentifier(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeParameters {
    pub types: TypeParameter,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeParameter {
    pub ident: BindingIdentifier,
    pub constraint: Option<Constraint>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constraint {
    pub extends_type: TsType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TsType {
    Union(UnionType),
    Intersection(IntersectionType),
    Primary(PrimaryType),
}

impl std::fmt::Display for TsType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "TsType")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayType {
    pub primary_type: Box<PrimaryType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TupleType {
    pub types: Vec<BindingIdentifier>,
}

/// A & B
#[derive(Debug, Clone, PartialEq, Eq, Template)]
#[template(source = "{{ types|join(\" & \") }}", ext = "txt")]
pub struct IntersectionType {
    pub types: Vec<TsType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceExtendsClause {
    pub type_list: Vec<TypeReference>,
}

/// $name $arguments
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeReference {
    pub name: IdentifierReference,
    pub arguments: Option<TypeArguments>,
}

#[derive(Debug, Clone, PartialEq, Eq, Template)]
#[template(
    source = "{% match namespace %}
{% when Some with (val) %}
    {{ val }}.{{ ident }}
{% when None %}
    {{ ident }}
{% endmatch %}",
    ext = "txt"
)]
pub struct IdentifierReference {
    pub namespace: Option<String>,
    pub ident: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeArguments {
    pub types: Vec<TypeParameter>,
}

#[derive(Debug, Clone, PartialEq, Eq, Template)]
#[template(source = "ObjectType", ext = "txt")]
pub struct ObjectType {
    pub type_body: Option<TypeBody>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeBody {
    pub type_members: Vec<TypeMember>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeMember {
    PropertySignature(PropertySignature),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropertySignature {
    pub name: PropertyName,
    pub optional: bool,
    pub type_annotation: Option<TypeAnnotation>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeAnnotation {
    pub type_: TsType,
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum PropertyName {
    #[display("{}", .0.render().unwrap())]
    Identifier(IdentifierReference),
    #[display("{0}")]
    StringLiteral(String),
    #[display("{0}")]
    NumericLiteral(u32),
}
*/
