use crate::process::ProcessModule;
use crate::process_spawner::ProcessSpawner;

/// Strategy that discards any external module
pub struct BypassProcessSpawner;

impl ProcessSpawner for BypassProcessSpawner {
    fn create_process(&self, _path: syn::Path) -> Option<ProcessModule> {
        None
    }
}
