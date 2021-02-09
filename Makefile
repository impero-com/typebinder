PROJECT_DIR:=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
TEST_ARTIFACTS_DIR=$(PROJECT_DIR)/tmp/test_artifacts
DEMO_TS=$(TEST_ARTIFACTS_DIR)/demo.ts

test: prepare demo.ts
	tsc --noEmit --strict --strictNullChecks $(DEMO_TS)
demo.ts:
	cargo run --example demo > $(DEMO_TS)
prepare:
	mkdir -p $(TEST_ARTIFACTS_DIR)
	rm $(TEST_ARTIFACTS_DIR)/* -rf
