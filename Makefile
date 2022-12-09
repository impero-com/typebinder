PROJECT_DIR:=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
TEST_ARTIFACTS_DIR=$(PROJECT_DIR)/tmp/test_artifacts
DEMO_TS=$(TEST_ARTIFACTS_DIR)/demo.ts

watch:
	cargo watch --shell "make test"
test: test_unit test_integration
test_unit:
	cargo test
test_integration:
	mkdir -p $(TEST_ARTIFACTS_DIR)
	rm $(TEST_ARTIFACTS_DIR)/* -rf
	cargo run --bin typebinder_test_suite demo > $(DEMO_TS)
	npx tsc --noEmit --strict --strictNullChecks $(DEMO_TS)
