use crate::git::*;
use std::io::{Result, Write};
use std::process::{Command, Output};

pub fn run_process(command: &GitCommand) -> Result<Output> {
    let mut cmd = command_builder(&command);
    let output = cmd.output()?;

    let status = git_parse::git_parse_decision(output.clone(), command.command.clone());

    //println!("{} | {:?} | {:?}",output.status.success(), output.stdout, output.stderr);
    // match status{
    //}
    Ok(output)
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

#[derive(Clone)]
pub struct GitCommand {
    pub path: String,
    pub command: String,
    pub args: Vec<String>,
}

impl GitCommand {
    /*
    pub fn new(path: String, branch: String) -> GitCommand {
        GitCommand {
            path,
            command: "switch".into(),
            args: format!("{} {}", "-c", branch),
        }
    }

    pub fn pull(path: String, branch: String) -> GitCommand {
        GitCommand {
            path,
            command: "pull".into(),
            args: String::from(branch),
        }
    }

    pub fn push(path: String, branch: String) -> GitCommand {
        GitCommand {
            path,
            command: "push".into(),
            args: String::from(branch),
        }
    }
    */

    pub fn switch(path: String, branch: String) -> GitCommand {
        GitCommand {
            path,
            command: "switch".into(),
            args: vec![&branch].iter().map(|&s| s.to_string()).collect(),
            //args: format!("{} {}", "-C", branch),
        }
    }
    /*
    pub fn status(path: String, branch: String) -> GitCommand {
        GitCommand {
            path,
            command: "status".into(),
            args: String::from(branch),
        }
    }

    pub fn delete(path: String, branch: String) -> GitCommand {
        GitCommand {
            path,
            command: "branch".into(),
            args: format!("{} {}", "-D", branch),
        }
    }

    pub fn show_current(path: String) -> GitCommand {
        GitCommand {
            path,
            command: "branch".into(),
            args: String::from("--show-current"),
        }
    }

    pub fn stash_list(path: String, branch: String) -> GitCommand {
        GitCommand {
            path,
            command: "stash".into(),
            args: format!("{} {}", "list", branch),
        }
    }

    pub fn stash(path: String, branch: String) -> GitCommand {
        GitCommand {
            path,
            command: "stash".into(),
            args: String::from("--all"),
        }
    }

    pub fn stash_pop(path: String) -> GitCommand {
        GitCommand {
            path,
            command: "stash".into(),
            args: String::from("apply"),
        }
    }

    pub fn repo_dir(path: String) -> GitCommand {
        GitCommand {
            path,
            command: "rev-parse".into(),
            args: String::from("--show-toplevel")
        }
    }

    */
    pub fn is_repo(path: &String) -> GitCommand {
        GitCommand {
            path: path.to_string(),
            command: "rev-parse".into(),
            args: vec!["--is-inside-work-tree"]
                .iter()
                .map(|&s| s.to_string())
                .collect(),
            //args: String::from("--is-inside-work-tree")
        }
    }
}
