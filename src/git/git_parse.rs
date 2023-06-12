/*
*
* methods that parse git stdout and stderr values
* Get this information from the .c files from git/git source code

*
*/
 
use std::process::*;
 
pub enum GitParseOperation {
    Switch(Output),
    Add(Output),
    Delete(Output),
    Push(Output),
    Pull(Output),
    Status(Output),
    Branch(Output),
    Stash(Output),
    RevParse(Output),
}
 
/*
Maybe this doesn't want to be an enum. Maybe just passing in the string of the command, and any relevant args is suffiecient. still need a way to direct it to the correct parser. 
*/

pub fn git_parse_decision(git_result: Output, command: GitParseOperation)-> Result<Ok,_>{
     
    match &command{
        GitParseOperation::Switch(git_result) => {
             
        },
        GitParseOperation::Add(git_result) => {
             
        },
        GitParseOperation::Delete(git_result) => {
             
        },
        GitParseOperation::Push(git_result) => {
             
        },
        GitParseOperation::Pull(git_result) => {
             
        },
         GitParseOperation::Status(git_result) => {
             
        },
          GitParseOperation::Branch(git_result) => {
             
        },
           GitParseOperation::Stash(git_result) => {
             
        },
          GitParseOperation::RevParse(git_result) => {
             
        },
    } 
     
    return Ok(()); 
}






fn parse_switch(result: Output) -> Result<Ok,_> {
}

fn parse_add(result: Output) -> Result<Ok,_> {
}

fn parse_delete(result: Output) -> Result<Ok,_> {
}

fn parse_push(result: Output) -> Result<Ok,_> {
}

fn parse_pull(result: Output) -> Result<Ok,_> {
}

fn parse_status(result: Output) -> Result<Ok,_> {
}

fn parse_branch(result: Output) -> Result<Ok,_> {
}

fn parse_stash(result: Output) -> Result<Ok,_> {
}

fn parse_rev_parse(result: Output) -> Result<Ok,_> {
}

