# use-escape

Escaping and unescaping helpers for HTML, XML, JSON, CSV, and basic shell text.

> Warning: versions below `0.3.0` are experimental and may change as the crate
> surface is refined.

## Example Usage

```rust
use use_escape::{escape_html, escape_json_string, unescape_html};

let html = escape_html("<tag>");
let json = escape_json_string("line\nbreak");
let unescaped = unescape_html("&lt;tag&gt;");

let _ = (html, json, unescaped);
```

## Scope

- HTML, XML, JSON, CSV, and shell-oriented escaping helpers
- lightweight unescaping for common named and numeric forms
- simple detection helpers for whether escaping is needed

## Non-Goals

- security-grade shell escaping across all shells
- full HTML, XML, or JSON parsing
- sanitization suitable for security boundaries

## License

Licensed under either of the following, at your option:

- MIT License
- Apache License, Version 2.0
