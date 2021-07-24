# serde-enum-str

* [Cargo package](https://crates.io/crates/serde-enum-str)

## Solves

https://stackoverflow.com/questions/57469527

## Dev

```
cargo clippy --all-features --tests -- -D clippy::all
cargo +nightly clippy --all-features --tests -- -D clippy::all

cargo fmt -- --check

cargo build-all-features
cargo test-all-features -- --nocapture
```

```
cargo expand --verbose --test test
```
