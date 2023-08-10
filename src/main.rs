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
    let mut settings: Settings;

    if let Some(setting) = crate::utility::settings::get_settings() {
        settings = setting;
    } else {
        settings = crate::utility::settings::create_default_settings();
    }

    let args = MainArgs::parse();

    match_command(&args, &mut settings);

    /*    for repo in &settings.repos {
            run_process(&GitCommand::switch(repo.path.clone(), "test".into()));
            println!("On repo: {} | Branch: {}", repo.path, "test");
            run_process(&GitCommand::switch(repo.path.clone(), "main".into()));
            println!("On repo: {} | Branch: {}", repo.path, "main");
        }
    */
    let _ = crate::utility::settings::write_settings(&settings);
}
