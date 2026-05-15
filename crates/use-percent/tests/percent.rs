use use_percent::{
    contains_percent_encoded, has_invalid_percent_encoding, is_percent_encoded, percent_decode,
    percent_decode_component, percent_encode, percent_encode_component,
};

#[test]
fn percent_encodes_input() {
    assert_eq!(percent_encode("hello world"), "hello%20world");
}

#[test]
fn percent_decodes_input() {
    assert_eq!(
        percent_decode("hello%20world"),
        Some("hello world".to_string())
    );
}

#[test]
fn component_encoding_matches_default() {
    assert_eq!(percent_encode_component("a/b c"), "a%2Fb%20c");
}

#[test]
fn component_decoding_round_trips() {
    assert_eq!(
        percent_decode_component("a%2Fb%20c"),
        Some("a/b c".to_string())
    );
}

#[test]
fn invalid_percent_encoding_is_detected() {
    assert_eq!(percent_decode("bad%2"), None);
    assert!(has_invalid_percent_encoding("bad%2"));
    assert!(contains_percent_encoded("good%20bad%2"));
    assert!(!is_percent_encoded("bad%2"));
}

#[test]
fn empty_input_is_supported() {
    assert_eq!(percent_encode(""), "");
    assert_eq!(percent_decode(""), Some(String::new()));
    assert!(!contains_percent_encoded(""));
}
