use chrono::NaiveDate;
use clap::Parser;

mod cli;

fn main() {
    let cli = cli::Cli::parse();
}
