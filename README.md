# RustUse use-encoding

Small, focused Rust 2024 utility crates for encoding, decoding, escaping, and
text-or-byte representation helpers.

> Warning: crates below `0.3.0` are experimental and may change as the
> workspace surface is refined.

## Crate List

- `use-encoding`: feature-gated umbrella crate for the full workspace
- `use-percent`: percent-encoding and decoding helpers for practical text inputs
- `use-base64`: standard and URL-safe Base64 helpers with lightweight detection
- `use-hex`: hexadecimal encoding, decoding, prefix, and normalization helpers
- `use-ascii`: ASCII detection, classification, and replacement helpers
- `use-utf8`: UTF-8 validation, truncation, and string statistics helpers
- `use-escape`: HTML, XML, JSON, CSV, and shell-oriented escaping helpers
- `use-base32`: RFC 4648 Base32 helpers with padding normalization

## Scope

- lightweight encoding and escaping primitives
- predictable byte and string formatting helpers
- malformed-input detection and graceful failure with `Option`-based fallbacks
- small composable utilities for CLIs, docs tooling, fixtures, config helpers,
  and application scaffolding

## Non-Goals

- replacing mature encoding, cryptography, or compression crates
- full parser ecosystems or WHATWG-complete URL processing
- heavyweight charset conversion frameworks
- shell or markup sanitization suitable for security boundaries

## Relationship to Other RustUse Sets

- `use-web` can consume these helpers for URL components, query fragments, and
  transport-safe text handling
- `use-text` stays focused on text structure such as case, slugs, words, and
  tokens while `use-encoding` handles representation and escaping primitives
- `use-data` can reuse the escaping and byte-format helpers around fixtures,
  serialization-adjacent tooling, and small validators
- `use-media` can build on hex, Base64, UTF-8, and escape helpers for asset,
  SVG, metadata, or file-format utilities

## Example Usage

```toml
[dependencies]
use-percent = "0.1"
use-utf8 = "0.1"
use-encoding = { version = "0.1", default-features = false, features = ["hex"] }
```

```rust
use use_encoding::hex;
use use_percent::percent_encode_component;
use use_utf8::truncate_utf8_bytes;

assert_eq!(percent_encode_component("hello world"), "hello%20world");
assert_eq!(truncate_utf8_bytes("cafe\u{301}", 5), "cafe");
assert_eq!(hex::normalize_hex("#FF00AA", hex::HexCase::Lower).unwrap(), "ff00aa");
```

## License

Licensed under either of the following, at your option:

- MIT License
- Apache License, Version 2.0
