// Modules
mod cli;
mod git;
mod utility;

// Crates
use crate::cli::commands::*;
use crate::cli::subcommands::*;
use crate::utility::settings::*;
use clap::Parser;

//Main
fn main() {
    let args = MainArgs::parse();

    let result = match_command(&args);

    utility::settings::init().ok();
}
