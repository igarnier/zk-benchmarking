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
    EcdsaThenHashes,
    IterEcdsa,
    IterSha2,
    BigSha2,
    Fact,
    BubbleSort,
}

fn main() {
    init_logging();
    let cli = Cli::parse();

    let prover = String::from("risczero");

    match cli.command {
        Command::All => {
            let _ = run_jobs::<iter_ecdsa::Job>(&prover, &cli.out, iter_ecdsa::new_jobs());
            let _ = run_jobs::<iter_sha2::Job>(&prover, &cli.out, iter_sha2::new_jobs());
            let _ = run_jobs::<big_sha2::Job>(&prover, &cli.out, big_sha2::new_jobs());
            let _ = run_jobs::<fact::Job>(&prover, &cli.out, fact::new_jobs());
            let _ = run_jobs::<bubble_sort::Job>(&prover, &cli.out, bubble_sort::new_jobs());
        }
        Command::EcdsaThenHashes => {
            let _ = run_jobs::<ecdsa_then_hashes::Job>(
                &prover,
                &cli.out,
                ecdsa_then_hashes::new_jobs(),
            );
        }
        Command::IterEcdsa => {
            let _ = run_jobs::<iter_ecdsa::Job>(&prover, &cli.out, iter_ecdsa::new_jobs());
        }
        Command::IterSha2 => {
            let _ = run_jobs::<iter_sha2::Job>(&prover, &cli.out, iter_sha2::new_jobs());
        }
        Command::BigSha2 => {
            let _ = run_jobs::<big_sha2::Job>(&prover, &cli.out, big_sha2::new_jobs());
        }
        Command::Fact => {
            let _ = run_jobs::<fact::Job>(&prover, &cli.out, fact::new_jobs());
        }
        Command::BubbleSort => {
            let _ = run_jobs::<bubble_sort::Job>(&prover, &cli.out, bubble_sort::new_jobs());
        }
    }
}
