# use-base32

Practical Base32 helpers for RFC 4648-oriented text workflows.

> Warning: versions below `0.3.0` are experimental and may change as the crate
> surface is refined.

## Example Usage

```rust
use use_base32::{base32_decode, base32_encode};

let encoded = base32_encode(b"rust");
let decoded = base32_decode(&encoded);

let _ = (encoded, decoded);
```

## Scope

- standard and no-padding Base32 helpers
- lightweight detection and padding normalization utilities
- small helpers for text, fixture, and key-like string workflows

## Non-Goals

- cryptography
- TOTP or HOTP implementations
- streaming encoders and decoders

## License

Licensed under either of the following, at your option:

- MIT License
- Apache License, Version 2.0
