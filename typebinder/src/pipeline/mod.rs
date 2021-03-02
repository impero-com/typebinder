use crate::{
    contexts::type_solving::TypeSolvingContext, error::TsExportError, exporters::Exporter,
    macros::context::MacroSolvingContext, path_mapper::PathMapper,
    step_spawner::PipelineStepSpawner,
};
use syn::{punctuated::Punctuated, Path};

use self::module_step::{ModuleStepResult, ModuleStepResultData};

pub mod module_step;
pub mod step_result;

/// The Pipeline is the starting point of Typebinder.
pub struct Pipeline<PSS, E> {
    pub pipeline_step_spawner: PSS,
    pub exporter: E,
    pub path_mapper: PathMapper,
}

/// TODO: refactor to closure
fn extractor(all: &mut Vec<ModuleStepResultData>, iter: ModuleStepResult) {
    iter.children
        .into_iter()
        .for_each(|child| extractor(all, child));
    all.push(iter.data);
}

impl<PSS, E> Pipeline<PSS, E>
where
    PSS: PipelineStepSpawner,
    E: Exporter,
    TsExportError: From<PSS::Error> + From<E::Error>,
{
    pub fn launch(
        &self,
        solving_context: &TypeSolvingContext,
        macro_context: &MacroSolvingContext,
    ) -> Result<(), TsExportError> {
        let path = Path {
            leading_colon: None,
            segments: Punctuated::default(),
        };

        let res = self
            .pipeline_step_spawner
            .create_process(path)?
            .ok_or(TsExportError::FailedToLaunch)?
            .launch(
                &self.pipeline_step_spawner,
                solving_context,
                macro_context,
                &self.path_mapper,
            )?;
        let mut all_results: Vec<ModuleStepResultData> = Vec::new();
        extractor(&mut all_results, res);

        for result_data in all_results.into_iter() {
            if result_data.imports.is_empty() && result_data.exports.is_empty() {
                continue;
            }
            self.exporter.export_module(result_data)?;
        }

        Ok(())
    }
}
