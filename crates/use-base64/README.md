# use-base64

Practical Base64 helpers for standard and URL-safe text workflows.

> Warning: versions below `0.3.0` are experimental and may change as the crate
> surface is refined.

## Example Usage

```rust
use use_base64::{base64_decode, base64_encode};

let encoded = base64_encode(b"rust");
let decoded = base64_decode(&encoded);

let _ = (encoded, decoded);
```

## Scope

- standard and URL-safe Base64 helpers
- lightweight detection and padding utilities
- small helpers for docs tooling, CLIs, fixtures, and web-adjacent strings

## Non-Goals

- cryptography
- MIME multipart handling
- streaming encoders and decoders

## License

Licensed under either of the following, at your option:

- MIT License
- Apache License, Version 2.0
