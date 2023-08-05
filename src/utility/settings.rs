use directories::*;
use std::fs;
use std::io::*;
use std::path::Path;
use std::u8;

use serde_derive::{Deserialize, Serialize};

fn settings_directory() -> Result<String> {
    if let Some(base_dirs) = BaseDirs::new() {
        let home_dir: &Path = &base_dirs.home_dir();
        let bog_path = format!("{}{}", home_dir.to_str().unwrap(), "/.config/BunchOfGits");
        let path = Path::new(&bog_path);
        if !path.exists() || !path.is_dir() {
            println!("New directory created.");
            if let Ok(()) = fs::create_dir(&bog_path) {
                return Ok(bog_path);
            };
        }
        return Ok(bog_path);
    };

    return Err(Error::new(
        ErrorKind::Other,
        "Could not find or create directory.",
    ));
}

pub fn settings_path() -> Result<String> {
    if let Err(_e) = settings_directory() {
        return Err(Error::new(
            ErrorKind::NotFound,
            "Base directories not found",
        ));
    };

    if let Some(base_dirs) = BaseDirs::new() {
        let home_dir: &Path = &base_dirs.home_dir();
        return Ok(format!(
            "{}{}",
            home_dir.to_str().unwrap(),
            "/.config/BunchOfGits/.bog_settings"
        ));
    };

    return Err(Error::new(
        ErrorKind::NotFound,
        "Base directories not found",
    ));
}

pub fn get_settings() -> Option<Settings> {
    let path = settings_path();
    if let Ok(file) = std::fs::read_to_string(&path.unwrap()) {
        if let Ok(settings) = toml::from_str(&file) {
            return Some(settings);
        };
    }

    let settings = crate::utility::settings::create_default_settings();
    let _ = crate::utility::settings::write_settings(&settings);
    println!("No settings were found. Default settings created.");
    return Some(settings);
}

pub fn write_settings(settings: &Settings) -> Result<()> {
    let path = settings_path();
    let serialized_string = toml::to_string(&settings);
    if let Err(e) = fs::write(&path.unwrap(), &serialized_string.unwrap()) {
        return Err(e);
    };
    Ok(())
}

pub fn create_default_settings() -> Settings {
    let settings = Settings::default();
    let _ = write_settings(&settings);
    return settings;
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Bunch {
    pub name: String,
    pub items: Vec<Item>,
}

impl Bunch {
    pub fn new(name: String) -> Bunch {
        let mut new = Bunch::default();
        new.name = name;
        return new;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub repo: String, //eventually will be "Repo" type
    pub branch: String,
    //pub stash: String,
    //pub prev_branch: String,
    //pub build_order: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Repo {
    pub path: String,
    pub nickname: String,
}

impl Repo {
    pub fn new(path: String, nickname: Option<String>) -> Self {
        Repo {
            path,
            nickname: nickname.unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Template{
    pub name: String,
    pub repos: Vec<Repo>,
}

impl Template {
    pub fn new(name: String) -> Template{
        let mut new = Template::default();
        new.name = name;
        return new;
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Settings {
    pub current_dir: String,
    pub repos: Vec<Repo>,
    pub bunches: Vec<Bunch>,
}
