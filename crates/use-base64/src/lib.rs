#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Base64Variant {
    Standard,
    UrlSafe,
    StandardNoPadding,
    UrlSafeNoPadding,
}

#[must_use]
pub fn base64_encode(input: &[u8]) -> String {
    encode_base64(input, Base64Variant::Standard)
}

pub fn base64_decode(input: &str) -> Option<Vec<u8>> {
    decode_base64(input, false)
}

#[must_use]
pub fn base64_url_encode(input: &[u8]) -> String {
    encode_base64(input, Base64Variant::UrlSafe)
}

pub fn base64_url_decode(input: &str) -> Option<Vec<u8>> {
    decode_base64(input, true)
}

#[must_use]
pub fn looks_like_base64(input: &str) -> bool {
    normalize_base64_input(input).is_some()
        && input
            .trim_end_matches('=')
            .chars()
            .all(|c| is_base64_char(c) || is_base64_url_char(c))
}

#[must_use]
pub fn is_base64_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, '+' | '/')
}

#[must_use]
pub fn is_base64_url_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, '-' | '_')
}

#[must_use]
pub fn base64_padding_len(input: &str) -> usize {
    input.chars().rev().take_while(|&c| c == '=').count()
}

#[must_use]
pub fn normalize_base64_padding(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }

    if let Some(position) = input.find('=') {
        let suffix = &input[position..];
        if suffix.chars().all(|c| c == '=') && input.len().is_multiple_of(4) && suffix.len() <= 2 {
            return input.to_string();
        }

        return input.to_string();
    }

    match input.len() % 4 {
        0 => input.to_string(),
        2 => format!("{input}=="),
        3 => format!("{input}="),
        _ => input.to_string(),
    }
}

fn encode_base64(input: &[u8], variant: Base64Variant) -> String {
    let (alphabet, padding) = match variant {
        Base64Variant::Standard => (STANDARD_ALPHABET, true),
        Base64Variant::UrlSafe => (URL_SAFE_ALPHABET, true),
        Base64Variant::StandardNoPadding => (STANDARD_ALPHABET, false),
        Base64Variant::UrlSafeNoPadding => (URL_SAFE_ALPHABET, false),
    };
    let mut output = String::with_capacity(input.len().div_ceil(3) * 4);

    for chunk in input.chunks(3) {
        let first = chunk[0];
        let second = chunk.get(1).copied().unwrap_or(0);
        let third = chunk.get(2).copied().unwrap_or(0);
        let combined = (u32::from(first) << 16) | (u32::from(second) << 8) | u32::from(third);

        output.push(alphabet[((combined >> 18) & 0x3f) as usize] as char);
        output.push(alphabet[((combined >> 12) & 0x3f) as usize] as char);

        if chunk.len() > 1 {
            output.push(alphabet[((combined >> 6) & 0x3f) as usize] as char);
        } else if padding {
            output.push('=');
        }

        if chunk.len() > 2 {
            output.push(alphabet[(combined & 0x3f) as usize] as char);
        } else if padding {
            output.push('=');
        }
    }

    output
}

fn decode_base64(input: &str, url_safe: bool) -> Option<Vec<u8>> {
    let normalized = normalize_base64_input(input)?;
    let bytes = normalized.as_bytes();
    let chunk_count = bytes.len() / 4;
    let mut output = Vec::with_capacity((bytes.len() / 4) * 3);

    for (chunk_index, chunk) in bytes.chunks(4).enumerate() {
        let last_chunk = chunk_index + 1 == chunk_count;
        let third_is_padding = chunk[2] == b'=';
        let fourth_is_padding = chunk[3] == b'=';

        if third_is_padding && !fourth_is_padding {
            return None;
        }

        if (third_is_padding || fourth_is_padding) && !last_chunk {
            return None;
        }

        let first = u32::from(decode_base64_byte(chunk[0], url_safe)?);
        let second = u32::from(decode_base64_byte(chunk[1], url_safe)?);
        let third = if third_is_padding {
            0
        } else {
            u32::from(decode_base64_byte(chunk[2], url_safe)?)
        };
        let fourth = if fourth_is_padding {
            0
        } else {
            u32::from(decode_base64_byte(chunk[3], url_safe)?)
        };

        let combined = (first << 18) | (second << 12) | (third << 6) | fourth;
        output.push(((combined >> 16) & 0xff) as u8);
        if !third_is_padding {
            output.push(((combined >> 8) & 0xff) as u8);
        }
        if !fourth_is_padding {
            output.push((combined & 0xff) as u8);
        }
    }

    Some(output)
}

fn normalize_base64_input(input: &str) -> Option<String> {
    if !input.is_ascii() {
        return None;
    }

    if input.is_empty() {
        return Some(String::new());
    }

    if let Some(position) = input.find('=') {
        let suffix = &input[position..];
        if !input.len().is_multiple_of(4) || suffix.len() > 2 || !suffix.chars().all(|c| c == '=') {
            return None;
        }

        return Some(input.to_string());
    }

    match input.len() % 4 {
        0 => Some(input.to_string()),
        2 => Some(format!("{input}==")),
        3 => Some(format!("{input}=")),
        _ => None,
    }
}

fn decode_base64_byte(byte: u8, url_safe: bool) -> Option<u8> {
    match byte {
        b'A'..=b'Z' => Some(byte - b'A'),
        b'a'..=b'z' => Some(byte - b'a' + 26),
        b'0'..=b'9' => Some(byte - b'0' + 52),
        b'+' if !url_safe => Some(62),
        b'/' if !url_safe => Some(63),
        b'-' if url_safe => Some(62),
        b'_' if url_safe => Some(63),
        _ => None,
    }
}

const STANDARD_ALPHABET: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const URL_SAFE_ALPHABET: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
