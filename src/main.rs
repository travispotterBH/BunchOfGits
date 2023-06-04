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

//Main
fn main() {
    let args = MainArgs::parse();

    let result = match_command(&args);

    let command = GitCommand::new( 
        "ls".to_string(), 
        vec!["-h".to_string(), "-a".to_string()]
    );


if let Err(e) = run_process(&command) {
    eprintln!("Error: {}", e);
}

utility::settings::init().ok();
}
