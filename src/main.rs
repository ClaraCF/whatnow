use clap::Parser;
use std::env;

mod utils;
mod domain;
mod cli;

fn main() {
    // Parse cli arguments
    //let cli = cli::Cli::parse();

    if let Some(config_home) = utils::get_config_home() {
        println!("Config home is: <{}>", config_home.display());
    } else {
        println!("We got no config home");
    }
}
