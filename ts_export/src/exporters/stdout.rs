use crate::exporters::Exporter;
use crate::{display_path::DisplayPath, process::ProcessModuleResultData};

pub struct StdoutExport;

impl Exporter for StdoutExport {
    fn export_module(&self, process_result: ProcessModuleResultData) {
        println!("//------");
        let mut display_path = DisplayPath(&process_result.path).to_string();
        if display_path.is_empty() {
            display_path = "Default module".to_string();
        }
        println!("// {}", display_path);
        println!("// ------");
        let output: String = process_result
            .statements
            .into_iter()
            .map(|statement| format!("{}\n", statement))
            .collect();
        println!("{}", output);
    }
}
