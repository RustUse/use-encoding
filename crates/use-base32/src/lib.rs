#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Base32Variant {
    Standard,
    NoPadding,
}

#[must_use]
pub fn base32_encode(input: &[u8]) -> String {
    encode_base32(input, Base32Variant::Standard)
}

pub fn base32_decode(input: &str) -> Option<Vec<u8>> {
    let normalized = normalize_base32_input(input)?;
    let trimmed = normalized.trim_end_matches('=');
    let mut output = Vec::with_capacity((trimmed.len() * 5) / 8);
    let mut buffer = 0_u32;
    let mut bits = 0_u8;

    for character in trimmed.chars() {
        let value = u32::from(decode_base32_char(character)?);
        buffer = (buffer << 5) | value;
        bits += 5;

        while bits >= 8 {
            bits -= 8;
            output.push(((buffer >> bits) & 0xff) as u8);
        }
    }

    if bits > 0 && (buffer & ((1_u32 << bits) - 1)) != 0 {
        return None;
    }

    Some(output)
}

#[must_use]
pub fn looks_like_base32(input: &str) -> bool {
    normalize_base32_input(input).is_some()
        && input.trim_end_matches('=').chars().all(is_base32_char)
}

#[must_use]
pub fn is_base32_char(c: char) -> bool {
    matches!(c, 'A'..='Z' | 'a'..='z' | '2'..='7')
}

#[must_use]
pub fn base32_padding_len(input: &str) -> usize {
    input.chars().rev().take_while(|&c| c == '=').count()
}

#[must_use]
pub fn normalize_base32_padding(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }

    if let Some(position) = input.find('=') {
        let suffix = &input[position..];
        if input.len().is_multiple_of(8)
            && matches!(suffix.len(), 1 | 3 | 4 | 6)
            && suffix.chars().all(|c| c == '=')
        {
            return input.to_string();
        }

        return input.to_string();
    }

    match input.len() % 8 {
        0 => input.to_string(),
        2 => format!("{input}======"),
        4 => format!("{input}===="),
        5 => format!("{input}==="),
        7 => format!("{input}="),
        _ => input.to_string(),
    }
}

fn encode_base32(input: &[u8], variant: Base32Variant) -> String {
    let mut output = String::with_capacity(((input.len() * 8).div_ceil(5)).max(1));
    let mut buffer = 0_u32;
    let mut bits = 0_u8;

    for byte in input {
        buffer = (buffer << 8) | u32::from(*byte);
        bits += 8;

        while bits >= 5 {
            bits -= 5;
            output.push(BASE32_ALPHABET[((buffer >> bits) & 0x1f) as usize] as char);
        }
    }

    if bits > 0 {
        output.push(BASE32_ALPHABET[((buffer << (5 - bits)) & 0x1f) as usize] as char);
    }

    if matches!(variant, Base32Variant::Standard) {
        while !output.len().is_multiple_of(8) {
            output.push('=');
        }
    }

    output
}

fn normalize_base32_input(input: &str) -> Option<String> {
    if !input.is_ascii() {
        return None;
    }

    if input.is_empty() {
        return Some(String::new());
    }

    if let Some(position) = input.find('=') {
        let suffix = &input[position..];
        if !input.len().is_multiple_of(8)
            || !matches!(suffix.len(), 1 | 3 | 4 | 6)
            || !suffix.chars().all(|c| c == '=')
        {
            return None;
        }

        return Some(input.to_string());
    }

    match input.len() % 8 {
        0 => Some(input.to_string()),
        2 => Some(format!("{input}======")),
        4 => Some(format!("{input}====")),
        5 => Some(format!("{input}===")),
        7 => Some(format!("{input}=")),
        _ => None,
    }
}

fn decode_base32_char(character: char) -> Option<u8> {
    match character {
        'A'..='Z' => Some(character as u8 - b'A'),
        'a'..='z' => Some(character as u8 - b'a'),
        '2'..='7' => Some(character as u8 - b'2' + 26),
        _ => None,
    }
}

const BASE32_ALPHABET: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
