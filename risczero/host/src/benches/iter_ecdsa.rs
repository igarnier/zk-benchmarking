use bincode;
use k256::{
    ecdsa::{signature::Signer, Signature, SigningKey, VerifyingKey},
    EncodedPoint,
};
use rand_core::OsRng;
use risc0_zkvm::prove::Prover;
use risc0_zkvm::serde::{from_slice, to_vec};
use risc0_zkvm::sha::DIGEST_WORDS;
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};
use rustbench::Benchmark;
use std::rc::Rc;

#[derive(Clone)]
pub struct Spec {
    encoded_verifying_key: EncodedPoint,
    message: Vec<u8>,
    signature: Signature,
    niter: u32,
}

pub struct Job<'a> {
    pub spec: Spec,
    pub env: ExecutorEnv<'a>,
    pub prover: Rc<dyn Prover>,
}

fn gen_spec(niter: u32) -> Spec {
    // Generate a random secp256k1 keypair and sign the message.
    let signing_key = SigningKey::random(&mut OsRng);
    let message = b"32 bytes message, including this";
    let signature: Signature = signing_key.sign(message);
    let verifying_key = signing_key.verifying_key();
    Spec {
        encoded_verifying_key: verifying_key.to_encoded_point(true),
        message: message.to_vec(),
        signature,
        niter,
    }
}

pub fn new_jobs() -> Vec<<Job<'static> as Benchmark>::Spec> {
    [1, 2, 3, 4].map(gen_spec).to_vec()
}

const METHOD_ID: [u32; DIGEST_WORDS] = risczero_benchmark_methods::ITER_ECDSA_ID;
const METHOD_ELF: &[u8] = risczero_benchmark_methods::ITER_ECDSA_ELF;

// TODO: is there a cleaner way to compute the proof size? Something directly
// exposed in the risc0 API?
fn inner_receipt_size_bytes(proof: &risc0_zkvm::receipt::InnerReceipt) -> u32 {
    let bytes: Vec<u8> = bincode::serialize(proof).unwrap();
    bytes.len() as u32
}

impl Benchmark for Job<'_> {
    const NAME: &'static str = "iter_ecdsa";
    type Spec = Spec;
    type ComputeOut = ();
    type ProofType = Receipt;

    fn job_size(spec: &Self::Spec) -> u32 {
        spec.niter
    }

    fn output_size_bytes(_output: &Self::ComputeOut, proof: &Self::ProofType) -> u32 {
        proof.journal.len() as u32
    }

    fn proof_size_bytes(proof: &Self::ProofType) -> u32 {
        inner_receipt_size_bytes(&proof.inner)
    }

    fn new(spec: Self::Spec) -> Self {
        let Spec {
            encoded_verifying_key,
            message,
            signature,
            niter,
        } = spec.clone();
        let env = ExecutorEnv::builder()
            .add_input(&to_vec(&(encoded_verifying_key, message, signature, niter)).unwrap())
            .build()
            .unwrap();

        let prover = default_prover();

        Job { spec, env, prover }
    }

    fn spec(&self) -> &Self::Spec {
        &self.spec
    }

    fn host_compute(&mut self) -> Option<Self::ComputeOut> {
        None
    }

    fn guest_compute(&mut self) -> (Self::ComputeOut, Self::ProofType) {
        let Receipt { inner, journal } =
            self.prover.prove_elf(self.env.clone(), METHOD_ELF).unwrap();

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
