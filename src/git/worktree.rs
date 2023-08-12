use std::env::{*, self};
use std::fs::{*, self};
use std::path::{*, self};
use std::process::{Command, Output};

use crate::git::git::*;

//convert folder to worktree

//convert worktree to regular as an undo action


// worktree git methods

/*
* if a user setups the worktree flag, either convert the repo to a git worktree
* or create a new folder prefixed with "bunchOfGits" + repo name that follows 
* the git worktree rules as set out: 
* [worktree tutorial](https://morgan.cugerone.com/blog/how-to-use-git-worktree-and-in-a-clean-way/)
* [worktree tutorial 2](https://dev.to/yankee/practical-guide-to-git-worktree-58o0)
* in the bunch or in the repo config item, set a flag that says this is a worktree
* regardless always check if is worktree or not
* 
*/


pub fn convert_to_bare(source_path: &str){
    // check if any branch on the remote has uncommited or unpush changes. 
    // get git clone location
    // create a folder as bunchofgits_reponame
    // clone in as bare repo
    // check original repo for which branches on on the local
    // if flag for all
    // create a worktree for each local branch -- will need to figure out how to handle stash... or
    // easiest for now it to not that a this will fail if any branches has uncommitted changes. 
    // add to repos indicating it is worktree type

    let source_path = Path::new(source_path);

    let mirror_path_name = format!("BunchOfGits_{}", source_path.file_name().unwrap().to_str().unwrap());
    let mirror_path = source_path.parent().unwrap().join(&mirror_path_name);
    let _ = fs::create_dir(&mirror_path);
    let _output = run_process(&GitCommand::clone_mirror(source_path.to_str().unwrap(), mirror_path.to_str().unwrap()));
    let _ = fs::write(mirror_path.join(".git"), "gitdir: ./.bare");

}





pub fn convert_from_bare(){

}
