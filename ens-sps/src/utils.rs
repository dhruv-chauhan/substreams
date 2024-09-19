#[allow(dead_code)]
use tiny_keccak::{Hasher, Keccak};

pub fn keccak256(data: Vec<u8>) -> [u8; 32] {
    let mut keccak = Keccak::v256();
    let mut output = [0u8; 32];
    keccak.update(data.as_slice());
    keccak.finalize(&mut output);
    output
}
