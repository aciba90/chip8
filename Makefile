.PHONY: all
all: lint test build

.PHONY: build
build:
	cargo build

.PHONY: test
test:
	cargo build

.PHONY: clean
lint: clippy fmt

.PHONY: clean
clippy:
	cargo clippy

.PHONY: clean
fmt:
	cargo fmt --check

.PHONY: clean
do_fmt:
	cargo fmt

.PHONY: clean
clean:
	rm -rf target
