#![no_main]

// code taken from risc0/examples, licensed under Apache v2
// Copyright 2023 RISC Zero, Inc.

use k256::{
    ecdsa::{signature::Verifier, Signature, VerifyingKey},
    EncodedPoint,
};
use risc0_zkvm::guest::env;

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
    for _i in 1..=niter {
        verifying_key
            .verify(&message, &signature)
            .expect("ECDSA signature verification failed");
    }

    // Commit to the journal the verifying key and message that was signed.
    env::commit(&(encoded_verifying_key, message));
}
