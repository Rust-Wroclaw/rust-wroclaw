#![allow(unconditional_panic)]

pub fn checksum(v: [u8; 32]) -> bool {
    (v[32] + v[0]) % 64 == 0
}
