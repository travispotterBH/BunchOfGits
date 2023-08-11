use crate::git::git::*;
use crate::utility::settings::*;
use clap::{Args, Subcommand};
use std::env;

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    /// For managing bunches
    Bunch(BunchArgs),
    
    /// Adds the current repository to a specific bunch
    Add(AddArgs),
    /*

    /// Deletes the current repository from a specific bunch
    Delete(DeleteArgs),

    */
    /// Switch to the bunch configuration specified
    Switch(SwitchArgs),

    /*
    /// Go to previous bunch configuration
    Previous(PreviousArgs),
    */
    /// List all available bunch configurations and their settings
    List,

    /// Initialize bunch of gits with a new config file
    Repo(RepoArgs),
    /*
    /// Go to the specified bunch
    Go(GoArgs),

    */
    /// Create a new bunch
    New(NewArgs),
    /*
    /// Change configuration settings
    Config(ConfigArgs),

    /// Push all commits on all repositories in the bunch
    Push(PushArgs),

    /// Pull new commits from remote for all repositories in the bunch
    Pull(PullArgs),

    */
    /// Set up a template
    Template(TemplateArgs),
}

#[derive(Args, Debug)]
pub struct BunchArgs {
    #[arg(short, long, group = "mode")]
    new: bool,

    #[arg(short, long, group = "mode")]
    delete: bool,

    #[arg(short, long, group = "mode")]
    add: bool,

    //bunch: String,
    //repo: String,
    //branch: String,
}

pub fn bunch(args: &BunchArgs) {
    println!("made it to bunch");
    }


#[derive(Args, Debug)]
pub struct AddArgs {
    bunch: String,
    repo: String,
    branch: String,
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

    let mut settings = get_settings();
    //refactor out into get_settings() method
    if let Some(ref mut bunch) = settings.bunches.iter_mut().find(|b| b.name == args.bunch) {
        if let Some(repo) = &settings.repos.iter_mut().find(|r| r.name == args.repo) {
            bunch.items.push(Item {
                repo: repo.path.clone(),
                branch: args.branch.clone(),
            });
        } else {
            println!("No repo of name '{}' found in the settings.", args.repo);
        }
    } else {
        println!("No bunch of name '{}' found in the settings.", args.bunch);
    }

    modify_settings(&settings);
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

    let settings = get_settings();
    let bunch: Vec<Bunch> = settings
        .bunches
        .into_iter()
        .filter(|b| b.name == "bunch: 3")
        .collect();
    println!("{:?}", bunch.first().unwrap().name);
    println!("{:?}", bunch.first().unwrap().items);
}

#[derive(Args, Debug)]
pub struct SwitchArgs {
    bunch: String,
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

    let mut settings = get_settings();
    if let Some(bunch) = &settings
        .bunches
        .iter()
        .filter(|b| b.name == args.bunch)
        .collect::<Vec<&Bunch>>()
        .first()
    {
        for item in &bunch.items {
            run_process(&GitCommand::switch(&item.repo, &item.branch));
        }
    } else {
        println!("No bunch of name '{}' found in the settings.", args.bunch);
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
    let settings = get_settings();
    let bunch: Vec<Bunch> = settings
        .bunches
        .into_iter()
        .filter(|b| b.name == "bunch: 3")
        .collect();
    println!("{:?}", bunch.first().unwrap().name);
    println!("{:?}", bunch.first().unwrap().items);
}

pub fn list() {
    let settings = get_settings();
    let bunches = settings.bunches.into_iter();
    println!("{:?}", "-----bunches------");

    for bunch in bunches {
        println!("Start: {:?}", bunch.name);
        for item in bunch.items {
            println!("    Repo:{:?} | Branch:{:?}", item.repo, item.branch);
        }
        println!("End: {:?}", bunch.name);
    }

    let repos = settings.repos.into_iter();
    println!("{:?}", "------repos--------");
    for repo in repos {
        println!("Name:{:?} | Path:{:?}", repo.name, repo.path);
    }
}

#[derive(Args, Debug)]
pub struct RepoArgs {
    name: String,
    default_branch: String,
}

pub fn repo(args: &RepoArgs) {
    let path = env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();

    let command = GitCommand::is_repo(&path);

    let mut settings = get_settings();

    if let Ok(output) = run_process(&command) {
        if output.status.success() == true {
            let item = &settings.repos.iter().any(|repo| repo.path == path);
            if !item {
                let _ = &settings.add_repo(Repo::new(
                    path,
                    args.name.to_owned(),
                    args.default_branch.to_owned(),
                ));
                modify_settings(&settings);
            } else {
                println!("Repo already exists in the settings.");
            }
        } else {
            println!("This is not a repo.");
        };
    }
}

#[derive(Args, Debug)]
pub struct NewArgs {
    name: String,
    template: Option<String>,

    #[arg(short, long)]
    go: bool,
}

pub fn new(args: &NewArgs) {
    let mut settings = get_settings();

    if let Some(template) = &args.template {
        if let Some(item) = &settings
            .templates
            .iter_mut()
            .find(|t| t.name == template.clone())
        {
            let bunch = Bunch::from_template(args.name.clone(), &item);
            for repo in item.repos.iter() {
                //               println!("we are supposedly making branches");
                let status = run_process(&GitCommand::branch(
                    &repo.path,
                    &args.name,
                    &repo.default_branch,
                ));
                println!("{:?}", status.unwrap().status.success());

                if args.go {
                    run_process(&GitCommand::switch(&repo.path, &args.name));
                    //                println!("we are supposedly going");
                }
            }

            &settings.add_bunch(&bunch);
        }
    } else {
            &settings.add_bunch(&Bunch::new(&args.name));
        //      println!("no template found");
    }

    modify_settings(&settings);
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
    name: String,

    #[arg(short, long)]
    delete: bool,

    #[arg(short, long, num_args(0..))]
    repos: Option<Vec<String>>,
}

pub fn template(args: &TemplateArgs) {
    //(takes in arguments vector of repo names to be used for common development workflows) ex. three repos are commonly involved in each feature development. allow for quickly creating a branch on each of the repos of the same name

    let mut settings = get_settings();

    if let true = args.delete {
        settings.delete_template(args.name.clone());
        modify_settings(&settings);
        return;
    };

    if args.repos == None || args.repos.clone().unwrap().len() == 0 {
        println!("No repos passed in.");
        return;
    };

    let mut template: Template = Template::new(args.name.clone());
    for repo in args.repos.clone().unwrap().iter() {
        if let Some(repository) = settings
            .repos
            .iter_mut()
            .find(|r| r.name == repo.to_owned())
        {
            template.repos.push(repository.clone());
            println!("Repo '{}' added to {}.", repo, args.name);
        } else {
            println!("No repo of name '{}' found in the settings.", repo);
        }
    }

    let item = &settings
        .templates
        .iter()
        .any(|template| template.name == args.name.clone());
    if !item {
        let _ = &settings.templates.push(template);
    } else {
        println!("Template already exists in the settings.");
    }
    //}

    modify_settings(&settings);
    println!("{:?}", args.repos)
}
