//! Utilities to transform Rust types into TS ones

use self::{member_info::MemberInfo, result::SolverResult, type_info::TypeInfo};
use crate::{contexts::exporter::ExporterContext, error::TsExportError};
use std::{rc::Rc, sync::Arc};
use ts_json_subset::types::{PropertyName, PropertySignature, TsType, TypeMember};

pub mod fn_solver;
pub mod member_info;
pub mod result;
pub mod solvers;
pub mod type_info;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct ImportEntry {
    pub path: String,
    pub ident: String,
}

/// The TypeSolver is the main abstraction of Typebinder. It is what allows its modularity.
///
/// A TypeSolver is given an ExporterContext, and information to solve.  
/// The TypeSolvingContext will chain the solvers one after the other the first TypeSolver that succeeds in solving the type to a TS type returns its result.
///
/// TypeSolver differentiates between two kinds of info.
///
/// A TypeInfo contains information regarding a type. It is used when solving :
/// * An element of a tuple,
/// * The element of a unit type,  
/// * A generic,
/// * As a fallback to solve the type of a TypeMember (see below)
///
/// A MemberInfo contains information regarding a field of a struct. That is, the name of the field, its type, its attributes.
/// It is used when solving a struct.
/// Most of the time, it will just fallback to solving the inner type by building the TypeInfo and using `solve_as_type`.
/// Sometimes, you might want to rename your field or put it as optional. For these case, your solver needs to implement `solve_as_member`.   
///
/// It returns a SolverResult, where :
/// * SolverResult::Continue means that the solver wasn't relevant
/// * SolverResult::Solved means that the solver has succeeded in its task
/// * SolverResult::Error means that the solver had an unrecoverable error
///
pub trait TypeSolver {
    fn solve_as_type(
        &self,
        solving_context: &ExporterContext,
        solver_info: &TypeInfo,
    ) -> SolverResult<TsType, TsExportError>;

    fn solve_as_member(
        &self,
        solving_context: &ExporterContext,
        solver_info: &MemberInfo,
    ) -> SolverResult<TypeMember, TsExportError> {
        let result = self.solve_as_type(solving_context, &solver_info.as_type_info());
        match result {
            SolverResult::Solved(inner_type, imports) => SolverResult::Solved(
                TypeMember::PropertySignature(PropertySignature {
                    inner_type,
                    name: PropertyName::from(solver_info.name.clone()),
                    optional: false,
                }),
                imports,
            ),
            SolverResult::Error(e) => SolverResult::Error(e),
            SolverResult::Continue => SolverResult::Continue,
        }
    }
}

pub trait TypeSolverExt: TypeSolver + Sized {
    fn into_rc(self) -> Rc<Self> {
        Rc::new(self)
    }

    fn into_arc(self) -> Arc<Self> {
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
