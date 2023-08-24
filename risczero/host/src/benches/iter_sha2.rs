use crate::helpers::inner_receipt_size_bytes;
use risc0_zkvm::serde::{from_slice, to_vec};
use risc0_zkvm::sha::Digest;
use risc0_zkvm::sha::DIGEST_WORDS;
use risc0_zkvm::{ExecutorEnv, Receipt};
use rustbench::Benchmark;

pub struct Job<'a> {
    pub spec: u32,
    pub env: ExecutorEnv<'a>,
    pub prover: crate::provers::Name,
}

pub fn new_jobs() -> Vec<<Job<'static> as Benchmark>::Spec> {
    vec![1, 10, 100, 1000, 10_000]
}

const METHOD_ID: [u32; DIGEST_WORDS] = risczero_benchmark_methods::ITER_SHA2_ID;
const METHOD_ELF: &[u8] = risczero_benchmark_methods::ITER_SHA2_ELF;

impl Benchmark for Job<'_> {
    const NAME: &'static str = "iter_sha2";
    type Spec = u32;
    type ComputeOut = Digest;
    type ProofType = Receipt;
    type Prover = crate::provers::Name;

    fn prover_name(&self) -> String {
        self.prover.to_string()
    }

    fn job_size(spec: &Self::Spec) -> u32 {
        *spec
    }

    fn output_size_bytes(_output: &Self::ComputeOut, proof: &Self::ProofType) -> u32 {
        proof.journal.len() as u32
    }

    fn proof_size_bytes(proof: &Self::ProofType) -> u32 {
        inner_receipt_size_bytes(&proof.inner)
    }

    fn new(spec: &Self::Spec, prover: &Self::Prover) -> Self {
        let spec_slice: [u32; 1] = [*spec];
        let initial_bytes: [u8; 32] = [0u8; 32];
        let env = ExecutorEnv::builder()
            .add_input(&to_vec(&spec_slice).unwrap())
            .add_input(&to_vec(&initial_bytes.to_vec()).unwrap())
            .build()
            .unwrap();

        Job {
            spec: *spec,
            env,
            prover: prover.clone(),
        }
    }

    fn spec(&self) -> &Self::Spec {
        &self.spec
    }

    fn host_compute(&mut self) -> Option<Self::ComputeOut> {
        let mut data = Vec::from([0u8; 32]);

        for _i in 0..self.spec {
            use sha2::Digest;
            let mut hasher = sha2::Sha256::new();
            hasher.update(&data);
            data = hasher.finalize().to_vec();
        }

        Some(risc0_zkvm::sha::Digest::try_from(data.as_slice()).unwrap())
    }

    fn guest_compute(&mut self) -> (Self::ComputeOut, Self::ProofType) {
        let prover = self.prover.get_prover();
        let Receipt { inner, journal } = prover.prove_elf(self.env.clone(), METHOD_ELF).unwrap();

        let guest_output: Digest =
            from_slice(&journal).expect("Journal output should output to data committed by guest");

        (guest_output, Receipt { inner, journal })
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
