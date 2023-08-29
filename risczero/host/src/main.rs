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
    IterSha2Pure,
    BigSha2,
    Fact,
    BubbleSort,
}

use crate::Command::*;

const ALL: [Command; 7] = [
    EcdsaThenHashes,
    IterEcdsa,
    IterSha2,
    IterSha2Pure,
    BigSha2,
    Fact,
    BubbleSort,
];

fn run_command(cli: &Cli, command: &Command) -> () {
    match command {
        All => {
            for c in ALL {
                run_command(cli, &c)
            }
        }
        EcdsaThenHashes => {
            let _ = run_jobs::<ecdsa_then_hashes::Job>(
                &cli.out,
                ecdsa_then_hashes::new_jobs(),
                provers::PROVERS.to_vec(),
            );
        }
        IterEcdsa => {
            let _ = run_jobs::<iter_ecdsa::Job>(
                &cli.out,
                iter_ecdsa::new_jobs(),
                provers::PROVERS.to_vec(),
            );
        }
        IterSha2 => {
            let _ = run_jobs::<iter_sha2::Job>(
                &cli.out,
                iter_sha2::new_jobs(),
                provers::PROVERS.to_vec(),
            );
        }
        IterSha2Pure => {
            let _ = run_jobs::<iter_sha2_pure::Job>(
                &cli.out,
                iter_sha2_pure::new_jobs(),
                provers::PROVERS.to_vec(),
            );
        }
        BigSha2 => {
            let _ = run_jobs::<big_sha2::Job>(
                &cli.out,
                big_sha2::new_jobs(),
                provers::PROVERS.to_vec(),
            );
        }
        Fact => {
            let _ = run_jobs::<fact::Job>(&cli.out, fact::new_jobs(), provers::PROVERS.to_vec());
        }
        BubbleSort => {
            let _ = run_jobs::<bubble_sort::Job>(
                &cli.out,
                bubble_sort::new_jobs(),
                provers::PROVERS.to_vec(),
            );
        }
    }
}

fn main() {
    init_logging();
    let cli = Cli::parse();

    run_command(&cli, &cli.command);
}
