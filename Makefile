.PHONY: all
all: lint test build

.PHONY: build
build:
	cargo build

.PHONY: test
test:
	cargo test

.PHONY: lint
lint: clippy fmt

.PHONY: clippy
clippy:
	cargo clippy

.PHONY: fmt
fmt:
	cargo fmt --check

.PHONY: do_fmt
do_fmt:
	cargo fmt

.PHONY: clean
clean:
	cargo clean
