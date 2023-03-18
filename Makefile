.DEFAULT: codegen
SCHEMA_VERSION ?= "v1.0.0-rc.2"

.PHONY: codegen
codegen:
	# cargo test -p ocsf-codegen
	cargo run -p ocsf-codegen -- -d ./
	cargo build -p ocsf
	cargo fmt -p ocsf

.PHONY: build
build: codegen
	cargo fmt -p ocsf --check
	cargo doc -p ocsf
	cargo test -p ocsf

.PHONY: schema
schema:
	rm -rf ./ocsf-schema/
	git clone -b $(SCHEMA_VERSION) https://github.com/ocsf/ocsf-schema ./ocsf-schema/
