// Modules
mod cli;
mod git;
mod utility;

// Crates
use crate::cli::commands::*;
use crate::cli::subcommands::*;
use crate::utility::settings::*;
use clap::Parser;
use crate::git::git::*;

fn main() {
    let args = MainArgs::parse();

    let result = match_command(&args);


    /*
    let command = GitCommand::new( 
        "/Users/travispotter/Desktop/Code/Notes".to_string(),
        vec!["status"].iter().map(|&s| s.to_string()).collect()
    );
    */
    let new_branch = GitCommand::new(
        "/Users/travispotter/Desktop/Code/Notes".to_string(),
         "test_branch3".to_string());

    let switch_branch = GitCommand::switch(
        "/Users/travispotter/Desktop/Code/Notes".to_string(),
        "test_branch".to_string());


    if let Err(e) = run_process(&new_branch) {
        eprintln!("Error: {}", e);
    }

    utility::settings::init().ok();
}
