// Modules
mod cli;
mod utility;
mod git;

// Crates
use crate::cli::commands::*;
use crate::cli::subcommands::*;
use crate::utility::settings::*;
use clap::{Parser};

//Main
fn main() {
    let args = BOGArgs::parse();


    let result = match_command(&args);
    
    utility::settings::init().ok();
}
