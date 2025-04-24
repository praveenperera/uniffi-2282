#!/bin/bash

# Set default Rust compiler
rustup default 1.84.1

# Move to the Rust library directory
cd ./map-north-america/

# Build the Rust library
cargo build --profile release-smaller --target aarch64-apple-darwin

# Generate Kotlin bindings using uniffi-bindgen
rm -rf ../org
cargo run --bin uniffi-bindgen generate --library ./target/aarch64-apple-darwin/release-smaller/libmap_na.dylib --language kotlin --out-dir ../ --no-format

# Swift
mkdir -p ../swift
cargo run --bin uniffi-bindgen generate --library ./target/aarch64-apple-darwin/release-smaller/libmap_na.a --language swift --out-dir ../swift --no-format
