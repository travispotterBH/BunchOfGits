use std::env::{self, *};
use std::fs::{self, *};
use std::io::Error;
use std::path::{self, *};
use std::process::{Command, Output};
use std::{ffi::*, usize};

use std::str::*;

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

pub fn convert_to_bare(source_path: &str) {
    // check if any branch on the remote has uncommited or unpush changes.
    // get git clone location
    // create a folder as bunchofgits_reponame
    // clone in as bare repo
    // check original repo for which branches on on the local
    // if flag for all
    // create a worktree for each local branch -- will need to figure out how to handle stash... or
    // easiest for now it to not that a this will fail if any branches has uncommitted changes.
    // add to repos indicating it is worktree type

    if let Ok(RepoState::RepoNotWorktree) = repo_state(source_path) {
        match check_branch_uncommitted_changes(source_path) {
            Ok(value) => {
                println!("{}", value);
            }
            Err(..) => {
                panic!("i never thought i'd get this far.")
            }
        };

        let source_path = Path::new(source_path);
        let source_path_as_string = source_path.to_str().unwrap();

        if let Some(mirror_path) = get_mirror_path(&source_path) {
            if let Some(mirror_path_as_string) = mirror_path.to_str() {
                match fs::create_dir(&mirror_path) {
                    Ok(()) => {
                        let _output = run_process(GitCommand::clone_mirror(
                            source_path_as_string,
                            mirror_path_as_string,
                        ));
                        let _ = fs::write(mirror_path.join(".git"), "gitdir: ./.bare");
                    }
                    Err(_err) => {}
                }
            };
        };
    };

    println!("I don't do anything because I am already a worktree, or because I am not a repo.");
    //is_worktree_repository(source_path);
    /*
    println!("{:?}", get_mirror_path(Path::new(source_path)));
        let source_path = Path::new(source_path);
        let source_path_as_string = source_path.to_str().unwrap();

        if let Some(mirror_path) = get_mirror_path(&source_path){
            if let Some(mirror_path_as_string) = mirror_path.to_str() {
                match fs::create_dir(&mirror_path) {
                    Ok(()) => {
                        let _output = run_process(GitCommand::clone_mirror(source_path_as_string, mirror_path_as_string));
                        let _ = fs::write(mirror_path.join(".git"), "gitdir: ./.bare");
                    },
                    Err(err) => {},
                }

            };

        };
    */
}

fn get_mirror_path(source_path: &Path) -> Option<PathBuf> {
    source_path
        .file_name()
        .and_then(OsStr::to_str)
        .map(|str_repo_name| format!("BunchOfGits_{}", str_repo_name))
        .and_then(|mirror_path_name| source_path.parent().map(|p| p.join(mirror_path_name)))
}

fn check_branch_uncommitted_changes(source_path: &str) -> Result<bool, Error> {
    let is_index_clear: bool = run_process(GitCommand::diff_index_quiet_head(&source_path))?
        .status
        .success();
    let is_files_unchanged: bool = run_process(GitCommand::diff_files_quiet(&source_path))?
        .status
        .success();

    Ok(is_files_unchanged && is_index_clear)
}

fn is_worktree_repository(source_path: &str) {
    let mut is_inside_worktree: bool = false;
    match run_process(GitCommand::is_inside_worktree(source_path)) {
        Ok(value) => {
            if value.status.success() && String::from_utf8_lossy(&value.stdout).trim() == "true" {
                is_inside_worktree = true;
            }
        }
        _ => todo!(),
    }

    let mut common_dir: String = String::new();
    match run_process(GitCommand::common_dir(source_path)) {
        Ok(value) => {
            if value.status.success() {
                common_dir = String::from_utf8_lossy(&value.stdout).trim().to_string();
            }
        }
        _ => todo!(),
    }

    let mut is_bare_repository: bool = false;

    match run_process(GitCommand::is_bare_repository(&common_dir)) {
        Ok(value) => {
            if value.status.success() {
                is_bare_repository = String::from_utf8_lossy(&value.stdout).trim() == "true";
            }
        }
        _ => todo!(),
    }

    println!(
        "is worktree:{} | common_dir:{} | common_dir is bare: {}",
        is_inside_worktree, common_dir, is_bare_repository
    )
}

fn repo_state(source_path: &str) -> Result<RepoState, Box<dyn std::error::Error>> {
    let is_inside_worktree =
        std::str::from_utf8(&run_process(GitCommand::is_inside_worktree(source_path))?.stdout)?
            .trim()
            == "true";

    let common_dir =
        std::str::from_utf8(&run_process(GitCommand::common_dir(source_path))?.stdout)?
            .trim()
            .to_string();

    let is_bare_repository =
        std::str::from_utf8(&run_process(GitCommand::is_bare_repository(&common_dir))?.stdout)?
            .trim()
            == "true";

    println!(
        "is worktree:{} | common_dir:{} | common_dir is bare: {}",
        is_inside_worktree, common_dir, is_bare_repository
    );

    match (is_inside_worktree, is_bare_repository) {
        (true, true) => {
            if common_dir == "".to_string() {
                return Ok(RepoState::NotARepo);
            };
            Ok(RepoState::WorktreeBranchDir)
        }

        (true, false) => return Ok(RepoState::RepoNotWorktree),

        (false, true) => return Ok(RepoState::WorktreeTopLevelDir),

        (false, false) => return Ok(RepoState::NotARepo),
    }
}

enum RepoState {
    NotARepo,
    RepoNotWorktree,
    WorktreeTopLevelDir,
    WorktreeBranchDir,
}
