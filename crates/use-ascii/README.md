# use-ascii

ASCII detection, classification, stripping, and replacement helpers.

> Warning: versions below `0.3.0` are experimental and may change as the crate
> surface is refined.

## Example Usage

```rust
use use_ascii::{ascii_kind, replace_non_ascii, strip_non_ascii};

let stripped = strip_non_ascii("caf\u{e9}");
let replaced = replace_non_ascii("caf\u{e9}", '?');
let kind = ascii_kind('A');

let _ = (stripped, replaced, kind);
```

## Scope

- ASCII-only validation and classification helpers
- removal or replacement of non-ASCII characters
- small utilities for config, CLI, docs, and text-cleanup workflows

## Non-Goals

- Unicode normalization
- transliteration
- locale-aware casing behavior

## License

Licensed under either of the following, at your option:

- MIT License
- Apache License, Version 2.0
