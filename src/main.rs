use clap::Parser;
use std::env;

mod utils;
mod domain;
mod cli;

use domain::task::Task;
use domain::checklist::Checklist;

fn main() {
    // Parse cli arguments
    //let cli = cli::Cli::parse();
    
    let data_home = utils::get_data_home().expect("a data home");

    let checklist = Checklist::from_disk(data_home.join("Tarea.json")).unwrap();

    println!("{:#?}", checklist);
}

