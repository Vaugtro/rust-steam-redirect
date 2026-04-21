# Makefile for convenient development commands

.PHONY: help build release test fmt lint doc clean check

help:
	@echo "Available commands:"
	@echo "  make build       - Build in debug mode"
	@echo "  make release     - Build optimized release binary"
	@echo "  make test        - Run all tests"
	@echo "  make fmt         - Format code (run rustfmt)"
	@echo "  make fmt-check   - Check code formatting"
	@echo "  make lint        - Run clippy linter"
	@echo "  make doc         - Generate documentation"
	@echo "  make clean       - Remove build artifacts"
	@echo "  make check       - Run cargo check"
	@echo "  make all         - Format, lint, test, and build"

build:
	cargo build

release:
	cargo build --release

test:
	cargo test --verbose

fmt:
	cargo fmt

fmt-check:
	cargo fmt -- --check

lint:
	cargo clippy -- -D warnings

doc:
	cargo doc --no-deps --open

clean:
	cargo clean

check:
	cargo check

all: fmt lint test build
	@echo "✓ All checks passed!"

.DEFAULT_GOAL := help
