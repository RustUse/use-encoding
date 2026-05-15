#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Utf8Stats {
    pub bytes: usize,
    pub chars: usize,
    pub is_ascii: bool,
}

#[must_use]
pub fn is_valid_utf8(bytes: &[u8]) -> bool {
    std::str::from_utf8(bytes).is_ok()
}

#[must_use]
pub fn utf8_lossy(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

#[must_use]
pub fn utf8_stats(input: &str) -> Utf8Stats {
    Utf8Stats {
        bytes: input.len(),
        chars: input.chars().count(),
        is_ascii: input.is_ascii(),
    }
}

#[must_use]
pub fn byte_len(input: &str) -> usize {
    input.len()
}

#[must_use]
pub fn char_len(input: &str) -> usize {
    input.chars().count()
}

#[must_use]
pub fn truncate_utf8(input: &str, max_chars: usize) -> String {
    input.chars().take(max_chars).collect()
}

#[must_use]
pub fn truncate_utf8_bytes(input: &str, max_bytes: usize) -> String {
    let boundary = safe_char_boundary(input, max_bytes);
    input[..boundary].to_string()
}

#[must_use]
pub fn safe_char_boundary(input: &str, index: usize) -> usize {
    if index >= input.len() {
        return input.len();
    }

    let mut boundary = index;
    while boundary > 0 && !input.is_char_boundary(boundary) {
        boundary -= 1;
    }
    boundary
}
