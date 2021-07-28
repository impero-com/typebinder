//! The core logic of `typebinder`

use crate::{
    contexts::type_solving::TypeSolvingContext, error::TsExportError, exporters::Exporter,
    macros::context::MacroSolvingContext, path_mapper::PathMapper,
    step_spawner::PipelineStepSpawner,
};
use syn::{punctuated::Punctuated, Path};

use self::module_step::{ModuleStepResult, ModuleStepResultData};

pub mod module_step;
pub mod step_result;

/// The Pipeline is the starting point of `typebinder`.
///
/// A Pipeline is customized with both a [PipelineStepSpawner] and an [Exporter] implementor.
///
/// When launched, the [Pipeline] will use its [PipelineStepSpawner] to spawn the "default" module, that is, the module with an empty path.
/// Each [ModuleStep](crate::pipeline::module_step::ModuleStep) thereby generated is then launched, see [ModuleStep::launch](crate::pipeline::module_step::ModuleStep).  
///
/// Each output is passed to the [Exporter], that is responsible for outputting the data.
pub struct Pipeline<PSS, E> {
    pub pipeline_step_spawner: PSS,
    pub exporter: E,
    pub path_mapper: PathMapper,
}

impl<PSS, E> Pipeline<PSS, E>
where
    PSS: PipelineStepSpawner,
    E: Exporter,
    TsExportError: From<PSS::Error> + From<E::Error>,
{
    pub fn launch(
        mut self,
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

/// TODO: refactor this to a closure
fn extractor(all: &mut Vec<ModuleStepResultData>, iter: ModuleStepResult) {
    iter.children
        .into_iter()
        .for_each(|child| extractor(all, child));
    all.push(iter.data);
}
