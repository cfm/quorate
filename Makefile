PORT=8080
THIS=proxy-solver-api

build:
	docker build --tag ${THIS} .

check:
	cargo fmt -- --check
	cargo clippy -- --deny clippy::all

fmt:
	cargo fmt

run:
	docker run \
		--env PORT=${PORT} \
		--publish ${PORT}:${PORT} \
		${THIS}

test:
	clear || true
	cargo test -- --show-output
