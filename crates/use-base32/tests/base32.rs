use use_base32::{
    base32_decode, base32_encode, base32_padding_len, is_base32_char, looks_like_base32,
    normalize_base32_padding,
};

#[test]
fn encoding_works() {
    assert_eq!(base32_encode(b"foo"), "MZXW6===");
}

#[test]
fn decoding_works() {
    assert_eq!(base32_decode("MZXW6==="), Some(b"foo".to_vec()));
}

#[test]
fn padding_normalization_works() {
    assert_eq!(normalize_base32_padding("MZXW6"), "MZXW6===");
    assert_eq!(base32_padding_len("MZXW6==="), 3);
}

#[test]
fn invalid_input_is_rejected() {
    assert_eq!(base32_decode("1"), None);
    assert!(!looks_like_base32("1"));
}

#[test]
fn character_classification_works() {
    assert!(is_base32_char('M'));
    assert!(is_base32_char('m'));
}

#[test]
fn empty_input_is_supported() {
    assert_eq!(base32_encode(b""), "");
    assert_eq!(base32_decode(""), Some(Vec::new()));
}
