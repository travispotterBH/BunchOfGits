use crate::cli::information::*;
use crate::git::git_parse::git_parse_decision;
use std::io::{Result, Write};
use std::process::Command;

pub fn run_process(command: &GitCommand) -> Result<()> {
    let mut cmd = command_builder(&command);
    let output = cmd.output()?;

    //let status = git_parse_decision(output, command.command);

    // match status{
    //}
    Ok(())
}

pub fn command_builder(command: &GitCommand) -> Command {
    let mut cmd = Command::new("git");
    cmd.arg("-C");
    cmd.arg(&command.path);
    cmd.arg(&command.command);
    cmd.args(&command.args);
    return cmd;
}

//Thoughts:
/*
If I set up an enum or struct that carries the type of command, then set up a vec of git objects --
name pending change then I can pass that through with the args to the command builder, reducing the
need for all the impl fn's down below

there are some additional benefits. When it comes to parsing the command output, error, we need
only pass through the git object type and then we can pass the output and the git object type
to a parse decision router, which determines how and what to look for in the stdout/stderr


also perhaps this is an enum:

function from subcommand calls run_git_process pasing in a GitCommand, the run process does a match over the type of command and pics the correct command name as an enum, which is where the rest of the arguments are held?
*/

pub fn commmand_options(){

}



pub struct GitCommand {
    path: String,
    command: String,
    args: Vec<String>,
}

impl GitCommand {
    pub fn new(path: String, branch: String) -> GitCommand {
        GitCommand {
            path,
            command: "switch".to_string(),
            args: vec!["-c", &branch].iter().map(|&s| s.to_string()).collect(),
        }
    }

    pub fn pull(path: String, branch: String) -> GitCommand {
        GitCommand {
            path,
            command: "pull".to_string(),
            args: vec![&branch].iter().map(|&s| s.to_string()).collect(),
        }
    }

    pub fn push(path: String, branch: String) -> GitCommand {
        GitCommand {
            path,
            command: "push".to_string(),
            args: vec![&branch].iter().map(|&s| s.to_string()).collect(),
        }
    }

    pub fn switch(path: String, branch: String) -> GitCommand {
        GitCommand {
            path,
            command: "switch".to_string(),
            args: vec![&branch].iter().map(|&s| s.to_string()).collect(),
        }
    }

    pub fn status(path: String, branch: String) -> GitCommand {
        GitCommand {
            path,
            command: "status".to_string(),
            args: vec![&branch].iter().map(|&s| s.to_string()).collect(),
        }
    }

    pub fn delete(path: String, branch: String) -> GitCommand {
        GitCommand {
            path,
            command: "branch".to_string(),
            args: vec!["-D", &branch].iter().map(|&s| s.to_string()).collect(),
        }
    }

    pub fn show_current(path: String) -> GitCommand {
        GitCommand {
            path,
            command: "branch".to_string(),
            args: vec!["--show-current"]
                .iter()
                .map(|&s| s.to_string())
                .collect(),
        }
    }

    pub fn stash_list(path: String, branch: String) -> GitCommand {
        GitCommand {
            path,
            command: "stash".to_string(),
            args: vec!["list", &branch]
                .iter()
                .map(|&s| s.to_string())
                .collect(),
        }
    }

    pub fn stash(path: String, branch: String) -> GitCommand {
        GitCommand {
            path,
            command: "stash".to_string(),
            args: vec!["--all"].iter().map(|&s| s.to_string()).collect(),
        }
    }

    pub fn stash_pop(path: String) -> GitCommand {
        GitCommand {
            path,
            command: "stash".to_string(),
            args: vec!["apply"].iter().map(|&s| s.to_string()).collect(),
        }
    }

    pub fn branch_name(path: String) -> GitCommand {
        GitCommand {
            path,
            command: "rev-parse".to_string(),
            args: vec!["--show-toplevel"]
                .iter()
                .map(|&s| s.to_string())
                .collect(),
        }
    }
}
