# Contributing

RustUse/use-encoding is intentionally small and focused. Contributions should
keep the public API explicit, pragmatic, dependency-light, and easy to compose.

## Development flow

1. Make the smallest useful change.
2. Add or update tests for public behavior changes.
3. Prefer direct helpers over broad framework abstractions.
4. Keep public docs aligned with the actual crate behavior, metadata, and
   release surface.

## Local validation

```sh
cargo fmt --all -- --check
cargo check --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

## Scope guidance

- `use-encoding` is the RustUse encoding-and-escaping primitives workspace, not
  a full parser suite and not a cryptography crate.
- Prefer small reusable helpers over framework-heavy wrappers.
- Keep examples and README usage aligned with the current crate surface.
- Avoid widening the API surface without a clear crate-level use case.
