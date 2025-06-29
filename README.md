# serde-extras

[![Crates.io](https://img.shields.io/crates/v/serde-extras.svg)](https://crates.io/crates/serde-extras)
[![Docs.rs](https://docs.rs/serde-extras/badge.svg)](https://docs.rs/serde-extras)

A lightweight crate providing utilities for serializing and deserializing types using their string representations with [Serde](https://serde.rs/).

## Features

- Serialize and deserialize types that implement [`ToString`](https://doc.rust-lang.org/std/string/trait.ToString.html) and [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) via Serde attributes.
- Minimal dependencies and small code footprint.

## Usage

Add to your `Cargo.toml`:

```toml
serde-extras = "0.1"
```

Annotate your struct fields with Serde's `with` attribute:

```rust
use std::net::IpAddr;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Wrapper {
    #[serde(with = "serde_extras::to_from_str")]
    ip: IpAddr,
}
```

This will serialize the `ip` field as a string and deserialize it from a string.

## Example

```rust
use std::net::IpAddr;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Wrapper {
    #[serde(with = "serde_extras::to_from_str")]
    ip: IpAddr,
}

let w = Wrapper { ip: IpAddr::V4("127.0.0.1".parse().unwrap()) };
let json = serde_json::to_string(&w).unwrap();
assert_eq!(json, r#"{"ip":"127.0.0.1"}"#);
let de: Wrapper = serde_json::from_str(&json).unwrap();
assert_eq!(de, w);
```

## Comparison

This crate is a lightweight alternative to a small portion of [`serde_with`](https://github.com/jonasbb/serde_with/). If you only need simple string-based (de)serialization helpers, `serde-extras` may be a better fit. For more advanced features, consider using `serde_with`.

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   <https://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   <https://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `serde-extras` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
