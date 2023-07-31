use crate::cli::subcommands::*;
use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "Bunch of Gits")]
#[command(author = "Travis P. <travis.s.potter@gmail.com>")]
#[command(version = "0.1.0")]
#[command(
    about = "Takes the hassle out of working within multi-repo code bases.",
    long_about = "Quickly switch between git repositories and specified branches to get your machine to the
right state for the feature or bug changes you care about right now. Bookmarks for git workflows.
Just a bunch of gits."
)]

pub struct MainArgs {
    #[command(subcommand)]
    command: SubCommands,
    /////Name of a person to greet
    //#[arg(short, long)]
    //name: String,

    /////Number of times to greet
    //#[arg(short, long, default_value_t = 1)]
    //count: u8,
}

pub fn match_command(args: &MainArgs) {
    match &args.command {
        /*
        SubCommands::Add(subcommand_args) => {
            add(&subcommand_args);
        }
        SubCommands::Delete(subcommand_args) => {
            delete(&subcommand_args);
        }
        SubCommands::Switch(subcommand_args) => {
            switch(&subcommand_args);
        }
        SubCommands::Previous(subcommand_args) => {
            previous(&subcommand_args);
        }
        */
        SubCommands::List => {
            list();
        }
        SubCommands::Init(subcommand_args) => {
            init(&subcommand_args);
        }
        /*
        SubCommands::Go(subcommand_args) => {
            go(&subcommand_args);
        }
        SubCommands::New(subcommand_args) => {
            new(&subcommand_args);
        }
        SubCommands::Config(subcommand_args) => {
            config(&subcommand_args);
        }
        SubCommands::Push(subcommand_args) => {
            push(&subcommand_args);
        }
        SubCommands::Pull(subcommand_args) => {
            pull(&subcommand_args);
        }
        SubCommands::Template(subcommand_args) => {
            template(&subcommand_args);
        }
        */
    }
}
