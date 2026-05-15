# use-encoding

Feature-gated umbrella crate for the RustUse encoding helpers workspace.

> Warning: versions below `0.3.0` are experimental and may change as the
> workspace evolves.

## Example Usage

```toml
[dependencies]
use-encoding = { version = "0.1", default-features = false, features = ["percent", "hex"] }
```

```rust
#[cfg(feature = "percent")]
use use_encoding::percent::percent_encode_component;
#[cfg(feature = "hex")]
use use_encoding::hex::{self, HexCase};

#[cfg(feature = "percent")]
let _encoded = percent_encode_component("hello world");
#[cfg(feature = "hex")]
let _normalized = hex::normalize_hex("#FF00AA", HexCase::Lower);
```

## Scope

- opt-in access to the focused `use-percent`, `use-base64`, `use-hex`,
  `use-ascii`, `use-utf8`, `use-escape`, and `use-base32` crates
- a small facade surface that does not force unused helpers into downstream
  builds
- lightweight composition across RustUse web, text, data, media, docs, and CLI
  tooling

## Non-Goals

- adding behavior beyond the child crates
- replacing dedicated parser, encoding, or escaping ecosystems
- forcing a shared abstraction across unrelated text and byte workflows

## License

Licensed under either of the following, at your option:

- MIT License
- Apache License, Version 2.0
