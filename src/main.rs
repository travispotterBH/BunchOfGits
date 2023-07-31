// Modules
mod cli;
mod git;
mod utility;

// Crates
use crate::cli::commands::*;
use clap::Parser;

fn main() {
    let args = MainArgs::parse();
    let _result = match_command(&args);
}
