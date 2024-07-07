DEFAULT: help

SCHEMA_BRANCH ?= "v1.2.0"
# SCHEMA_BRANCH ?= main
SCHEMA_MODULE_NAME ?= ocsf-schema
SCHEMA_PATH ?= ./ocsf-schema

.PHONY: help
help:
	@grep -E -h '\s##\s' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

.PHONY: codegen
codegen: ## generate the ocsf crate, run cargo fmt on it and test it actually builds
	cargo run -p ocsf-codegen -- -d ./
	cargo fmt -p ocsf
	cargo build -p ocsf


.PHONY: build
build: ## codegen + doc generation + clippy + test
build: codegen doc
	cargo clippy
	cargo test -p ocsf

.PHONY: schema_pull
schema_pull:
schema_pull: ## Update the schema git submodule based on the SCHEMA_BRANCH argument
	git submodule set-branch --branch $(SCHEMA_BRANCH) $(SCHEMA_MODULE_NAME)
	@echo "Removing existing schema dir..."
	rm -rf "$(SCHEMA_PATH)"
	git submodule update --checkout --force
	@git submodule update --remote $(SCHEMA_MODULE_NAME)
	@echo "Checking version..."
	cat "$(SCHEMA_PATH)/version.json"

.PHONY: doc
doc:
doc: ## Generate the rust documentation for the OCSF module and also an index.html that redirects to it from the parent dir.
	cargo doc -p ocsf --no-deps --all-features
	@echo "Recreating index redirector file..."
	@echo "<meta http-equiv=\"refresh\" content=\"0; url=ocsf\">" > target/doc/index.html


.PHONY: doc/open
doc/open: ## Generate the documentation and then open it.
doc/open:
	cargo doc -p ocsf --no-deps --all-features --open


.PHONY: watch/doc
watch/doc: ## Use cargo watch to generate the documentation for the OCSF package, and regenerate when the source changes.
watch/doc:
	cargo watch -w ocsf --no-restart -s 'make doc'

.PHONY: watch/build
watch/build: ## Use cargo watch to build the OCSF package as the codegen source changes.
watch/build:
	cargo watch -w ocsf-codegen --no-restart -s 'make'

