#!/bin/bash

# Set default Rust compiler
rustup default 1.84.1

# Move to the Rust library directory
cd ./map-north-america/

# Build the Rust library
cargo build --release --target aarch64-apple-darwin

# Generate Kotlin bindings using uniffi-bindgen
cargo run --bin uniffi-bindgen generate --library ./target/aarch64-apple-darwin/release/libmap_na.dylib --language kotlin --out-dir ../ --no-format
