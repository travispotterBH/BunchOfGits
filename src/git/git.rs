use crate::git::*;
use std::io::{Result, Write};
use std::process::{Command, Output};

pub fn run_process(command: &GitCommand) -> Result<Output> {
    let mut cmd = command_builder(&command);
    let output = cmd.output()?;

    //let status = git_parse::git_parse_decision(output.clone(), command.command.clone());

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
     * Remember to read the friendly manual
     * multiple arguments must use the "cmd.args" method
     * so the values given must be in the form of a vec.
     */

    //    println!("git -C {} {} {}",&command.path, &command.command, &command.args.join(""));
    return cmd;
}

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
            args: vec![branch, default_branch]
                .into_iter()
                .map(|s| s.into())
                .collect(),
        }
    }

    pub fn switch(path: &str, branch: &str) -> GitCommand {
        GitCommand {
            path: path.into(),
            command: "switch".into(),
            args: vec![branch].into_iter().map(|s| s.into()).collect(),
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

    */
    pub fn repo_dir(path: &str) -> GitCommand {
        GitCommand {
            path: path.into(),
            command: "rev-parse".into(),
            args: vec!["--show-toplevel"].into_iter().map(|s| s.into()).collect(),
        }
    }

    pub fn is_repo(path: &str) -> GitCommand {
        GitCommand {
            path: path.to_string(),
            command: "rev-parse".into(),
            args: vec!["--is-inside-git-dir"]
                .into_iter()
                .map(|s| s.into())
                .collect(),
        }
    }

//  Worktree v 
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
            args: vec!["remove", branch]
                .into_iter()
                .map(|s| s.into())
                .collect(),
        }
    }

    pub fn worktree_list(path: &str, branch: &str) -> GitCommand {
        GitCommand {
            path: path.into(),
            command: "worktree".into(),
            args: vec!["list", branch].into_iter().map(|s| s.into()).collect(),
        }
    }

    pub fn is_inside_worktree(path: &str) -> GitCommand {
        GitCommand {
            path: path.to_string(),
            command: "rev-parse".into(),
            args: vec!["--is-inside-work-tree"]
                .into_iter()
                .map(|s| s.into())
                .collect(),
        }
    }

    pub fn is_bare_repository(path: &str) -> GitCommand {
        GitCommand {
            path: path.to_string(),
            command: "rev-parse".into(),
            args: vec!["--is-bare-repository"]
                .into_iter()
                .map(|s| s.into())
                .collect(),
        }
    }

    pub fn worktree_common_dir(path: &str) -> GitCommand {
        GitCommand {
            path: path.to_string(),
            command: "--git-common-dir".into(),
            args: vec![""]
                .into_iter()
                .map(|s| s.into())
                .collect(),
        }
    }

    pub fn clone_mirror(source_path: &str, mirror_path: &str) -> GitCommand {
        let mirror_path = format!("{}{}",mirror_path, "/.bare");
        GitCommand {
            path: source_path.to_string(),
            command: "clone".into(),
            args: vec!["--mirror", source_path, &mirror_path]
                .into_iter()
                .map(|s| s.into())
                .collect(),
        }
    }

    pub fn clone_bare(source_path: &str, mirror_path: &str) -> GitCommand {
        let mirror_path = format!("{}{}",mirror_path, "/.bare");
        GitCommand {
            path: source_path.to_string(),
            command: "clone".into(),
            args: vec!["--bare", source_path, &mirror_path]
                .into_iter()
                .map(|s| s.into())
                .collect(),
        }
    }

    pub fn remote_add_origin(source_path: &str, origin: &str) -> GitCommand {
        GitCommand {
            path: source_path.to_string(),
            command: "remote".into(),
            args: vec!["add", "origin", origin]
                .into_iter()
                .map(|s| s.into())
                .collect(),
        }
    }

    pub fn remote_url(source_path: &str) -> GitCommand {
        GitCommand {
            path: source_path.to_string(),
            command: "config".into(),
            args: vec!["--get", "remote.origin.url"]
                .into_iter()
                .map(|s| s.into())
                .collect(),
        }
    }

//  Worktree ^

}
