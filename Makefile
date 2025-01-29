.PHONY: jsonnet pathofexile-dat

all: jsonnet pathofexile-dat

jsonnet:
	jsonnet --jpath lib --string --multi ./ gen.jsonnet
	find . -name '*.rs' -print0 | xargs -0 rustfmt
	cargo fix --lib --allow-dirty --allow-staged
pathofexile-dat: data/config.json
	cd data && pathofexile-dat
