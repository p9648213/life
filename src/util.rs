use crate::{
    sha256::{digest_to_hex, sha256},
    http::error::HttpError,
};
use std::random::random;

pub fn escape_html(text: &str, out: &mut String) {
    for ch in text.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '\'' => out.push_str("&#x27;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(ch),
        }
    }
}

fn decode_hex(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

pub fn decode_form(input: &[u8]) -> Result<String, HttpError> {
    let mut decoded = Vec::with_capacity(input.len());
    let mut index = 0;
    while index < input.len() {
        match input[index] {
            b'+' => {
                decoded.push(b' ');
                index += 1;
            }
            b'%' => {
                let high = input
                    .get(index + 1)
                    .and_then(|byte| decode_hex(*byte))
                    .ok_or(HttpError::FormParseError)?;
                let low = input
                    .get(index + 2)
                    .and_then(|byte| decode_hex(*byte))
                    .ok_or(HttpError::FormParseError)?;
                // Hex 2B = 2 * 16^1 + B * 16^0 = 32 + 11 * 2^0 = 43(+)
                decoded.push(high * 16 + low);
                index += 3;
            }
            byte => {
                decoded.push(byte);
                index += 1;
            }
        }
    }
    String::from_utf8(decoded).map_err(|_| HttpError::FormParseError)
}

pub fn create_session() -> String {
    let bits: u128 = random(..);
    let g1 = (bits >> 96) as u32;
    let g2 = (bits >> 80) as u16;
    let g3 = (0x4000 | (bits >> 64) & 0x0fff) as u16;
    let g4 = (0x8000 | (bits >> 48) & 0x3fff) as u16;
    let g5 = (bits & 0xffffffffffff) as u64;
    format!("{g1:08x}-{g2:04x}-{g3:04x}-{g4:04x}-{g5:012x}")
}

pub fn hash_password(password: &str) -> String {
    let digest = sha256(password.as_bytes());
    digest_to_hex(&digest)
}
