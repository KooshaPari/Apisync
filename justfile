# Apisync — task runner (https://just.systems)
# Parallel to Taskfile.yml; use either, justfile is the canonical entrypoint.

set shell := ["bash", "-uc"]

default:
    @just --list

# Start dev server / watch mode
dev:
    cargo watch -x check -x test

# Produce release artifacts
build:
    cargo build --all-targets --release

# Run the test suite
test:
    cargo test --all-targets

# Run the linter
lint:
    cargo clippy --all-targets -- -D warnings

# Apply formatter
fmt:
    cargo fmt

# Remove build artifacts
clean:
    cargo clean
    rm -rf target
