use std::process::Command;
use std::io::Result;

pub fn run_process(command: &GitCommand) -> Result<()>{
    let mut cmd = command_builder(&command);
    let output = cmd.output()?;

    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr));

    Ok(())
} 


pub fn command_builder(command: &GitCommand) -> Command{
    let mut cmd = Command::new("git");
        cmd.arg("-C");
        cmd.arg(&command.path);
        cmd.args(&command.args);
        cmd
}

pub struct GitCommand {
    path: String,
    args: Vec<String>
}

impl GitCommand{
    pub fn new(path: String, branch: String) -> GitCommand{
        GitCommand{ 
            path,
            args:vec!["switch", "-c", &branch].iter().map(|&s| s.to_string()).collect()
        }
    }

    pub fn pull(path: String, branch: String) -> GitCommand{
        GitCommand{ 
            path,
            args:vec!["pull", &branch].iter().map(|&s| s.to_string()).collect()
        }
    }

    pub fn push(path: String, branch: String) -> GitCommand{
        GitCommand{ 
            path,
            args:vec!["push", &branch].iter().map(|&s| s.to_string()).collect()
        }
    }

    pub fn switch (path: String, branch: String) -> GitCommand{
        GitCommand{ 
            path,
            args:vec!["switch", &branch].iter().map(|&s| s.to_string()).collect()
        }
    }

    pub fn status(path: String, branch: String) -> GitCommand{
        GitCommand{ 
            path,
            args:vec!["status", &branch].iter().map(|&s| s.to_string()).collect()
        }
    }

    pub fn delete(path: String, branch: String) -> GitCommand{
        GitCommand{ 
            path,
            args:vec!["branch", "-D", &branch].iter().map(|&s| s.to_string()).collect()
        }
    }

    pub fn show_current(path: String) -> GitCommand{
        GitCommand{ 
            path,
            args:vec!["branch", "--show-current"].iter().map(|&s| s.to_string()).collect()
        }
    }

    pub fn stash_list(path: String, branch: String) -> GitCommand{
        GitCommand{ 
            path,
            args:vec!["stash", "list", &branch].iter().map(|&s| s.to_string()).collect()
        }
    }

    pub fn stash(path: String, branch: String) -> GitCommand{
        GitCommand{ 
            path,
            args:vec!["stash", "--all"].iter().map(|&s| s.to_string()).collect()
        }
    }

    pub fn stash_pop(path: String) -> GitCommand{
        GitCommand{ 
            path,
            args:vec!["stash", "pop"].iter().map(|&s| s.to_string()).collect()
        }
    }

    pub fn branch_name(path: String) -> GitCommand{
        GitCommand{ 
            path,
            args:vec!["rev-parse", "--show-toplevel"].iter().map(|&s| s.to_string()).collect()
        }
    }

}

