use clap::Parser;

use crate::runner;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Specify the problem to run.
    #[arg(short, long)]
    problem: String,
}

pub fn run() {
    let cli = Cli::parse();
    let problem = cli.problem;
    runner::run_solution(problem);
}
