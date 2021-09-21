use crate::exporters::HeaderComment;
use crate::pipeline::module_step::ModuleStepResultData;

use std::path::PathBuf;

pub(crate) fn get_output_file_path(
    process_result: &ModuleStepResultData,
    default_module_name: &Option<String>,
    root_path: &PathBuf,
) -> PathBuf {
    let mut file_path: PathBuf = if process_result.path.segments.is_empty() {
        default_module_name
            .clone()
            .unwrap_or_else(|| "index".to_string())
            .into()
    } else {
        process_result
            .path
            .segments
            .iter()
            .map(|segm| segm.ident.to_string())
            .collect()
    };
    file_path.set_extension("ts");
    let mut path = root_path.clone();
    path.push(file_path);

    path
}

pub(crate) fn get_file_contents(
    process_result: ModuleStepResultData,
    header_comment: &HeaderComment,
) -> String {
    let header = header_comment.render(&process_result.path);
    let main_content: String = process_result
        .imports
        .into_iter()
        .map(|statement| format!("{}\n", statement))
        .chain(
            process_result
                .exports
                .into_iter()
                .map(|stm| format!("{}\n", stm.to_string())),
        )
        .collect();

    match header {
        None => main_content,
        Some(comment) => format!("{}\n\n{}", comment, main_content),
    }
}
