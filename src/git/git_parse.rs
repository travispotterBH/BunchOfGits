/*
*
* methods that parse git stdout and stderr values
* Get this information from the .c files from git/git source code

*
*/

use crate::git::git::*;
use regex::Regex;
use std::process::*;

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
pub fn git_parse_decision(git_result: Output, command: String) {
    match command.as_str() {
        "switch" => parse_switch(git_result),
        "add" => parse_add(git_result),
        "delete" => parse_delete(git_result),
        "push" => parse_push(git_result),
        "pull" => parse_pull(git_result),
        "status" => parse_status(git_result),
        "branch" => parse_branch(git_result),
        "stash" => parse_stash(git_result),
        "rev-parse" => parse_rev_parse(git_result),
        _ => todo!(),
    }
}

pub fn parse_switch(result: Output) {}

pub fn parse_add(result: Output) {}

pub fn parse_delete(result: Output) {}

pub fn parse_push(result: Output) {}

pub fn parse_pull(result: Output) {}

pub fn parse_status(result: Output) {}

pub fn parse_branch(result: Output) {}

pub fn parse_stash(result: Output) {}

pub fn parse_rev_parse(result: Output) {}
