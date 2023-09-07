#![no_main]

use risc0_zkvm::guest::env;
use risc0_zkvm::sha::{Impl, Sha256};

risc0_zkvm::guest::entry!(main);

fn pow(n: u64) -> u64 {
    let mut acc = 1;
    for _i in 1..=n {
        acc *= 3;
    }
    acc
}
pub fn main() {
    let a: u64 = env::read();
    let to_hash = vec![0_u8; 20];
    let mut digest = Impl::hash_bytes(to_hash.as_slice());
    let result = pow(a);
    env::commit(&result);
}
