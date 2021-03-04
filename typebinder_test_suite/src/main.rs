fn main() {
    pretty_env_logger::init();
    typebinder::process_file("typebinder_test_suite/src/models.rs")
        .expect("Failed to process file");
}
