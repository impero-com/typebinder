use std::{rc::Rc, sync::Arc};

use serde_derive_internals::ast::Field;
use syn::{Generics, Type};
use ts_json_subset::types::{PropertyName, PropertySignature, TsType, TypeMember};

use crate::error::TsExportError;

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
        _solving_context: &TypeSolvingContext,
        _solver_info: &TypeInfo,
    ) -> SolverResult<TsType, TsExportError> {
        SolverResult::Continue
    }

    fn solve_as_member(
        &self,
        solving_context: &TypeSolvingContext,
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

impl<T> TypeSolver for Rc<T>
where
    T: TypeSolver,
{
    fn solve_as_type(
        &self,
        solving_context: &TypeSolvingContext,
        solver_info: &TypeInfo,
    ) -> SolverResult<TsType, TsExportError> {
        <T as TypeSolver>::solve_as_type(self.as_ref(), solving_context, solver_info)
    }

    fn solve_as_member(
        &self,
        solving_context: &TypeSolvingContext,
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
        solving_context: &TypeSolvingContext,
        solver_info: &TypeInfo,
    ) -> SolverResult<TsType, TsExportError> {
        <T as TypeSolver>::solve_as_type(self.as_ref(), solving_context, solver_info)
    }

    fn solve_as_member(
        &self,
        solving_context: &TypeSolvingContext,
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
        solving_context: &TypeSolvingContext,
        solver_info: &TypeInfo,
    ) -> SolverResult<TsType, TsExportError> {
        <T as TypeSolver>::solve_as_type(self.as_ref(), solving_context, solver_info)
    }

    fn solve_as_member(
        &self,
        solving_context: &TypeSolvingContext,
        solver_info: &MemberInfo,
    ) -> SolverResult<TypeMember, TsExportError> {
        <T as TypeSolver>::solve_as_member(self.as_ref(), solving_context, solver_info)
    }
}

/*
use xxx;
-> TypeSolvingContext : Ajouter "ImportContext"
-> Mark import as "used"

HashMap<String, String>
* SingleDto => impero_common::api::SingleDto,

ImportContext: syn::TypePath => syn::Type::(syn::TypePath), e.g. std::collections::HashSet, std::vec::Vec
DefaultSolver => Type: FullPath

ImperoCommonSolver:
impero_common::api::SingleDto => Solved(TsType::"SingleDto", Some(AddImport("SingleDto", "types/impero_common/api")))

ImportContext:
* use
* scoped
* prelude + primitive

*/

/*
impl TypeSolver
    for fn(solving_context: &TypeSolvingContext, solver_info: &TypeInfo) -> Option<TsType>
{
    fn solve_as_type(
        &self,
        solving_context: &TypeSolvingContext,
        solver_info: &TypeInfo,
    ) -> Option<TsType> {
        self(solving_context, solver_info)
    }
}
*/

#[derive(Default)]
pub struct TypeSolvingContext {
    solvers: Vec<Box<dyn TypeSolver>>,
}

impl TypeSolvingContext {
    pub fn add_solver<S: TypeSolver + 'static>(&mut self, solver: S) {
        self.solvers.push(Box::new(solver));
    }

    pub fn solve_type(&self, solver_info: &TypeInfo) -> Result<TsType, TsExportError> {
        for solver in &self.solvers {
            match solver.as_ref().solve_as_type(&self, solver_info) {
                SolverResult::Continue => (),
                SolverResult::Solved(inner) => return Ok(inner),
                SolverResult::Error(inner) => return Err(inner),
            }
        }
        return Err(TsExportError::UnsolvedType(solver_info.ty.clone()));
    }

    pub fn solve_member(&self, solver_info: &MemberInfo) -> Result<TypeMember, TsExportError> {
        for solver in &self.solvers {
            match solver.as_ref().solve_as_member(&self, solver_info) {
                SolverResult::Continue => (),
                SolverResult::Solved(inner) => return Ok(inner),
                SolverResult::Error(inner) => return Err(inner),
            }
        }
        return Err(TsExportError::UnsolvedField(
            solver_info.field.original.clone(),
        ));
    }
}
