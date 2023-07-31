use directories::*;
use std::fs;
use std::io::*;
use std::path::Path;
use std::u8;

use chrono::prelude::*;

use serde_derive::{Deserialize, Serialize};

pub fn settings_directory() -> Result<String>{
    if let Some(base_dirs) = BaseDirs::new() {
        let home_dir: &Path = &base_dirs.home_dir();
        let bog_path = format!(
            "{}{}",
            home_dir.to_str().unwrap(),
            "/Desktop/Code/Rust/BOG_Test/.bog/"
        );

        let path = format!(
            "{}{}",
            home_dir.to_str().unwrap(),
            "/Desktop/Code/Rust/BOG_Test/.bog/.bog_settings"
        );


        let file = std::fs::read_to_string(&path);

        match file {
            Ok(_) => {}
            Err(_) => {
                println!("New directory created.");
                if let Ok(()) = fs::create_dir(&bog_path){
                    return Ok(path);
                };
                return Err(Error::new(ErrorKind::Other, "Could not find or create directory."));
            }
        }
    };

    return Err(Error::new(ErrorKind::Other, "Could not find or create directory."));
}

pub fn settings_path() -> Result<String>{
    if let Some(base_dirs) = BaseDirs::new() {
        let home_dir: &Path = &base_dirs.home_dir();
        return Ok(format!(
            "{}{}",
            home_dir.to_str().unwrap(),
            "/Desktop/Code/Rust/BOG_Test/.bog/.bog_settings"));
    };

return Err(Error::new(ErrorKind::NotFound, "Base directories not found"));
}

pub fn get_settings() -> Option<Settings> {
    if let Ok(path) = settings_path(){
        if let Ok(file) = std::fs::read_to_string(&path) {
            if let Ok(settings) = toml::from_str(&file){
                return Some(settings);
            } else {
                let mut settings = crate::utility::settings::create_default_settings();
                let _ = crate::utility::settings::write_settings(&settings);
                println!("No settings were found. Default settings created.");
                return Some(settings);
            };
        };
    };

    return None;
}


pub fn write_settings(settings: &Settings) -> Result<()>{
    if let Ok(path) = settings_path(){
        let serialized_string = toml::to_string(&settings);
        if let Err(e) = fs::write(&path, &serialized_string.unwrap()) {
            return Err(e);
        };
    };

    Ok(())
}


pub fn create_default_settings() -> Settings{
    let settings = Settings::default();
    let _ = write_settings(&settings);
    return settings;
}

pub fn init() -> std::io::Result<()> {
    match settings_directory() {
        Ok(bog_path) => {
            let path = format!(
                "{}{}",
                bog_path,
                "/.bog_settings"
            );

            let file = std::fs::read_to_string(&path);

            match file {
                Ok(_) => {}
                Err(_) => {
                    println!("New directory created.");
                    fs::create_dir(&bog_path)?;
                }
            }

            let settings = create_test_data();
            let serialized_string = toml::to_string(&settings);

            if let Err(e) = fs::write(&path, &serialized_string.unwrap_or("fail".to_string())) {
                panic!("{}", e);
            }

            let file = std::fs::read_to_string(&path)?;

            let read_file: Settings = toml::from_str(&file).unwrap_or(Settings {
                current_dir: path.clone(),
                bunches: Vec::new(),
                repos: Vec::new(),
            });

            println!("{:?}", read_file);
            Ok(())
        },

        Err(e) => {

            let error = Error::new(ErrorKind::Other, "OS specific root directory not found.");
            return Err(error)

        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bunch {
    pub name: String,
    pub items: Vec<Item>,
}


impl Bunch {
    pub fn new(){
        
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub repo: String, //eventually will be "Repo" type
    pub branch: String,
    pub stash: String,
    pub prev_branch: String,
    pub build_order: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Repo {
    pub path: String,
    pub nickname: String,
}

impl Repo {
    pub fn new(path: String, nickname: Option<String>) -> Self{
        Repo {
            path,
            nickname: nickname.unwrap(),
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub current_dir: String,
    pub repos: Vec<Repo>,
    pub bunches: Vec<Bunch>,
}


impl Default for Settings{
    fn default() -> Self {
        Self {
            current_dir: String::new(),
            repos: Vec::new(),
            bunches: Vec::new(),
        }
    }
}

fn create_test_data() -> Settings {
    let mut things: Vec<Item> = Vec::new();

    let mut index = 0;
    while index < 5 {
        things.push(Item {
            repo: String::from("Test repo"),
            branch: String::from("Test branch"),
            stash: String::from("Test stash"),
            prev_branch: String::from("Test prev branch"),
            build_order: index,
        });
        index += 1;
    }

    let mut bunch_index = 0;
    let mut bunches: Vec<Bunch> = Vec::new();

    while bunch_index < 5 {
        bunches.push(Bunch {
            name: format!("bunch: {}", bunch_index),
            items: things.clone(),
        });
        bunch_index += 1;
    }

    Settings {
        current_dir: String::new(),
        bunches,
        ..Default::default()
    }
}
