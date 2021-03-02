DATABASE_URL="postgres://armaniferrante:password@localhost/armaniferrante"

.PHONY: watch migrate

watch:
	DATABASE_URL=$(DATABASE_URL) cargo watch -x 'run --bin gateway -- --config config/dev.yaml'

migrate:
	DATABASE_URL=$(DATABASE_URL) sqlx migrate run
