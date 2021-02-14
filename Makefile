.PHONY: watch

watch:
	cargo watch -x 'run --bin actix-quickstart -- --config config/dev.yaml'
