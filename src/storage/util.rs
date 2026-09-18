use std::random::random;

pub trait HasId {
    fn id(&self) -> u32;
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

pub fn hash_password(password: &str) {
    pad_256(password.as_bytes());
}

// Original bytes
// Marker -> 1 bytes
// Zeros
// Original -> 8 bytes
// Total -> a multiple of 64 bytes
pub fn pad_256(input: &[u8]) -> Vec<u8> {
    let input_length = (input.len() as u64) * 8;
    let mut bytes = Vec::new();
    for byte in input {
        bytes.push(*byte);
    }
    bytes.push(0x80);
    while bytes.len() % 64 != 56 {
        bytes.push(0);
    }
    for byte in input_length.to_be_bytes() {
        bytes.push(byte);
    }
    bytes
}

// convert 64 bytes to 16 words
fn block_to_words(block: &[u8; 64]) -> [u32; 16] {
    std::array::from_fn(|i| u32::from_be_bytes(block[i * 4..i * 4 + 4].try_into().unwrap()))
}

// rotate x right by 7
// XOR rotate x right by 18
// XOR shift x right by 3
fn small_sigma0(x: u32) -> u32 {
    x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3)
}

// rotate x right by 17
// XOR rotate x right by 19
// XOR shift x right by 10
fn small_sigma1(x: u32) -> u32 {
    x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10)
}