.PHONY: all build debug release clean run test check fmt lint

BINARY_NAME := countdown-timer-rs

all: build

# Debug build (default)
build: debug

debug:
	cargo build

# Release build with optimizations
release:
	cargo build --release

# Run the debug binary
run:
	cargo run

# Run with arguments (usage: make run-args ARGS="--minutes 5")
run-args:
	cargo run -- $(ARGS)

# Run tests
test:
	cargo test

# Type-check without building
check:
	cargo check

# Format code
fmt:
	cargo fmt

# Lint with clippy
lint:
	cargo clippy -- -D warnings

# Clean build artifacts
clean:
	cargo clean
