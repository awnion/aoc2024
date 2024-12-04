default:
	cargo build -r -q
	cargo run -r -q

test:
	cargo test -r

.PHONY: default test
