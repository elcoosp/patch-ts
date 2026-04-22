# justfile for patch-ts

# default recipe: build in debug mode
default:
    cargo build

# build in release mode
release:
    cargo build --release

# run tests
test:
    cargo test

# run tests with output (no capture)
test-verbose:
    cargo test -- --nocapture

# run benchmarks (requires nightly)
bench:
    cargo bench

# format code
fmt:
    cargo fmt

# lint with clippy
lint:
    cargo clippy -- -D warnings

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
