use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use log::info;
use serde::Serialize;

pub struct Metrics {
    pub prover: String,
    pub job_name: String,
    pub job_size: u32,
    pub proof_duration: Duration,
    pub verify_duration: Duration,
    pub output_bytes: u32,
    pub proof_bytes: u32,
}

impl Metrics {
    pub fn new(job_name: String, job_size: u32, prover: String) -> Self {
        Metrics {
            prover,
            job_name,
            job_size,
            proof_duration: Duration::default(),
            verify_duration: Duration::default(),
            output_bytes: 0,
            proof_bytes: 0,
        }
    }

    pub fn println(&self, prefix: &str) {
        info!("{}prover:             {:?}", prefix, &self.prover);
        info!("{}job_name:           {:?}", prefix, &self.job_name);
        info!("{}job_size:           {:?}", prefix, &self.job_size);
        info!("{}proof_duration:     {:?}", prefix, &self.proof_duration);
        info!("{}verify_duration:    {:?}", prefix, &self.verify_duration);
        info!("{}output_bytes:       {:?}", prefix, &self.output_bytes);
        info!("{}proof_bytes:        {:?}", prefix, &self.proof_bytes);
    }
}

pub trait Benchmark {
    const NAME: &'static str;
    type Spec;
    type ComputeOut: Eq + core::fmt::Debug;
    type ProofType;
    type Prover;

    fn prover_name(&self) -> String;

    fn job_size(spec: &Self::Spec) -> u32;

    fn output_size_bytes(output: &Self::ComputeOut, proof: &Self::ProofType) -> u32;

    fn proof_size_bytes(proof: &Self::ProofType) -> u32;

    fn new(spec: &Self::Spec, prover: &Self::Prover) -> Self;

    fn spec(&self) -> &Self::Spec;

    fn host_compute(&mut self) -> Option<Self::ComputeOut> {
        None
    }

    fn guest_compute(&mut self) -> (Self::ComputeOut, Self::ProofType);
    fn verify_proof(&self, output: &Self::ComputeOut, proof: &Self::ProofType) -> bool;

    fn run(&mut self) -> Metrics {
        let mut metrics = Metrics::new(
            String::from(Self::NAME),
            Self::job_size(self.spec()),
            Self::prover_name(self),
        );

        let (g_output, proof) = {
            let start = Instant::now();
            let result = self.guest_compute();
            metrics.proof_duration = start.elapsed();
            result
        };

        if let Some(h_output) = self.host_compute() {
            assert_eq!(g_output, h_output);
        }

        metrics.output_bytes = Self::output_size_bytes(&g_output, &proof);
        metrics.proof_bytes = Self::proof_size_bytes(&proof);

        let verify_proof = {
            let start = Instant::now();
            let result = self.verify_proof(&g_output, &proof);
            metrics.verify_duration = start.elapsed();
            result
        };

        assert_eq!(verify_proof, true);

        metrics
    }
}

pub fn init_logging() {
    env_logger::init();
}

#[derive(Serialize)]
struct CsvRow<'a> {
    prover: &'a str,
    job_name: &'a str,
    job_size: u32,
    proof_duration_millisec: u128,
    verify_duration_millisec: u128,
    proof_bytes: u32,
}

pub fn run_jobs<B: Benchmark>(
    out_path: &PathBuf,
    specs: &Vec<B::Spec>,
    provers: &Vec<B::Prover>,
) -> Vec<Metrics> {
    println!(
        "Running {} jobs; saving output to {}",
        specs.len(),
        out_path.display()
    );

    let mut out = {
        let out_file_exists = Path::new(out_path).exists();
        let out_file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(out_path)
            .unwrap();
        csv::WriterBuilder::new()
            .has_headers(!out_file_exists)
            .from_writer(out_file)
    };

    let mut all_metrics: Vec<Metrics> = Vec::new();

    for spec in specs {
        for prover in provers {
            let mut job = B::new(spec, prover);
            let job_number = all_metrics.len();

            println!(
                "Benchmarking:   {} {} {}",
                job_number,
                B::NAME,
                job.prover_name()
            );

            let job_metrics = job.run();
            job_metrics.println("+ ");
            let prover = B::prover_name(&job);
            out.serialize(CsvRow {
                prover: &prover,
                job_name: &job_metrics.job_name,
                job_size: job_metrics.job_size,
                proof_duration_millisec: job_metrics.proof_duration.as_millis(),
                verify_duration_millisec: job_metrics.verify_duration.as_millis(),
                proof_bytes: job_metrics.proof_bytes,
            })
            .expect("Could not serialize");
            out.flush().expect("Could not flush");

            all_metrics.push(job_metrics);
        }
    }

    out.flush().expect("Could not flush");

    all_metrics
}
