use crate::helpers::inner_receipt_size_bytes;
use risc0_zkvm::sha::DIGEST_WORDS;
use risc0_zkvm::{
    serde::{from_slice, to_vec},
    ExecutorEnv, Receipt,
};
use rustbench::Benchmark;

pub struct Job<'a> {
    pub spec: u64,
    pub env: ExecutorEnv<'a>,
    pub prover: crate::provers::Name,
}

pub fn new_jobs() -> Vec<<Job<'static> as Benchmark>::Spec> {
    vec![5, 10, 15, 20, 25]
}

const METHOD_ID: [u32; DIGEST_WORDS] = risczero_benchmark_methods::XP2_ID;
const METHOD_ELF: &[u8] = risczero_benchmark_methods::XP2_ELF;

impl Benchmark for Job<'_> {
    const NAME: &'static str = "xp2";
    type Spec = u64;
    type ComputeOut = u64;
    type ProofType = Receipt;
    type Prover = crate::provers::Name;

    fn prover_name(&self) -> String {
        self.prover.to_string()
    }

    fn job_size(spec: &Self::Spec) -> u32 {
        *spec as u32
    }

    fn output_size_bytes(_output: &Self::ComputeOut, proof: &Self::ProofType) -> u32 {
        proof.journal.len() as u32
    }

    fn proof_size_bytes(proof: &Self::ProofType) -> u32 {
        inner_receipt_size_bytes(&proof.inner)
    }

    fn new(spec: &Self::Spec, prover: &Self::Prover) -> Self {
        let env = ExecutorEnv::builder()
            .add_input(&to_vec(&spec).unwrap())
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

    fn guest_compute(&mut self) -> (Self::ComputeOut, Self::ProofType) {
        let prover = self.prover.get_prover();
        let Receipt { inner, journal } = prover.prove_elf(self.env.clone(), METHOD_ELF).unwrap();

        let guest_output: u64 =
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
