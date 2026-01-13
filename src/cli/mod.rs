use clap::{Parser, Subcommand};

use crate::solutions::{
    self, birthday_cake_candles, compare_triplets, diagonal_difference, min_max_sum, sum_array,
    time_conversion,
};

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

    DiagonalDifference {},

    MinMaxSum {
        /// An array of 5 numbers. E.g 1 2 3 4 5
        #[arg(long, num_args = 5, value_delimiter = ' ', default_values_t = vec![1, 2, 3, 4, 5])]
        input: Vec<i64>,
    },

    BirthdayCakeCandles {
        /// An array of candles e.g 4, 4, 1, 3
        #[arg(long, value_delimiter = ' ', num_args = 1.., default_values_t = vec![4, 4, 1, 3])]
        input: Vec<i32>,
    },

    TimeConversion {
        /// A 12 hour time string. Example 07:33:43PM
        #[arg(long, default_value = "07:33:43PM")]
        input: String,
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
        Commands::DiagonalDifference {} => diagonal_difference::run(),
        Commands::MinMaxSum { input } => min_max_sum::run(&input),
        Commands::BirthdayCakeCandles { input } => birthday_cake_candles::run(input),
        Commands::TimeConversion { input } => time_conversion::run(&input),
    };
}
