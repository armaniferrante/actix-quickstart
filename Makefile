.PHONY: watch

watch:
	DATABASE_URL="postgres://armaniferrante:password@localhost/armaniferrante" cargo watch -x 'run --bin actix-quickstart -- --config config/dev.yaml'
