#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HexCase {
    Lower,
    Upper,
}

#[must_use]
pub fn hex_encode(input: &[u8]) -> String {
    encode_hex(input, HexCase::Lower)
}

#[must_use]
pub fn hex_encode_upper(input: &[u8]) -> String {
    encode_hex(input, HexCase::Upper)
}

pub fn hex_decode(input: &str) -> Option<Vec<u8>> {
    let value = strip_hex_prefix(input);
    if !value.len().is_multiple_of(2) || !value.chars().all(is_hex_char) {
        return None;
    }

    let mut output = Vec::with_capacity(value.len() / 2);
    let bytes = value.as_bytes();
    let mut index = 0;

    while index < bytes.len() {
        let high = decode_nibble(bytes[index])?;
        let low = decode_nibble(bytes[index + 1])?;
        output.push((high << 4) | low);
        index += 2;
    }

    Some(output)
}

#[must_use]
pub fn is_hex(input: &str) -> bool {
    let value = strip_hex_prefix(input);
    value.len().is_multiple_of(2) && value.chars().all(is_hex_char)
}

#[must_use]
pub fn is_hex_char(c: char) -> bool {
    c.is_ascii_hexdigit()
}

#[must_use]
pub fn strip_hex_prefix(input: &str) -> &str {
    input
        .strip_prefix("0x")
        .or_else(|| input.strip_prefix("0X"))
        .or_else(|| input.strip_prefix('#'))
        .unwrap_or(input)
}

#[must_use]
pub fn ensure_hex_prefix(input: &str) -> String {
    format!("0x{}", strip_hex_prefix(input))
}

pub fn normalize_hex(input: &str, case: HexCase) -> Option<String> {
    let value = strip_hex_prefix(input);
    if !value.chars().all(is_hex_char) {
        return None;
    }

    Some(match case {
        HexCase::Lower => value.to_ascii_lowercase(),
        HexCase::Upper => value.to_ascii_uppercase(),
    })
}

fn encode_hex(input: &[u8], case: HexCase) -> String {
    let digits = match case {
        HexCase::Lower => b"0123456789abcdef",
        HexCase::Upper => b"0123456789ABCDEF",
    };
    let mut output = String::with_capacity(input.len() * 2);

    for byte in input {
        output.push(digits[(byte >> 4) as usize] as char);
        output.push(digits[(byte & 0x0f) as usize] as char);
    }

    output
}

fn decode_nibble(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}
