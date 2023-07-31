use directories::*;
use std::fs;
use std::io::*;
use std::path::Path;
use std::u8;
use std::env;
use clap::{Args, Subcommand};
use crate::git::*;
use crate::utility::*;
use crate::utility::settings::*;
use crate::utility::settings::Settings;

#[derive(Subcommand, Debug)]
pub enum SubCommands {
    /*
    /// Adds the current repository to a specific bunch
    Add(AddArgs),

    /// Deletes the current repository from a specific bunch
    Delete(DeleteArgs),

    /// Switch to the bunch configuration specified
    Switch(SwitchArgs),

    /// Go to previous bunch configuration
    Previous(PreviousArgs),
    */
    /// List all available bunch configurations and their settings
    List,

    /// Initialize bunch of gits with a new config file
    Init(InitArgs),
    /*
    /// Go to the specified bunch
    Go(GoArgs),

    /// Create a new bunch
    New(NewArgs),

    /// Change configuration settings
    Config(ConfigArgs),

    /// Push all commits on all repositories in the bunch
    Push(PushArgs),

    /// Pull new commits from remote for all repositories in the bunch
    Pull(PullArgs),

    /// Set up a template
    Template(TemplateArgs),
    */
}



/*
let new_branch = GitCommand::new(
"/Users/travispotter/Desktop/Code/Notes".to_string(),
"test_branch3".to_string(),
);

let switch_branch = GitCommand::switch(
"/Users/travispotter/Desktop/Code/Notes".to_string(),
"test_branch".to_string(),
);

if let Err(e) = run_process(&new_branch) {
eprintln!("Error: {}", e);
}

/*
let command = GitCommand::new(
"/Users/travispotter/Desktop/Code/Notes".to_string(),
vec!["status"].iter().map(|&s| s.to_string()).collect()
);
*/
*/

#[derive(Args, Debug)]
pub struct AddArgs {
    bunch: Option<String>,
    file: Option<String>,
    #[arg(required = false)]
    order: i32,
}

pub fn add(args: &AddArgs) {
    //do something with this
    /*
    Need a known place for settings. I guess always look in the root directory or there should be
    an os specific location Read in settings
    Get the bunch with the name
    modify the bunch by adding an additional config
    write settings back
    return confimration
    */

    //refactor out into get_settings() method

    if let Some(settings) = crate::utility::settings::get_settings() {
        let bunch: Vec<Bunch> = settings.bunches.into_iter().filter(|b| b.name == "bunch: 3").collect();
        println!("{:?}", bunch.first().unwrap().name);
        println!("{:?}", bunch.first().unwrap().items);
    };
}

#[derive(Args, Debug)]
pub struct DeleteArgs {
    bunch: Option<String>,
    file: Option<String>,
}

pub fn delete(args: &DeleteArgs) {
    //do something with this
    /*
    read in settings
    get the bunch with the same name
    modify the bunch by deleting the relavant part of the config
    write settings back
    return confirmaiton
    */

    if let Some(settings) = crate::utility::settings::get_settings() {
        let bunch: Vec<Bunch> = settings.bunches.into_iter().filter(|b| b.name == "bunch: 3").collect();
        println!("{:?}", bunch.first().unwrap().name);
        println!("{:?}", bunch.first().unwrap().items);
    };
}

#[derive(Args, Debug)]
pub struct SwitchArgs {
    bunch: Option<String>,
}

pub fn switch(args: &SwitchArgs) {
    //do something with this
    /*
    read in settings
    given the name of the bunch, find the bunch
    for each config iem in the bunch, change the repo and branch
    handle needing to stash on change, default to yes stash
    run build script for each branch in dependency order
    return back confirmation
    */
    if let Some(settings) = crate::utility::settings::get_settings() {
        let bunch: Vec<Bunch> = settings.bunches.into_iter().filter(|b| b.name == "bunch: 3").collect();
        println!("{:?}", bunch.first().unwrap().name);
        println!("{:?}", bunch.first().unwrap().items);
    };
}

#[derive(Args, Debug)]
pub struct PreviousArgs {}

pub fn previous(args: &PreviousArgs) {
    //do something with this
    /*
    get previous state name from settings
    call switch on a saved previous state command.
    write to settings the new previous state name
    */
    if let Some(settings) = crate::utility::settings::get_settings() {
        let bunch: Vec<Bunch> = settings.bunches.into_iter().filter(|b| b.name == "bunch: 3").collect();
        println!("{:?}", bunch.first().unwrap().name);
        println!("{:?}", bunch.first().unwrap().items);
    };
}

pub fn list() {
    //do something with this
    /*
    get settings
    get all bunches
    write to command line
    */
    if let Some(settings) = crate::utility::settings::get_settings() {
        let bunches = settings.bunches.into_iter();
            println!("{:?}", "-----bunches------");
        for bunch in bunches {
            println!("{:?}", "------------------");
            println!("{:?}", bunch.name);
        }

        let repos = settings.repos.into_iter();
            println!("{:?}", "------repos--------");
        for repo in repos{
            println!("{:?}", "------------------");
            println!("{:?}{:?}", repo.nickname, repo.path);
        }
    };
}

#[derive(Args, Debug)]
pub struct InitArgs {
    nickname: Option<String>,
}

pub fn init(args: &InitArgs) {
    //initialize a new bunch current branch/repo
    /*
    *check if in a git repo
    *check if settings exist if not create them.
    *if in a git repo, check if repo is in the settings already
    *place in settings with nickname if no nickname set, take the repo name as the name
    * return back a console line with verbal feedback of what happened
    */
    //is git repo? 
    //
    //

    /*
    if let Ok(current_dir) = env::current_dir(){
    println!("The current directory is {}", current_dir.display());
    };

    if let Ok(settings_path) = settings_path(){
    println!("The settings path is {}", settings_path);
    };

    if let Some(base_dirs) = BaseDirs::new() {
        let home_dir: &Path = &base_dirs.home_dir();
        println!("The home directory is {:?}", home_dir);
    }
    */

    let path = env::current_dir().unwrap().into_os_string().into_string().unwrap();
    let command = git::GitCommand::is_repo(path.clone());
    match git::run_process(&command){
        Ok(output) => {
            if git::is_success_status_code(output.status.to_string()) == true{
                if let Some(mut settings) = crate::utility::settings::get_settings() {
                    let item:&Vec<Repo> = &settings.repos.clone().into_iter().filter(|repo| repo.path == path.clone()).collect();
                    if item.len() == 0 {
                        let _ = &settings.repos.push(Repo::new(path.clone(), args.nickname.clone()));
                        let _ = crate::utility::settings::write_settings(&settings);
                    } else {
                        println!("Repo already exists in the settings.");
                    }
                };
            } else {
                println!("This is not a repo.");
            };
        },
        Err(_) => {
            println!("I made it to error.");
        }
    }
}

#[derive(Args, Debug)]
pub struct GoArgs {
    name: String,
}

pub fn go(args: &GoArgs) {
    //(take in a bunch name, and then run needed commands to switch over all the branches)
    /*
    maybe rename to switch
    read in the args
    read in the settings
    set previous  -> reorder as as necessary
    find the matching settings item with the bunch name
    open each item
    if specific run the build script
    */
    if let Some(settings) = crate::utility::settings::get_settings() {
        let bunch: Vec<Bunch> = settings.bunches.into_iter().filter(|b| b.name == args.name).collect();
        println!("{:?}", bunch.first().unwrap().name);
        println!("{:?}", bunch.first().unwrap().items);
    };
}

#[derive(Args, Debug)]
pub struct NewArgs {
    name: String,

    #[arg(num_args(0..))]
    repo: Option<String>, 

    /////Name of template to use
    //#[arg(short, long)]
    //template: String,
}

pub fn new(args: &NewArgs) {

    /*
    * get current settings, if don't exist create default. --> go to init and change so default
    * settings are created in get_settings rather than at init. 
    *
    *
    */

    if let Some(mut settings) = crate::utility::settings::get_settings() {
        let bunch: Vec<Bunch> = settings.bunches.into_iter().filter(|b| b.name == args.name).collect();

        //settings.bunches.push(Bunch::new())
    };
    //(create new set of branches in repos based on bunch template)
}

#[derive(Args, Debug)]
pub struct ConfigArgs {
    name: Option<String>,
}

pub fn config(args: &ConfigArgs) {

    //(opens toml config file in text editor to set global settings) or flags/args inline to set
}

#[derive(Args, Debug)]
pub struct PullArgs {
    bunch: Option<String>,
}

pub fn pull(args: &PullArgs) {
    //(switch to branches from bunch collection, and pull down latest, just fail on merge conflicts)
}

#[derive(Args, Debug)]
pub struct PushArgs {
    bunch: Option<String>,
}

pub fn push(args: &PushArgs) {

    //(switch to brances from bunch collection, and push to remote, does not handle tracking files or committing -- user to do this)
}

#[derive(Args, Debug)]
pub struct TemplateArgs {
    name: Option<String>,
    repos: Vec<String>,
}

pub fn template(args: &TemplateArgs) {

    //(takes in arguments vector of repo names to be used for common development workflows) ex. three repos are commonly involved in each feature development. allow for quickly creating a branch on each of the repos of the same name
}
