use typebinder::process_file;

fn main() {
    pretty_env_logger::init();
    process_file("assets/example_file.rs").expect("Failed to export TS");
}
