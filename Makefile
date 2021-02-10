PROJECT_DIR:=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
TEST_ARTIFACTS_DIR=$(PROJECT_DIR)/tmp/test_artifacts
DEMO_TS=$(TEST_ARTIFACTS_DIR)/demo.ts

watch:
	cargo watch --shell "make test"
test: test_unit test_itegration
test_unit:
	cargo test
test_itegration:
	mkdir -p $(TEST_ARTIFACTS_DIR)
	rm $(TEST_ARTIFACTS_DIR)/* -rf
	cargo run --example demo > $(DEMO_TS)
	tsc --noEmit --strict --strictNullChecks $(DEMO_TS)
