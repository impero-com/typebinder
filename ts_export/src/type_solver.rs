use std::{rc::Rc, sync::Arc};

use serde_derive_internals::ast::Field;
use syn::{Generics, Type};
use ts_json_subset::types::{PropertyName, PropertySignature, TsType, TypeMember};

use crate::{
    error::TsExportError,
    exporter_context::ExporterContext,
    solvers::{
        array::ArraySolver, chrono::ChronoSolver, collections::CollectionsSolver,
        generics::GenericsSolver, import::ImportSolver, option::OptionSolver,
        primitives::PrimitivesSolver, reference::ReferenceSolver, tuple::TupleSolver,
    },
};

pub struct MemberInfo<'a> {
    pub generics: &'a Generics,
    pub field: Field<'a>,
}

#[derive(Debug)]
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

/// The result of a TypeSolver
pub enum SolverResult<T, E> {
    /// The solver could not process the given type info
    Continue,
    /// The solver correctly processed the input type
    Solved(T),
    /// The solver tried to process the input type, but it failed to do so
    Error(E),
}

impl From<Result<TsType, TsExportError>> for SolverResult<TsType, TsExportError> {
    fn from(result: Result<TsType, TsExportError>) -> Self {
        match result {
            Ok(ty) => SolverResult::Solved(ty),
            Err(e) => SolverResult::Error(e),
        }
    }
}

impl From<Result<TypeMember, TsExportError>> for SolverResult<TypeMember, TsExportError> {
    fn from(result: Result<TypeMember, TsExportError>) -> Self {
        match result {
            Ok(ty) => SolverResult::Solved(ty),
            Err(e) => SolverResult::Error(e),
        }
    }
}

pub trait TypeSolver {
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
        let result = self.solve_as_type(solving_context, &solver_info.as_type_info());
        match result {
            SolverResult::Solved(inner_type) => {
                SolverResult::Solved(TypeMember::PropertySignature(PropertySignature {
                    inner_type,
                    name: PropertyName::Identifier(solver_info.field.attrs.name().serialize_name()),
                    optional: false,
                }))
            }
            SolverResult::Error(e) => SolverResult::Error(e),
            SolverResult::Continue => SolverResult::Continue,
        }
    }
}

pub trait TypeSolverExt: TypeSolver + Sized {
    fn as_rc(self) -> Rc<Self> {
        Rc::new(self)
    }

    fn as_arc(self) -> Arc<Self> {
        Arc::new(self)
    }

    fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

impl<T: TypeSolver> TypeSolverExt for T {}

impl<T> TypeSolver for Rc<T>
where
    T: TypeSolver,
{
    fn solve_as_type(
        &self,
        solving_context: &ExporterContext,
        solver_info: &TypeInfo,
    ) -> SolverResult<TsType, TsExportError> {
        <T as TypeSolver>::solve_as_type(self.as_ref(), solving_context, solver_info)
    }

    fn solve_as_member(
        &self,
        solving_context: &ExporterContext,
        solver_info: &MemberInfo,
    ) -> SolverResult<TypeMember, TsExportError> {
        <T as TypeSolver>::solve_as_member(self.as_ref(), solving_context, solver_info)
    }
}

impl<T> TypeSolver for Arc<T>
where
    T: TypeSolver,
{
    fn solve_as_type(
        &self,
        solving_context: &ExporterContext,
        solver_info: &TypeInfo,
    ) -> SolverResult<TsType, TsExportError> {
        <T as TypeSolver>::solve_as_type(self.as_ref(), solving_context, solver_info)
    }

    fn solve_as_member(
        &self,
        solving_context: &ExporterContext,
        solver_info: &MemberInfo,
    ) -> SolverResult<TypeMember, TsExportError> {
        <T as TypeSolver>::solve_as_member(self.as_ref(), solving_context, solver_info)
    }
}

impl<T> TypeSolver for Box<T>
where
    T: TypeSolver,
{
    fn solve_as_type(
        &self,
        solving_context: &ExporterContext,
        solver_info: &TypeInfo,
    ) -> SolverResult<TsType, TsExportError> {
        <T as TypeSolver>::solve_as_type(self.as_ref(), solving_context, solver_info)
    }

    fn solve_as_member(
        &self,
        solving_context: &ExporterContext,
        solver_info: &MemberInfo,
    ) -> SolverResult<TypeMember, TsExportError> {
        <T as TypeSolver>::solve_as_member(self.as_ref(), solving_context, solver_info)
    }
}

pub struct TypeSolvingContext {
    solvers: Vec<Box<dyn TypeSolver>>,
}

impl TypeSolvingContext {
    pub fn solvers(&self) -> &Vec<Box<dyn TypeSolver>> {
        &self.solvers
    }
}

#[derive(Default)]
pub struct TypeSolvingContextBuilder {
    solvers: Vec<Box<dyn TypeSolver>>,
}

impl TypeSolvingContextBuilder {
    pub fn add_solver<S: TypeSolver + 'static>(mut self, solver: S) -> Self {
        self.solvers.push(solver.boxed());
        self
    }

    pub fn add_default_solvers(self) -> Self {
        self.add_solver(TupleSolver)
            .add_solver(ReferenceSolver)
            .add_solver(ArraySolver)
            .add_solver(CollectionsSolver::default())
            .add_solver(PrimitivesSolver::default())
            .add_solver(OptionSolver::default())
            .add_solver(GenericsSolver)
            .add_solver(ChronoSolver::default())
    }

    pub fn finish(self) -> TypeSolvingContext {
        let builder = self.add_solver(ImportSolver);
        TypeSolvingContext {
            solvers: builder.solvers,
        }
    }
}
