use std::process;

use clap::{Parser, Subcommand};

use crate::runner;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Specify the problem to run.
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    CompareTriplets {},

    Sum {},

    SumArray {},
}

pub fn run() {
    let cli = Cli::parse();
    let problem = match cli.command {
        Commands::CompareTriplets {} => "compare_triplets",
        Commands::Sum {  } => "sum",
        Commands::SumArray {  } => "sum",
    };

    runner::run_solution(problem);
}
