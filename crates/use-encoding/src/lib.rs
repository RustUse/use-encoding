#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "ascii")]
pub use use_ascii as ascii;

#[cfg(feature = "base32")]
pub use use_base32 as base32;

#[cfg(feature = "base64")]
pub use use_base64 as base64;

#[cfg(feature = "escape")]
pub use use_escape as escape;

#[cfg(feature = "hex")]
pub use use_hex as hex;

#[cfg(feature = "percent")]
pub use use_percent as percent;

#[cfg(feature = "utf8")]
pub use use_utf8 as utf8;

#[cfg(test)]
mod tests {
    #[cfg(feature = "percent")]
    use super::percent::percent_encode_component;

    #[cfg(feature = "hex")]
    use super::hex::{HexCase, normalize_hex};

    #[test]
    fn compiles_without_features() {}

    #[cfg(feature = "percent")]
    #[test]
    fn percent_feature_reexports_helpers() {
        assert_eq!(percent_encode_component("hello world"), "hello%20world");
    }

    #[cfg(feature = "hex")]
    #[test]
    fn hex_feature_reexports_helpers() {
        assert_eq!(
            normalize_hex("#FF00AA", HexCase::Lower),
            Some("ff00aa".to_string())
        );
    }
}
