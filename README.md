# shorten-url
omit parts of a URL for friendlier display

Rust port of [shorten-url](https://github.com/goto-bus-stop/shorten-url).

## Installation
With `cargo-edit` do:
```
cargo add shorten-url
```

Or, in `Cargo.toml`:
```toml
[dependencies]
shorten-url = "1.0"
```

## Test
To run tests:
```
cargo test
```

There is also a fuzz test using `cargo-fuzz`. Run it by doing:
```
cargo +nightly fuzz run basic
```

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)
