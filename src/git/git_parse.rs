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

pub struct Parse {
    branch: String,
    added: u32,
    deleted: u32,
    modified: u32,
}

/*
Maybe this doesn't want to be an enum. Maybe just passing in the string of the command, and any relevant args is suffiecient. still need a way to direct it to the correct parser.
*/
//need to make sure to use the porcelain options.
/*
status -- git status --porcelain=v2  | git status -s | git name-rev

git -C /Users/travispotter/Desktop/Code/Notes rev-parse --abbrev-ref HEAD --> branch name

git switch - --> switch to previous branch
*/
pub fn git_parse_decision(git_result: Output, command: GitParseOperation) {
    match &command {
        GitParseOperation::Switch(git_result) => {}
        GitParseOperation::Add(git_result) => {}
        GitParseOperation::Delete(git_result) => {}
        GitParseOperation::Push(git_result) => {}
        GitParseOperation::Pull(git_result) => {}
        GitParseOperation::Status(git_result) => {}
        GitParseOperation::Branch(git_result) => {}
        GitParseOperation::Stash(git_result) => {}
        GitParseOperation::RevParse(git_result) => {}
    }
}

fn parse_switch(result: Output) {}

fn parse_add(result: Output) {}

fn parse_delete(result: Output) {}

fn parse_push(result: Output) {}

fn parse_pull(result: Output) {}

fn parse_status(result: Output) {}

fn parse_branch(result: Output) {}

fn parse_stash(result: Output) {}

fn parse_rev_parse(result: Output) {}
