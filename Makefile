MAKEFLAGS += --warn-undefined-variables
SHELL := /bin/bash -e -u -o pipefail

run: N ?= 1
run: ARGS ?= 2,7,11,15 9
run: ENV_VARS += RUST_BACKTRACE=1
run:
	${ENV_VARS} cargo run --bin ${N} -- ${ARGS}

lint:
	cargo clippy --fix -- -Dwarnings
