use std::process;

use clap::Parser;

use crate::runner;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Specify the problem to run.
    #[arg(short, long)]
    problem: Option<String>,
}

pub fn run() {
    let cli = Cli::parse();
    let problem = match cli.problem {
        Some(problem) => problem,
        None => {
            println!("Problem cannot be empty");
            process::exit(1)
        }
    };

    runner::run_solution(problem);
}
