// #![no_std]
#![no_main]

use risc0_zkvm::guest::env;
use risc0_zkvm::sha::{Impl, Sha256};

risc0_zkvm::entry!(main);

pub fn main() {
    let data: Vec<u8> = env::read();

    let digest = Impl::hash_bytes(data.as_slice());
    env::commit(&digest.as_bytes());
}
