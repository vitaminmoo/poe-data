.PHONY: jsonnet

all: jsonnet

jsonnet:
	jsonnet --jpath lib --string --multi ./ gen.jsonnet
	find src/tables -name '*.rs' -print0 | xargs -0 rustfmt
	rustfmt src/tables.rs
	cargo fix --lib --allow-dirty --allow-staged
