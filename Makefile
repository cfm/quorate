check:
	cargo fmt -- --check
	cargo clippy -- --deny clippy::all

fmt:
	cargo fmt

test:
	clear || true
	cargo test -- --show-output
