// #![no_std]
#![no_main]

use risc0_zkvm::guest::env;
use risc0_zkvm::sha::{Impl, Sha256};

risc0_zkvm::entry!(main);

pub fn main() {
    let num_iter: u32 = env::read();
    let data: Vec<u8> = env::read();

    let mut digest = Impl::hash_bytes(data.as_slice());

    for _ in 1..num_iter {
        digest = Impl::hash_bytes(digest.as_bytes());
    }

    env::commit(&digest)
}
