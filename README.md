# Readme

This repository is an attempt at a minimal reproducible example of [uniffi-rs#2282](https://github.com/mozilla/uniffi-rs/issues/2282).

The `map-north-america` crate uses the `countries` crate as a dependency, but only imports the `NorthAmericanCountries` enum and the `hello_canada()` method, yet other types are present in the final library Kotlin code: 

- [`EuropeanCountries` enum](https://github.com/thunderbiscuit/uniffi-2282/blob/master/org/example/countries/countries.kt#L1057-L1063)
- [`hello_ireland()` function](https://github.com/thunderbiscuit/uniffi-2282/blob/master/org/example/countries/countries.kt#L1123-L1130)

The build steps are in `build.sh`, and are standard uniffi build commands:

```sh
# Build the Rust library
cargo build --release --target aarch64-apple-darwin

# Generate Kotlin bindings using uniffi-bindgen
cargo run --bin uniffi-bindgen generate --library ./target/aarch64-apple-darwin/release/libmap_na.dylib --language kotlin --out-dir ../ --no-format
```
