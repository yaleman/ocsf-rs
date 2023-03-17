.DEFAULT: codegen
SCHEMA_VERSION ?= "v1.0.0-rc.2"

.PHONY: codegen
codegen:
	cd ocsf-codegen && cargo run --bin ocsf-codegen

.PHONY: schema
schema:
	rm -rf ./ocsf-schema/
	git clone -b $(SCHEMA_VERSION) https://github.com/ocsf/ocsf-schema ./ocsf-schema/
