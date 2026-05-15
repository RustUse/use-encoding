# use-percent

Percent-encoding and decoding helpers for common text components.

> Warning: versions below `0.3.0` are experimental and may change as the crate
> surface is refined.

## Example Usage

```rust
use use_percent::{percent_decode_component, percent_encode_component};

let encoded = percent_encode_component("hello world");
let decoded = percent_decode_component(&encoded);

let _ = (encoded, decoded);
```

## Scope

- encoding and decoding practical percent-escaped text
- detecting valid, invalid, and present percent escape sequences
- small helpers for URL component style workflows without a full URL parser

## Non-Goals

- full WHATWG URL processing
- charset conversion
- replacing mature URL parser libraries

## License

Licensed under either of the following, at your option:

- MIT License
- Apache License, Version 2.0
