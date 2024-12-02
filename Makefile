default:
	cargo build -r -q
	cargo run -r -q

test:
	cargo test

.PHONY: default test
