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
    pub fn new(path: String, args: Vec<String>) -> GitCommand{
        GitCommand{ 
            path,
            args,
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
            args:vec!["pull", &branch].iter().map(|&s| s.to_string()).collect()
        }
    }

    pub fn switch (path: String, branch: String) -> GitCommand{
        GitCommand{ 
            path,
            args:vec!["pull", &branch].iter().map(|&s| s.to_string()).collect()
        }
    }

    pub fn status(path: String, branch: String) -> GitCommand{
        GitCommand{ 
            path,
            args:vec!["status", &branch].iter().map(|&s| s.to_string()).collect()
        }
    }
}
