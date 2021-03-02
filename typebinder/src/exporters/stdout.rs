use crate::error::TsExportError;
use crate::exporters::Exporter;
use crate::{pipeline::module_step::ModuleStepResultData, utils::display_path::DisplayPath};

/// A strategy that prints the name and the output module's statements to stdout.
pub struct StdoutExport;

impl Exporter for StdoutExport {
    type Error = TsExportError;

    fn export_module(&self, process_result: ModuleStepResultData) -> Result<(), TsExportError> {
        println!("//------");
        let mut display_path = DisplayPath(&process_result.path).to_string();
        if display_path.is_empty() {
            display_path = "Default module".to_string();
        }
        println!("// {}", display_path);
        println!("// ------");
        let output: String = process_result
            .imports
            .into_iter()
            .map(|statement| format!("{}\n", statement))
            .chain(
                process_result
                    .exports
                    .into_iter()
                    .map(|stm| stm.to_string()),
            )
            .collect();

        println!("{}", output);

        Ok(())
    }
}
