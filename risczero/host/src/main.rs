use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod benches;

use benches::*;
use rustbench::{init_logging, run_jobs};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    // CSV output file
    #[arg(long, value_name = "FILE")]
    out: PathBuf,

    #[command(subcommand)]
    command: Command,
}

#[derive(Eq, PartialEq, Subcommand)]
enum Command {
    All,
}

fn main() {
    init_logging();
    let cli = Cli::parse();

    let prover = String::from("risczero");

    run_jobs::<iter_ecdsa::Job>(&prover, &cli.out, iter_ecdsa::new_jobs());
    // run_jobs::<iter_sha2::Job>(&prover, &cli.out, iter_sha2::new_jobs());
    // run_jobs::<big_sha2::Job>(&prover, &cli.out, big_sha2::new_jobs());
    // run_jobs::<fact::Job>(&prover, &cli.out, fact::new_jobs());
    // run_jobs::<bubble_sort::Job>(&prover, &cli.out, bubble_sort::new_jobs());
}
