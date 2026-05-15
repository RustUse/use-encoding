#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsciiKind {
    Alphabetic,
    Numeric,
    Alphanumeric,
    Whitespace,
    Control,
    Punctuation,
    Other,
}

#[must_use]
pub fn is_ascii(input: &str) -> bool {
    input.is_ascii()
}

#[must_use]
pub fn is_ascii_alpha(input: &str) -> bool {
    !input.is_empty()
        && input
            .chars()
            .all(|character| character.is_ascii_alphabetic())
}

#[must_use]
pub fn is_ascii_numeric(input: &str) -> bool {
    !input.is_empty() && input.chars().all(|character| character.is_ascii_digit())
}

#[must_use]
pub fn is_ascii_alphanumeric(input: &str) -> bool {
    !input.is_empty()
        && input
            .chars()
            .all(|character| character.is_ascii_alphanumeric())
}

#[must_use]
pub fn is_ascii_whitespace_only(input: &str) -> bool {
    !input.is_empty()
        && input
            .chars()
            .all(|character| character.is_ascii_whitespace())
}

#[must_use]
pub fn contains_non_ascii(input: &str) -> bool {
    !input.is_ascii()
}

#[must_use]
pub fn strip_non_ascii(input: &str) -> String {
    input
        .chars()
        .filter(|character| character.is_ascii())
        .collect()
}

#[must_use]
pub fn replace_non_ascii(input: &str, replacement: char) -> String {
    input
        .chars()
        .map(|character| {
            if character.is_ascii() {
                character
            } else {
                replacement
            }
        })
        .collect()
}

#[must_use]
pub fn ascii_kind(c: char) -> AsciiKind {
    if !c.is_ascii() {
        return AsciiKind::Other;
    }

    if c.is_ascii_alphabetic() {
        return AsciiKind::Alphabetic;
    }

    if c.is_ascii_digit() {
        return AsciiKind::Numeric;
    }

    if c.is_ascii_alphanumeric() {
        return AsciiKind::Alphanumeric;
    }

    if c.is_ascii_whitespace() {
        return AsciiKind::Whitespace;
    }

    if c.is_ascii_control() {
        return AsciiKind::Control;
    }

    if c.is_ascii_punctuation() {
        return AsciiKind::Punctuation;
    }

    AsciiKind::Other
}
