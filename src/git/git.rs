use crate::git::*;
use std::io::{Result, Write};
use std::process::{Command, Output};

pub fn run_process(command: &GitCommand) -> Result<Output> {
    let mut cmd = command_builder(&command);
    let output = cmd.output()?;

    let status = git_parse::git_parse_decision(output.clone(), command.command.clone());

    //println!("{}" ,output.status.success());
    //println!("{:?}", cmd);

    Ok(output)
}

pub fn command_builder(command: &GitCommand) -> Command {
    let mut cmd = Command::new("git");
    cmd.arg("-C");
    cmd.arg(&command.path);
    cmd.arg(&command.command);
    cmd.args(&command.args);
    /*
    * Remember to read the friednly manual
    * multiple arguments must use the "cmd.args" method
    * so the values given must be in the form of a vec.
    */


//    println!("git -C {} {} {}",&command.path, &command.command, &command.args.join(""));
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
    pub fn new(path: &str, branch: &str) -> GitCommand {
        GitCommand {
            path,
            command: "switch".into(),
            args: format!("{} {}", "-c", branch),
        }
    }

    pub fn pull(path: &str, branch: &str) -> GitCommand {
        GitCommand {
            path,
            command: "pull".into(),
            args: format!("{}", branch),
        }
    }

    pub fn push(path: &str, branch: &str) -> GitCommand {
        GitCommand {
            path,
            command: "push".into(),
            args: &str::from(branch),
        }
    }
    */

    pub fn branch(path: &str, branch: &str, default_branch: &str) -> GitCommand {
        GitCommand {
            path: path.into(),
            command: "branch".into(),
            args: vec![branch, default_branch].into_iter().map(|s| s.into()).collect(),
        }
    }

    pub fn switch(path: &str, branch: &str) -> GitCommand {
        GitCommand {
            path: path.into(),
            command: "switch".into(),
            args: vec![branch].into_iter().map(|s| s.into()).collect(),
        }
    }


    pub fn worktree_add(path: &str, branch: &str) -> GitCommand {
        GitCommand {
            path: path.into(),
            command: "worktree".into(),
            args: vec!["add", branch].into_iter().map(|s| s.into()).collect(),
        }
    }

    pub fn worktree_remove(path: &str, branch: &str) -> GitCommand {
        GitCommand {
            path: path.into(),
            command: "worktree".into(),
            args: vec!["remove", branch].into_iter().map(|s| s.into()).collect(),
        }
    }

    pub fn worktree_list(path: &str, branch: &str) -> GitCommand {
        GitCommand {
            path: path.into(),
            command: "worktree".into(),
            args: vec!["list", branch].into_iter().map(|s| s.into()).collect(),
        }
    }

    pub fn init(path: &str, branch: &str) -> GitCommand {
        GitCommand {
            path: path.into(),
            command: "switch".into(),
            args: vec![branch].into_iter().map(|s| s.into()).collect(),
        }
    }
    /*
    pub fn status(path: &str, branch: &str) -> GitCommand {
        GitCommand {
            path,
            command: "status".into(),
            args: &str::from(branch),
        }
    }

    pub fn delete(path: &str, branch: &str) -> GitCommand {
        GitCommand {
            path,
            command: "branch".into(),
            args: format!("{} {}", "-D", branch),
        }
    }

    pub fn show_current(path: &str) -> GitCommand {
        GitCommand {
            path,
            command: "branch".into(),
            args: &str::from("--show-current"),
        }
    }

    pub fn stash_list(path: &str, branch: &str) -> GitCommand {
        GitCommand {
            path,
            command: "stash".into(),
            args: format!("{} {}", "list", branch),
        }
    }

    pub fn stash(path: &str, branch: &str) -> GitCommand {
        GitCommand {
            path,
            command: "stash".into(),
            args: &str::from("--all"),
        }
    }

    pub fn stash_pop(path: &str) -> GitCommand {
        GitCommand {
            path,
            command: "stash".into(),
            args: &str::from("apply"),
        }
    }

    pub fn repo_dir(path: &str) -> GitCommand {
        GitCommand {
            path,
            command: "rev-parse".into(),
            args: &str::from("--show-toplevel")
        }
    }

    */
    pub fn is_repo(path: &str) -> GitCommand {
        GitCommand {
            path: path.to_string(),
            command: "rev-parse".into(),
            args: vec!["--is-inside-work-tree"].into_iter().map(|s| s.into()).collect(),
        }
    }
}
