#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PercentEncodeSet {
    UrlComponent,
    PathSegment,
    QueryComponent,
    Fragment,
}

#[must_use]
pub fn percent_encode(input: &str) -> String {
    percent_encode_with_set(input, PercentEncodeSet::UrlComponent)
}

pub fn percent_decode(input: &str) -> Option<String> {
    percent_decode_internal(input)
}

#[must_use]
pub fn percent_encode_component(input: &str) -> String {
    percent_encode_with_set(input, PercentEncodeSet::UrlComponent)
}

pub fn percent_decode_component(input: &str) -> Option<String> {
    percent_decode_internal(input)
}

#[must_use]
pub fn is_percent_encoded(input: &str) -> bool {
    contains_percent_encoded(input) && !has_invalid_percent_encoding(input)
}

#[must_use]
pub fn contains_percent_encoded(input: &str) -> bool {
    let bytes = input.as_bytes();
    let mut index = 0;

    while index + 2 < bytes.len() {
        if bytes[index] == b'%' && is_hex_byte(bytes[index + 1]) && is_hex_byte(bytes[index + 2]) {
            return true;
        }

        index += 1;
    }

    false
}

#[must_use]
pub fn has_invalid_percent_encoding(input: &str) -> bool {
    let bytes = input.as_bytes();
    let mut index = 0;

    while index < bytes.len() {
        if bytes[index] == b'%' {
            if index + 2 >= bytes.len()
                || !is_hex_byte(bytes[index + 1])
                || !is_hex_byte(bytes[index + 2])
            {
                return true;
            }

            index += 3;
            continue;
        }

        index += 1;
    }

    false
}

fn percent_encode_with_set(input: &str, set: PercentEncodeSet) -> String {
    let mut output = String::with_capacity(input.len());

    for byte in input.bytes() {
        if should_encode(byte, set) {
            output.push('%');
            output.push(HEX[(byte >> 4) as usize] as char);
            output.push(HEX[(byte & 0x0f) as usize] as char);
        } else {
            output.push(byte as char);
        }
    }

    output
}

fn percent_decode_internal(input: &str) -> Option<String> {
    let bytes = input.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;

    while index < bytes.len() {
        if bytes[index] == b'%' {
            if index + 2 >= bytes.len() {
                return None;
            }

            let value = decode_hex_pair(bytes[index + 1], bytes[index + 2])?;
            decoded.push(value);
            index += 3;
            continue;
        }

        decoded.push(bytes[index]);
        index += 1;
    }

    String::from_utf8(decoded).ok()
}

fn should_encode(byte: u8, set: PercentEncodeSet) -> bool {
    if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~') {
        return false;
    }

    match set {
        PercentEncodeSet::Fragment => matches!(byte, b' ' | b'"' | b'<' | b'>' | b'`'),
        PercentEncodeSet::PathSegment => true,
        PercentEncodeSet::QueryComponent => true,
        PercentEncodeSet::UrlComponent => true,
    }
}

fn decode_hex_pair(high: u8, low: u8) -> Option<u8> {
    Some((decode_hex_nibble(high)? << 4) | decode_hex_nibble(low)?)
}

fn decode_hex_nibble(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

fn is_hex_byte(byte: u8) -> bool {
    decode_hex_nibble(byte).is_some()
}

const HEX: &[u8; 16] = b"0123456789ABCDEF";
