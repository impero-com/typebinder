use crate::step_spawner::PipelineStepSpawner;
use crate::{error::TsExportError, pipeline::module_step::ModuleStep};

/// Strategy that discards any external module
pub struct BypassProcessSpawner;

impl PipelineStepSpawner for BypassProcessSpawner {
    type Error = TsExportError;

    fn create_process(&self, _path: syn::Path) -> Result<Option<ModuleStep>, TsExportError> {
        Ok(None)
    }
}
