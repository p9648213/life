use crate::sha256::{digest_to_hex, sha256};
use std::random::random;

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
