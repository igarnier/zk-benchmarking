#![no_main]

// code taken from risc0/examples, licensed under Apache v2
// Copyright 2023 RISC Zero, Inc.

use k256::{
    ecdsa::{signature::Verifier, Signature, VerifyingKey},
    EncodedPoint,
};
use risc0_zkvm::guest::env;
use risc0_zkvm::sha::{Impl, Sha256};

risc0_zkvm::guest::entry!(main);

fn main() {
    // Decode the verifying key, message, and signature from the inputs.
    let (encoded_verifying_key, message, signature, niter): (
        EncodedPoint,
        Vec<u8>,
        Signature,
        u32,
    ) = env::read();
    let verifying_key = VerifyingKey::from_encoded_point(&encoded_verifying_key).unwrap();

    // Verify the signature, panicking if verification fails.
    verifying_key
        .verify(&message, &signature)
        .expect("ECDSA signature verification failed");

    let pair_of_roots = vec![0_u8; 512];

    // Splicing a trivial increment allows for the zk prover to properly
    // segment the computation; otherwise it fails for high values of [num_iter].
    let mut c = 0;

    for _i in 0..niter {
        c = c + 1;
        let _ = Impl::hash_bytes(pair_of_roots.as_slice());
    }
}
