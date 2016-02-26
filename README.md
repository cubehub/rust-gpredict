# rust-gpredict
Rust wrapper for [libgpredict](https://github.com/cubehub/libgpredict).

## dependencies
### libgpredict
Follow install instructions from here: https://github.com/cubehub/libgpredict

## usage
Put this in your `Cargo.toml`:

```toml
[dependencies.gpredict]
git = "https://github.com/cubehub/rust-gpredict.git"
```

And this in your crate root:

```rust
extern crate gpredict;
```

## run example
```
cargo run --example predict
```
