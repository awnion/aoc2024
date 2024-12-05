default: inputs
	cargo build -r -q
	cargo run -r -q

test:
	cargo test -r


inputs:
	dotenvy ./bin/fetch_inputs.sh


clean_inputs:
	rm -rf inputs


fix:
	cargo fix --allow-dirty --allow-staged
	cargo fmt

.PHONY: default test inputs clean_inputs fix
