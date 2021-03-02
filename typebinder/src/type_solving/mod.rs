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

pub trait TypeSolver {
    fn solve_as_type(
        &self,
        _solving_context: &ExporterContext,
        _solver_info: &TypeInfo,
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
                    name: PropertyName::Identifier(solver_info.name.to_string()),
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
