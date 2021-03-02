.PHONY: build test fmt

build:
	cd gateway && cargo build --release && cd ../
	cd app && yarn && yarn build && cd ../

test:
	cd gateway && cargo test && cd ../

fmt:
	cd gateway && cargo fmt -- --check && cd ../
	cd app && yarn && yarn lint && cd ../
