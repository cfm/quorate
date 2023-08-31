PORT=8080
THIS=proxy-solver-api

VENV=.venv
PYTHON_REQUIREMENTS=requirements.txt

build:
	docker build --tag ${THIS} .

check:
	cargo fmt -- --check
	cargo clippy -- --deny clippy::all

check-py: bin/anonymize
	black --check $<
	isort --check $<

docs:
	cargo doc --no-deps

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

venv:
	virtualenv --python python3 ${VENV}
	${VENV}/bin/pip3 install --requirement ${PYTHON_REQUIREMENTS}
