use serde_derive_internals::ast::Field;
use syn::{Generics, Type};
use ts_json_subset::types::{PropertyName, PropertySignature, TsType, TypeMember};

pub struct MemberInfo<'a> {
    pub generics: &'a Generics,
    pub field: Field<'a>,
}

pub struct TypeInfo<'a> {
    pub generics: &'a Generics,
    pub ty: &'a Type,
}

impl<'a> MemberInfo<'a> {
    pub fn as_type_info(&self) -> TypeInfo<'a> {
        let MemberInfo { generics, field } = self;
        TypeInfo {
            generics,
            ty: field.ty,
        }
    }
}

pub trait TypeSolver {
    fn solve_as_type(
        &self,
        _solving_context: &TypeSolvingContext,
        _solver_info: &TypeInfo,
    ) -> Option<TsType> {
        None
    }

    fn solve_as_member(
        &self,
        solving_context: &TypeSolvingContext,
        solver_info: &MemberInfo,
    ) -> Option<ts_json_subset::types::TypeMember> {
        let inner_type = self.solve_as_type(solving_context, &solver_info.as_type_info())?;
        Some(TypeMember::PropertySignature(PropertySignature {
            inner_type,
            name: PropertyName::Identifier(solver_info.field.attrs.name().serialize_name()),
            optional: false,
        }))
    }
}

#[derive(Default)]
pub struct TypeSolvingContext {
    solvers: Vec<Box<dyn TypeSolver>>,
}

impl TypeSolvingContext {
    pub fn add_solver<S: TypeSolver + 'static>(&mut self, solver: S) {
        self.solvers.push(Box::new(solver));
    }

    pub fn solve_type(&self, solver_info: &TypeInfo) -> Option<TsType> {
        self.solvers
            .iter()
            .filter_map(|solver| solver.as_ref().solve_as_type(&self, solver_info))
            .next()
    }

    pub fn solve_member(&self, solver_info: &MemberInfo) -> Option<TypeMember> {
        self.solvers
            .iter()
            .filter_map(|solver| solver.as_ref().solve_as_member(&self, solver_info))
            .next()
    }
}
