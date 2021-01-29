use ts_export::process_file;

fn main() {
    process_file("assets/example_file.rs").expect("Failed to export TS");
}
