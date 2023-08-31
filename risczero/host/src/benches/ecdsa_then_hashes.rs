use crate::helpers::inner_receipt_size_bytes;
use k256::{
    ecdsa::{signature::Signer, Signature, SigningKey},
    EncodedPoint,
};
use rand_core::OsRng;
use risc0_zkvm::serde::to_vec;
use risc0_zkvm::sha::DIGEST_WORDS;
use risc0_zkvm::{ExecutorEnv, Receipt};
use rustbench::Benchmark;

#[derive(Clone)]
pub struct Spec {
    encoded_verifying_key: EncodedPoint,
    message: Vec<u8>,
    signature: Signature,
    nhashes: u32,
}

pub struct Job<'a> {
    pub spec: Spec,
    pub env: ExecutorEnv<'a>,
    pub prover: crate::provers::Name,
}

fn gen_spec(nhashes: u32) -> Spec {
    // Generate a random secp256k1 keypair and sign the message.
    let signing_key = SigningKey::random(&mut OsRng);
    let message = b"32 bytes message, including this".to_vec();
    let signature: Signature = signing_key.sign(message.as_slice());
    let verifying_key = signing_key.verifying_key();
    Spec {
        encoded_verifying_key: verifying_key.to_encoded_point(true),
        message,
        signature,
        nhashes,
    }
}

pub fn new_jobs() -> Vec<<Job<'static> as Benchmark>::Spec> {
    [1, 100, 200, 300, 400, 500, 600, 700, 800, 900, 1000]
        .map(gen_spec)
        .to_vec()
}

const METHOD_ID: [u32; DIGEST_WORDS] = risczero_benchmark_methods::ECDSA_THEN_HASHES_ID;
const METHOD_ELF: &[u8] = risczero_benchmark_methods::ECDSA_THEN_HASHES_ELF;

impl Benchmark for Job<'_> {
    const NAME: &'static str = "ecdsa_then_hashes";
    type Spec = Spec;
    type ComputeOut = ();
    type ProofType = Receipt;
    type Prover = crate::provers::Name;

    fn prover_name(&self) -> String {
        self.prover.to_string()
    }

    fn job_size(spec: &Self::Spec) -> u32 {
        spec.nhashes
    }

    fn output_size_bytes(_output: &Self::ComputeOut, proof: &Self::ProofType) -> u32 {
        proof.journal.len() as u32
    }

    fn proof_size_bytes(proof: &Self::ProofType) -> u32 {
        inner_receipt_size_bytes(&proof.inner)
    }

    fn new(spec: &Self::Spec, prover: &Self::Prover) -> Self {
        let env = ExecutorEnv::builder()
            .add_input(
                &to_vec(&(
                    spec.encoded_verifying_key,
                    &spec.message,
                    spec.signature,
                    spec.nhashes,
                ))
                .unwrap(),
            )
            .build()
            .unwrap();

        Job {
            spec: spec.clone(),
            env,
            prover: prover.clone(),
        }
    }

    fn spec(&self) -> &Self::Spec {
        &self.spec
    }

    fn host_compute(&mut self) -> Option<Self::ComputeOut> {
        None
    }

    fn guest_compute(&mut self) -> (Self::ComputeOut, Self::ProofType) {
        let prover = self.prover.get_prover();
        let Receipt { inner, journal } = prover.prove_elf(self.env.clone(), METHOD_ELF).unwrap();

        ((), Receipt { inner, journal })
    }

    fn verify_proof(&self, _output: &Self::ComputeOut, proof: &Self::ProofType) -> bool {
        let result = proof.verify(METHOD_ID);

        match result {
            Ok(_) => true,
            Err(err) => {
                println!("{}", err);
                false
            }
        }
    }
}
