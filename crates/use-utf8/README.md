# use-utf8

UTF-8 validation, truncation, and string-statistics helpers.

> Warning: versions below `0.3.0` are experimental and may change as the crate
> surface is refined.

## Example Usage

```rust
use use_utf8::{truncate_utf8_bytes, utf8_lossy, utf8_stats};

let lossy = utf8_lossy(b"hi\xFF");
let truncated = truncate_utf8_bytes("cafe\u{301}", 5);
let stats = utf8_stats("rust");

let _ = (lossy, truncated, stats);
```

## Scope

- UTF-8 validity checks and lossy conversions
- byte-safe truncation helpers that avoid splitting code points
- lightweight string length and ASCII/statistics helpers

## Non-Goals

- grapheme segmentation
- Unicode normalization
- locale-specific behavior

## License

Licensed under either of the following, at your option:

- MIT License
- Apache License, Version 2.0
