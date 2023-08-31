// #![no_std]
#![no_main]

use risc0_zkvm::guest::env;
use risc0_zkvm::sha::{Impl, Sha256};

risc0_zkvm::entry!(main);

pub fn main() {
    let num_iter: u32 = env::read();
    let data: Vec<u8> = env::read();

    let mut digest = Impl::hash_bytes(data.as_slice());

    // Splicing a trivial increment allows for the zk prover to properly
    // segment the computation; otherwise it fails for high values of [num_iter].
    let mut c = 0;

    for _ in 1..num_iter {
        c = std::hint::black_box(c + 1);
        digest = Impl::hash_bytes(digest.as_bytes());
    }

    env::commit(&digest)
}
