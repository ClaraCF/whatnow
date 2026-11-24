use chrono::NaiveDate;
use clap::Parser;

mod domain;
mod cli;

fn main() {
    let cli = cli::Cli::parse();
}
