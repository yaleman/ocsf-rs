.DEFAULT: codegen
# SCHEMA_BRANCH ?= "v1.0.0"
SCHEMA_BRANCH ?= "main"
SCHEMA_MODULE_NAME ?= "ocsf-schema"
SCHEMA_PATH ?= ./ocsf-schema

.PHONY: codegen
codegen:
	# cargo test -p ocsf-codegen
	cargo run -p ocsf-codegen -- -d ./
	cargo build -p ocsf
	cargo fmt -p ocsf

.PHONY: fmt
fmt:
	cargo fmt -p ocsf --check

.PHONY: build
build: codegen fmt doc
	cargo test -p ocsf
	cargo clippy

.PHONY: schema_pull
schema_pull:
	git submodule set-branch --branch $(SCHEMA_BRANCH) $(SCHEMA_MODULE_NAME)
	@echo "Removing existing schema dir...
	@rm -rf "$(SCHEMA_PATH)"
	git submodule update --checkout --force
	@git submodule update --remote $(SCHEMA_MODULE_NAME)
	@echo "Checking version..."
	cat "$(SCHEMA_PATH)/version.json"

.PHONY: doc
doc:
	cargo doc -p ocsf --no-deps --all-features

.PHONY: doc/open
doc/open:
	cargo doc -p ocsf --no-deps --all-features --open

