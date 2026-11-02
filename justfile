# justfile for patch-ts

# default recipe: build in debug mode
default:
    cargo build

# build in release mode
release:
    cargo build --release

# run the full workspace test suite
test:
    cargo test --workspace

# run tests with output (no capture)
test-verbose:
    cargo test --workspace -- --nocapture

# run benchmarks (criterion)
bench:
    cargo bench

# format code
fmt:
    cargo fmt

# lint with clippy: pedantic is denied workspace-wide, warnings are errors
lint:
    cargo clippy --workspace --all-targets -- -D warnings

# measure line coverage (requires cargo-llvm-cov + llvm-tools-preview)
coverage:
    cargo llvm-cov --workspace --summary-only

# emit lcov coverage report for CI consumption
coverage-lcov:
    cargo llvm-cov --workspace --lcov --output-path lcov.info

# clean build artifacts
clean:
    cargo clean

# run the binary (pass arguments after '--')
run *args:
    cargo run -- {{ args }}

# install the binary locally
install:
    cargo install --path .

# show help
help:
    @just --list
