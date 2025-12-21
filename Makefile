.PHONY: jsonnet

all: jsonnet

jsonnet:
	jsonnet --jpath lib --string --multi ./ gen.jsonnet
	find src/tables -name '*.rs' -print0 | xargs -0 rustfmt
	cargo fix --lib --allow-dirty --allow-staged
