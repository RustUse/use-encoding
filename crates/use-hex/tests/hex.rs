use use_hex::{
    HexCase, ensure_hex_prefix, hex_decode, hex_encode, hex_encode_upper, is_hex, is_hex_char,
    normalize_hex, strip_hex_prefix,
};

#[test]
fn encodes_lowercase_hex() {
    assert_eq!(hex_encode(b"\xff\x00\xaa"), "ff00aa");
}

#[test]
fn encodes_uppercase_hex() {
    assert_eq!(hex_encode_upper(b"\xff\x00\xaa"), "FF00AA");
}

#[test]
fn decodes_hex_input() {
    assert_eq!(hex_decode("0xff00aa"), Some(vec![0xff, 0x00, 0xaa]));
}

#[test]
fn strips_prefixes() {
    assert_eq!(strip_hex_prefix("0xff00aa"), "ff00aa");
    assert_eq!(strip_hex_prefix("#ff00aa"), "ff00aa");
}

#[test]
fn ensures_prefix() {
    assert_eq!(ensure_hex_prefix("ff00aa"), "0xff00aa");
}

#[test]
fn normalizes_hex() {
    assert_eq!(
        normalize_hex("#FF00AA", HexCase::Lower),
        Some("ff00aa".to_string())
    );
    assert_eq!(
        normalize_hex("0xff00aa", HexCase::Upper),
        Some("FF00AA".to_string())
    );
}

#[test]
fn rejects_invalid_hex() {
    assert_eq!(hex_decode("xyz"), None);
    assert!(!is_hex("xyz"));
    assert!(!is_hex_char('z'));
}

#[test]
fn empty_input_is_supported() {
    assert_eq!(hex_encode(b""), "");
    assert_eq!(hex_decode(""), Some(Vec::new()));
    assert!(is_hex(""));
}
