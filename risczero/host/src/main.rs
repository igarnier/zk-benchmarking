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
    Xp,
    Xp2,
}

use crate::Command::*;

const ALL: [Command; 9] = [
    EcdsaThenHashes,
    IterEcdsa,
    IterSha2,
    IterSha2Pure,
    BigSha2,
    Fact,
    BubbleSort,
    Xp,
    Xp2,
];

fn run_command(cli: &Cli, command: &Command, provers: &Vec<provers::Name>) -> () {
    match command {
        All => {
            for c in ALL {
                run_command(cli, &c, provers)
            }
        }
        EcdsaThenHashes => {
            let _ = run_jobs::<ecdsa_then_hashes::Job>(
                &cli.out,
                &ecdsa_then_hashes::new_jobs(),
                provers,
            );
        }
        IterEcdsa => {
            let _ = run_jobs::<iter_ecdsa::Job>(&cli.out, &iter_ecdsa::new_jobs(), provers);
        }
        IterSha2 => {
            let _ = run_jobs::<iter_sha2::Job>(&cli.out, &iter_sha2::new_jobs(), provers);
        }
        IterSha2Pure => {
            let _ = run_jobs::<iter_sha2_pure::Job>(&cli.out, &iter_sha2_pure::new_jobs(), provers);
        }
        BigSha2 => {
            let _ = run_jobs::<big_sha2::Job>(&cli.out, &big_sha2::new_jobs(), provers);
        }
        Fact => {
            let _ = run_jobs::<fact::Job>(&cli.out, &fact::new_jobs(), provers);
        }
        BubbleSort => {
            let _ = run_jobs::<bubble_sort::Job>(&cli.out, &bubble_sort::new_jobs(), provers);
        }
        Xp => {
            let _ = run_jobs::<xp::Job>(&cli.out, &xp::new_jobs(), provers);
        }
        Xp2 => {
            let _ = run_jobs::<xp2::Job>(&cli.out, &xp2::new_jobs(), provers);
        }
    }
}

fn main() {
    init_logging();
    let cli = Cli::parse();

    let provers =
        if std::env::var("BONSAI_API_URL").is_ok() && std::env::var("BONSAI_API_KEY").is_ok() {
            provers::PROVERS.to_vec()
        } else {
            println!("BONSAI_API_URL or BONSAI_API_KEY env vars not set, will not use Bonsai");
            let mut provers = provers::PROVERS.to_vec();
            provers.retain(|&prover| prover != provers::Name::Bonsai);
            provers
        };

    run_command(&cli, &cli.command, &provers);
}
