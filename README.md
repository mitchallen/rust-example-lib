rust-example-lib
==


[![Actions Status](https://github.com/mitchallen/rust-example-lib/workflows/Build%20Test/badge.svg)](https://github.com/mitchallen/rust-example-lib/actions)


## Usage

* Edit Cargo.toml
* Add dependency (edit latest tag):

```toml
[dependencies]
rust-example_lib = { git = "https://github.com/mitchallen/rust-example-lib.git", tag = "v0.1.0" }
```

## Rust

This will return a random boolean (true or false)

```rs
let result = rust_example_lib::coin::flip();
```

## Build

```sh
make all
```

## Tag new versions


```sh
git tag v0.1.0
```

```sh
git push origin main --tags
```

## How this project was initialized

```sh
cargo new --lib rust-example-lib
cd rust-example-lib
```

## Structure

```sh
├── Cargo.toml
└── src
    └── lib.rs
```

## References

* https://www.tutorialspoint.com/rust/rust_modules.htm
