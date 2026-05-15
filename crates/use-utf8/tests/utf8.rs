use use_utf8::{
    Utf8Stats, byte_len, char_len, is_valid_utf8, safe_char_boundary, truncate_utf8,
    truncate_utf8_bytes, utf8_lossy, utf8_stats,
};

#[test]
fn detects_valid_utf8() {
    assert!(is_valid_utf8("café".as_bytes()));
    assert!(!is_valid_utf8(b"hi\xff"));
}

#[test]
fn converts_lossy_utf8() {
    assert_eq!(utf8_lossy(b"hi\xff"), "hi\u{fffd}");
}

#[test]
fn counts_bytes() {
    assert_eq!(byte_len("café"), 5);
}

#[test]
fn counts_chars() {
    assert_eq!(char_len("café"), 4);
}

#[test]
fn computes_stats() {
    assert_eq!(
        utf8_stats("rust"),
        Utf8Stats {
            bytes: 4,
            chars: 4,
            is_ascii: true,
        }
    );
}

#[test]
fn truncates_by_chars() {
    assert_eq!(truncate_utf8("café", 3), "caf");
}

#[test]
fn truncates_by_safe_byte_boundary() {
    assert_eq!(truncate_utf8_bytes("café", 4), "caf");
    assert_eq!(safe_char_boundary("café", 4), 3);
}

#[test]
fn empty_input_is_supported() {
    assert_eq!(truncate_utf8("", 1), "");
    assert_eq!(truncate_utf8_bytes("", 1), "");
}
