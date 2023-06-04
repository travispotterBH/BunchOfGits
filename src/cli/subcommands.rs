use clap::{Args, Subcommand};

#[derive(Subcommand, Debug)]
pub enum SubCommands {
    /// Adds the current repository to a specific bunch
    Add(AddArgs),

    /// Deletes the current repository from a specific bunch
    Delete(DeleteArgs),

    /// Switch to the bunch configuration specified
    Switch(SwitchArgs),

    /// Go to previous bunch configuration
    Previous(PreviousArgs),

    /// List all available bunch configurations and their settings
    List(ListArgs),

    /// Initialize bunch of gits with a new config file
    Init(InitArgs),

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
    Template(TemplateArgs)
}

#[derive(Args, Debug)]
pub struct AddArgs {
    name: Option<String>,
}

pub fn add(args: &AddArgs){
    //do something with this
    /*
    Need a known place for settings. I guess always look in the root directory or there should be
    an os specific location Read in settings
    Get the bunch with the name
    modify the bunch by adding an additional config
    write settings back
    return confimration
    */

    println!("{:?}", args.name);
}

#[derive(Args, Debug)]
pub struct DeleteArgs {
    name: Option<String>,
}

pub fn delete(args: &DeleteArgs){
    //do something with this
    /*
    read in settings
    get the bunch with the same name 
    modify the bunch by deleting the relavant part of the config
    write settings back
    return confirmaiton
    */
}

#[derive(Args, Debug)]
pub struct SwitchArgs {
    name: Option<String>,
}

pub fn switch(args: &SwitchArgs){
    //do something with this
    /*
    read in settings
    given the name of the bunch, find the bunch
    for each config iem in the bunch, change the repo and branch
    handle needing to stash on change, default to yes stash
    run build script for each branch in dependency order
    return back confirmation
    */
}

#[derive(Args, Debug)]
pub struct PreviousArgs {
    name: Option<String>,
}

pub fn previous(args: &PreviousArgs){
    //do something with this
    /*
    get previous state name from settings
    call switch on a saved previous state command.
    write to settings the new previous state name
    */
}

#[derive(Args, Debug)]
pub struct ListArgs {
    name: Option<String>,
}

pub fn list(args: &ListArgs){
    //do something with this
    /*
    get settings
    get all bunches
    write to command line
    */
}

#[derive(Args, Debug)]
pub struct InitArgs {
    name: Option<String>,
}

pub fn init(args: &InitArgs){
    //initialize a new bunch current branch/repo

}

#[derive(Args, Debug)]
pub struct GoArgs {
    name: Option<String>,
}

pub fn go (args: &GoArgs){
    //(take in a bunch name, and then run needed commands to switch over all the branches)
}

#[derive(Args, Debug)]
pub struct NewArgs {
    name: Option<String>,
}

pub fn new(args: &NewArgs){

    //(create new set of branches in repos based on bunch template)
}

#[derive(Args, Debug)]
pub struct ConfigArgs {
    name: Option<String>,
}

pub fn config(args: &ConfigArgs){

    //(opens toml config file in text editor to set global settings) or flags/args inline to set
}

#[derive(Args, Debug)]
pub struct PullArgs {
    name: Option<String>,
}

pub fn pull(args: &PullArgs){
    //(switch to branches from bunch collection, and pull down latest, just fail on merge conflicts)
}

#[derive(Args, Debug)]
pub struct PushArgs {
    name: Option<String>,
}

pub fn push(args: &PushArgs){

    //(switch to brances from bunch collection, and push to remote, does not handle tracking files or committing -- user to do this)
}

#[derive(Args, Debug)]
pub struct TemplateArgs {
    name: Option<String>,
}

pub fn template(args: &TemplateArgs){

    //(takes in arguments vector of repo names to be used for common development workflows) ex. three repos are commonly involved in each feature development. allow for quickly creating a branch on each of the repos of the same name
}
