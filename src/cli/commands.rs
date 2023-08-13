use crate::cli::subcommands::*;
use crate::utility::settings::Settings;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "BunchOfGits")]
#[command(author = "Travis Potter <travis.s.potter@gmail.com>")]
#[command(version = "0.1.0")]
#[command(arg_required_else_help(true))]
#[command(
    about = "Takes the hassle out of working within multi-repo code bases.",
    long_about = "Quickly switch between git repositories and specified branches to get your machine to the
right state for the feature or bug changes you care about right now. Bookmarks for git workflows.
Just a bunch of gits."
)]

pub struct MainArgs {
    #[command(subcommand)]
    command: SubCommand,
}

pub fn match_command(args: &MainArgs) {
    match &args.command {
        SubCommand::Add(args) => {
            add(args);
        }
        /*
        SubCommands::Delete(args) => {
            delete(args);
        }
        */
        SubCommand::Switch(args) => {
            switch(args);
        }
        /*
         SubCommands::Previous(args) => {
            previous(args);
        }
        */
        SubCommand::List => {
            list();
        }
        SubCommand::Bunch(args) => {
            bunch(args);
        }
        SubCommand::Repo(args) => {
            repo(args);
        }
        /*
        SubCommands::Go(args) => {
            go(args);
        }
          */
        SubCommand::New(args) => {
            new(args);
        } /*
        SubCommands::Config(args) => {
        config(args);
        }
        SubCommands::Push(args) => {
        push(args);
        }
        SubCommands::Pull(args) => {
        pull(args);
        }
         */
        SubCommand::Template(args) => {
            template(args);
        }

        SubCommand::Init(args) => {
            init(args);
        }
    }
}
