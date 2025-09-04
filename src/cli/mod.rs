use clap::{Parser, Subcommand};

use crate::{runner, solutions};

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

    Sum {
        /// Values to add.
        #[arg(long, num_args = 2, allow_hyphen_values = true, value_names = ["a", "b"])]
        input: Option<Vec<i32>>,
    },

    SumArray {},
}

pub fn run() {
    let cli = Cli::parse();
    let problem = match cli.command {
        Commands::CompareTriplets {} => "compare_triplets",
        Commands::Sum { input } => {
            match input {
                Some(values) => {
                    let a = values.first();
                    let b = values.get(1);
                    solutions::sum::run(a.copied(), b.copied());
                }
                None => solutions::sum::run(None, None),
            }
            "Sum"
        }
        Commands::SumArray {} => "sum",
    };

    runner::run_solution(problem);
}
