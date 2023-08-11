// Modules
mod cli;
mod git;
mod utility;

use clap::Parser;
use git::git::{run_process, GitCommand};

// Crates
use crate::cli::commands::*;
use crate::utility::settings::*;

fn main() {
    //let mut settings: Settings;
    let _ = initialize_settings();

    let args = MainArgs::parse();

    match_command(&args);

    let _ = crate::utility::settings::write_settings(&get_settings());
}
