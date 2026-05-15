#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EscapeKind {
    Html,
    Xml,
    Json,
    Shell,
    Csv,
}

#[must_use]
pub fn escape_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[must_use]
pub fn unescape_html(input: &str) -> String {
    unescape_markup(input)
}

#[must_use]
pub fn escape_xml(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[must_use]
pub fn unescape_xml(input: &str) -> String {
    unescape_markup(input)
}

#[must_use]
pub fn escape_json_string(input: &str) -> String {
    let mut output = String::with_capacity(input.len());

    for character in input.chars() {
        match character {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            '\u{08}' => output.push_str("\\b"),
            '\u{0c}' => output.push_str("\\f"),
            control if control <= '\u{1f}' => {
                output.push_str("\\u");
                output.push_str(&format!("{:04X}", control as u32));
            },
            other => output.push(other),
        }
    }

    output
}

pub fn unescape_json_string(input: &str) -> Option<String> {
    let mut output = String::new();
    let mut chars = input.chars();

    while let Some(character) = chars.next() {
        if character != '\\' {
            output.push(character);
            continue;
        }

        match chars.next()? {
            '\\' => output.push('\\'),
            '"' => output.push('"'),
            '/' => output.push('/'),
            'b' => output.push('\u{08}'),
            'f' => output.push('\u{0c}'),
            'n' => output.push('\n'),
            'r' => output.push('\r'),
            't' => output.push('\t'),
            'u' => output.push(decode_json_codepoint(&mut chars)?),
            _ => return None,
        }
    }

    Some(output)
}

#[must_use]
pub fn escape_csv_field(input: &str) -> String {
    if !needs_csv_escape(input) {
        return input.to_string();
    }

    format!("\"{}\"", input.replace('"', "\"\""))
}

#[must_use]
pub fn escape_shell_basic(input: &str) -> String {
    if input.is_empty() {
        return "''".to_string();
    }

    format!("'{}'", input.replace('\'', "'\"'\"'"))
}

#[must_use]
pub fn needs_html_escape(input: &str) -> bool {
    input
        .chars()
        .any(|c| matches!(c, '&' | '<' | '>' | '"' | '\''))
}

#[must_use]
pub fn needs_json_escape(input: &str) -> bool {
    input
        .chars()
        .any(|c| matches!(c, '\\' | '"' | '\n' | '\r' | '\t'))
}

#[must_use]
pub fn needs_csv_escape(input: &str) -> bool {
    input.chars().any(|c| matches!(c, ',' | '"' | '\n' | '\r'))
}

fn unescape_markup(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut index = 0;

    while index < input.len() {
        if input.as_bytes()[index] != b'&' {
            let Some(character) = input[index..].chars().next() else {
                break;
            };
            output.push(character);
            index += character.len_utf8();
            continue;
        }

        let rest = &input[index + 1..];
        if let Some(entity_end) = rest.find(';') {
            let entity = &rest[..entity_end];
            if let Some(decoded) = decode_entity(entity) {
                output.push(decoded);
                index += entity_end + 2;
                continue;
            }
        }

        output.push('&');
        index += 1;
    }

    output
}

fn decode_entity(entity: &str) -> Option<char> {
    match entity {
        "amp" => Some('&'),
        "lt" => Some('<'),
        "gt" => Some('>'),
        "quot" => Some('"'),
        "apos" => Some('\''),
        _ => {
            if let Some(hexadecimal) = entity
                .strip_prefix("#x")
                .or_else(|| entity.strip_prefix("#X"))
            {
                u32::from_str_radix(hexadecimal, 16)
                    .ok()
                    .and_then(char::from_u32)
            } else if let Some(decimal) = entity.strip_prefix('#') {
                decimal.parse::<u32>().ok().and_then(char::from_u32)
            } else {
                None
            }
        },
    }
}

fn decode_json_codepoint(chars: &mut std::str::Chars<'_>) -> Option<char> {
    let high = parse_json_hex_quad(chars)?;

    if (0xD800..=0xDBFF).contains(&high) {
        if chars.next()? != '\\' || chars.next()? != 'u' {
            return None;
        }

        let low = parse_json_hex_quad(chars)?;
        if !(0xDC00..=0xDFFF).contains(&low) {
            return None;
        }

        let codepoint = 0x10000 + (((high - 0xD800) as u32) << 10) + u32::from(low - 0xDC00);
        return char::from_u32(codepoint);
    }

    if (0xDC00..=0xDFFF).contains(&high) {
        return None;
    }

    char::from_u32(u32::from(high))
}

fn parse_json_hex_quad(chars: &mut std::str::Chars<'_>) -> Option<u16> {
    let mut value = 0_u16;

    for _ in 0..4 {
        value = (value << 4) | u16::from(chars.next()?.to_digit(16)? as u8);
    }

    Some(value)
}
