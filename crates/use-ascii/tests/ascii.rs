use use_ascii::{
    AsciiKind, ascii_kind, contains_non_ascii, is_ascii, is_ascii_alpha, is_ascii_alphanumeric,
    is_ascii_numeric, replace_non_ascii, strip_non_ascii,
};

#[test]
fn detects_ascii() {
    assert!(is_ascii("plain-text"));
    assert!(!is_ascii("café"));
}

#[test]
fn detects_alphabetic_ascii() {
    assert!(is_ascii_alpha("Rust"));
    assert!(!is_ascii_alpha("Rust1"));
}

#[test]
fn detects_numeric_ascii() {
    assert!(is_ascii_numeric("12345"));
    assert!(!is_ascii_numeric("12a45"));
}

#[test]
fn detects_alphanumeric_ascii() {
    assert!(is_ascii_alphanumeric("Rust2024"));
    assert!(!is_ascii_alphanumeric("Rust 2024"));
}

#[test]
fn detects_non_ascii() {
    assert!(contains_non_ascii("café"));
    assert!(!contains_non_ascii("cafe"));
}

#[test]
fn strips_non_ascii() {
    assert_eq!(strip_non_ascii("café"), "caf");
}

#[test]
fn replaces_non_ascii() {
    assert_eq!(replace_non_ascii("café", '?'), "caf?");
}

#[test]
fn classifies_ascii_characters() {
    assert_eq!(ascii_kind('A'), AsciiKind::Alphabetic);
    assert_eq!(ascii_kind('5'), AsciiKind::Numeric);
    assert_eq!(ascii_kind(' '), AsciiKind::Whitespace);
    assert_eq!(ascii_kind('\u{7f}'), AsciiKind::Control);
    assert_eq!(ascii_kind('!'), AsciiKind::Punctuation);
    assert_eq!(ascii_kind('é'), AsciiKind::Other);
}

#[test]
fn empty_input_is_supported() {
    assert!(is_ascii(""));
    assert!(!is_ascii_alpha(""));
    assert_eq!(strip_non_ascii(""), "");
}
