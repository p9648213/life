use super::error::HttpError;

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
