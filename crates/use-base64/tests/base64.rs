use use_base64::{
    base64_decode, base64_encode, base64_padding_len, base64_url_decode, base64_url_encode,
    is_base64_char, is_base64_url_char, looks_like_base64, normalize_base64_padding,
};

#[test]
fn standard_encoding_works() {
    assert_eq!(base64_encode(b"hello"), "aGVsbG8=");
}

#[test]
fn standard_decoding_works() {
    assert_eq!(base64_decode("aGVsbG8="), Some(b"hello".to_vec()));
}

#[test]
fn url_safe_encoding_works() {
    assert_eq!(base64_url_encode(&[0xfb, 0xff]), "-_8=");
}

#[test]
fn url_safe_decoding_works() {
    assert_eq!(base64_url_decode("-_8="), Some(vec![0xfb, 0xff]));
}

#[test]
fn padding_normalization_works() {
    assert_eq!(normalize_base64_padding("aGVsbG8"), "aGVsbG8=");
    assert_eq!(base64_padding_len("aGVsbG8="), 1);
}

#[test]
fn invalid_input_is_rejected() {
    assert_eq!(base64_decode("!"), None);
    assert!(!looks_like_base64("abcde"));
}

#[test]
fn character_classification_works() {
    assert!(is_base64_char('+'));
    assert!(is_base64_url_char('_'));
    assert!(looks_like_base64("-_8="));
}

#[test]
fn empty_input_is_supported() {
    assert_eq!(base64_encode(b""), "");
    assert_eq!(base64_decode(""), Some(Vec::new()));
}
