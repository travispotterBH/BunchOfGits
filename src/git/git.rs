use std::process::Command;
use std::io::Result;

pub fn run_process(command: &GitCommand) -> Result<()>{
    let mut cmd = command_builder(&command);
    let output = cmd.output()?;

    println!("{}", String::from_utf8_lossy(&output.stdout));

    Ok(())
} 


pub fn command_builder(command: &GitCommand) -> Command{
    let mut cmd = Command::new(command.command.clone());
        cmd.args(&command.args);
        cmd
}

pub struct GitCommand {
    command: String,
    args: Vec<String>
}

impl GitCommand{
    pub fn new(command: String, args: Vec<String>) -> GitCommand{
        GitCommand{ 
         command,
            args,
        }
    }
}
