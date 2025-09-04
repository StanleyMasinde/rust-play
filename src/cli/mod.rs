use clap::{Parser, Subcommand};

use crate::solutions::{self, compare_triplets, sum_array};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Specify the problem to run.
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    CompareTriplets {
        /// Alice
        #[arg[long, num_args = 3]]
        alice: Option<Vec<i32>>,

        /// Bob
        #[arg[long, num_args = 3]]
        bob: Option<Vec<i32>>,
    },

    Sum {
        /// Values to add.
        #[arg(long, num_args = 2, allow_hyphen_values = true, value_names = ["a", "b"])]
        input: Option<Vec<i32>>,
    },

    SumArray {
        /// An array to sum. E.g 1 2 4
        #[arg(long, num_args = 1.., allow_hyphen_values = true)]
        input: Option<Vec<i32>>,
    },
}

pub fn run() {
    let cli = Cli::parse();
    match cli.command {
        Commands::CompareTriplets { alice, bob } => {
            compare_triplets::run(alice, bob);
        }
        Commands::Sum { input } => match input {
            Some(values) => {
                let a = values.first();
                let b = values.get(1);
                solutions::sum::run(a.copied(), b.copied());
            }
            None => solutions::sum::run(None, None),
        },
        Commands::SumArray { input } => sum_array::run(input),
    };
}
