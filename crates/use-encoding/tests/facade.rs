#[cfg(feature = "percent")]
use use_encoding::percent::percent_encode_component;

#[cfg(feature = "hex")]
use use_encoding::hex::{HexCase, normalize_hex};

#[test]
fn facade_builds_without_features() {}

#[cfg(all(feature = "percent", feature = "hex"))]
#[test]
fn facade_reexports_enabled_modules() {
    assert_eq!(percent_encode_component("hello world"), "hello%20world");
    assert_eq!(
        normalize_hex("#FF00AA", HexCase::Lower),
        Some("ff00aa".to_string())
    );
}
