use crate::process::ProcessModule;
use crate::process_spawner::ProcessSpawner;
use crate::error::TsExportError;

/// Strategy that discards any external module
pub struct BypassProcessSpawner;

impl ProcessSpawner for BypassProcessSpawner {
    type Error = TsExportError;

    fn create_process(&self, _path: syn::Path) -> Result<Option<ProcessModule>, TsExportError> {
        Ok(None)
    }
}
