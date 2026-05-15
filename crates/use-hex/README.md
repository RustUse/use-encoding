# use-hex

Hexadecimal formatting, decoding, and normalization helpers.

> Warning: versions below `0.3.0` are experimental and may change as the crate
> surface is refined.

## Example Usage

```rust
use use_hex::{hex_decode, hex_encode, normalize_hex, HexCase};

let encoded = hex_encode(b"rust");
let decoded = hex_decode(&encoded);
let normalized = normalize_hex("#FF00AA", HexCase::Lower);

let _ = (encoded, decoded, normalized);
```

## Scope

- lowercase and uppercase hexadecimal formatting
- decoding prefixed and unprefixed hex strings
- prefix helpers and normalization for general text and color-adjacent workflows

## Non-Goals

- color parsing
- cryptographic hashing
- binary file parsing frameworks

## License

Licensed under either of the following, at your option:

- MIT License
- Apache License, Version 2.0
