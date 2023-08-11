use directories::*;
use std::fs;
use std::io::*;
use std::path::Path;
use std::u8;

use serde_derive::{Deserialize, Serialize};

extern crate lazy_static;

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref SETTINGS: Mutex<Settings> = Mutex::new(Settings::default());
}

pub fn initialize_settings() -> Settings {
    let mut value = SETTINGS.lock().unwrap();
    *value = read_settings().unwrap();
    value.clone()
}

pub fn modify_settings(new_value: &Settings) {
    let mut value = SETTINGS.lock().unwrap();
    *value = new_value.clone();
}

pub fn get_settings() -> Settings {
    let value = SETTINGS.lock().unwrap();
    value.clone()
}

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

fn read_settings() -> Option<Settings> {
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

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Bunch {
    pub name: String,
    pub items: Vec<Item>,
}

impl Bunch {
    pub fn new(name: &str) -> Bunch {
        let mut new = Bunch::default();
        new.name = name.into();
        return new;
    }

    pub fn from_template(name: String, template: &Template) -> Bunch {
        let mut bunch = Bunch::default();
        bunch.name = name;
        for repo in template.repos.iter() {
            bunch.items.push(Item {
                repo: repo.path.clone(),
                branch: repo.default_branch.clone(),
            })
        }

        return bunch;
    }

    pub fn add_item(&mut self, item: Item) {
        if let None = self.items.iter().find(|i| i.repo == item.repo) {
            println!("Item of repo: '{}' on branch: {} added.", item.repo, item.branch);
            self.items.push(item);
        } else {
            println!("An item matching the one passed in already exists on this bunch.");
        };
    }

    pub fn remove_item(&mut self, item: &str) {
        if let Some(idx) = self.items.iter().position(|i| i.repo == item) {
            self.items.remove(idx);
            println!("Item of repo: '{}' removed.", item);
        } else {
            println!("No matching items.");
        };
    }

    pub fn update(&mut self, new: Bunch) {
        self.name = new.name;
        self.items = new.items;
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
    pub name: String,
    pub default_branch: String,
}

impl Repo {
    pub fn new(path: String, name: String, default_branch: String) -> Self {
        Repo {
            path,
            name,
            default_branch,
        }
    }

    pub fn update(&mut self, new: Repo) {
        self.path = new.path;
        self.name = new.name;
        self.default_branch = new.default_branch;
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Template {
    pub name: String,
    pub repos: Vec<Repo>,
}

impl Template {
    pub fn new(name: String) -> Template {
        let mut new = Template::default();
        new.name = name;
        return new;
    }

    pub fn add_repo(&mut self, repo: Repo) {
        if let None = self.repos.iter().find(|r| r.path == repo.path || r.name == repo.name) {
            println!("Repo: '{}' added.", repo.name);
            self.repos.push(repo);
        } else {
            println!("An item matching the one passed in already exists on this Template.");
        };
    }

    pub fn remove_repo(&mut self, item: &str) {
        if let Some(idx) = self.repos.iter().position(|r| r.name == item) {
            self.repos.remove(idx);
            println!("Repo: '{}' removed.", item);
        } else {
            println!("No matching items.");
        };
    }
    pub fn update(&mut self, new: Template) {
        self.name = new.name;
        self.repos = new.repos;
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Settings {
    pub current_dir: String,
    pub repos: Vec<Repo>,
    pub bunches: Vec<Bunch>,
    pub templates: Vec<Template>,
}

impl Settings {
    pub fn add_repo(&mut self, repo: Repo) {
        if let None = self.repos.iter().find(|r| r.name == repo.name) {
            println!("Repo: '{}' added.", repo.name);
            self.repos.push(repo);
        } else {
            println!("A repo of the name '{}' already exists.", repo.name);
        };
    }

    pub fn delete_repo(&mut self, repo_name: String) {
        if let Some(index) = self.repos.iter().position(|r| r.name == repo_name) {
            self.repos.swap_remove(index);
            println!("Repo: '{}' deleted.", repo_name);
        } else {
            println!("A repo of the name '{}' does not exist.", repo_name);
        };
    }

    pub fn update_repo(&mut self, repo_name: String, new: Repo) {
        if let Some(repo) = self.repos.iter_mut().find(|r| r.name == repo_name) {
            //let _ = std::mem::replace(repo, new);
            repo.update(new);
            println!("Repo: '{}' updated.", repo_name);
        } else {
            println!("A repo of the name '{}' does not exist.", repo_name);
        };
    }

    pub fn add_bunch(&mut self, bunch: &Bunch) {
        if let None = self.bunches.iter().find(|b| b.name == bunch.name) {
            println!("Bunch: '{}' added.", bunch.name);
            self.bunches.push(bunch.clone());
        } else {
            println!("A bunch of the name '{}' already exists.", bunch.name);
        };
    }

    pub fn delete_bunch(&mut self, bunch_name: String) {
        if let Some(index) = self.bunches.iter().position(|b| b.name == bunch_name) {
            self.bunches.swap_remove(index);
            println!("Bunch: '{}' deleted.", bunch_name);
        } else {
            println!("A bunch of the name '{}' does not exist.", bunch_name);
        };
    }

    pub fn update_bunch(&mut self, bunch_name: String, new: Bunch) {
        if let Some(bunch) = self.bunches.iter_mut().find(|b| b.name == bunch_name) {
            //let _ = std::mem::replace(bunch, new);
            bunch.update(new);
            println!("Bunch: '{}' updated.", bunch_name);
        } else {
            println!("A bunch of the name '{}' does not exist.", bunch_name);
        };
    }

    pub fn add_template(&mut self, template: Template) {
        if let None = self.templates.iter().find(|t| t.name == template.name) {
            println!("Template: '{}' added.", template.name);
            self.templates.push(template);
        } else {
            println!("A template of the name '{}' already exists.", template.name);
        };
    }

    pub fn delete_template(&mut self, template_name: String) {
        if let Some(index) = self.templates.iter().position(|t| t.name == template_name) {
            println!("Template: '{}' deleted.", template_name);
            self.templates.swap_remove(index);
        } else {
            println!("A template of the name '{}' does not exist.", template_name);
        };
    }

    pub fn update_template(&mut self, template_name: String, new: Template) {
        if let Some(template) = self.templates.iter_mut().find(|b| b.name == template_name) {
            //let _ = std::mem::replace(template, new);
            template.update(new);
            println!("Template: '{}' updated.", template_name);
        } else {
            println!("A template of the name '{}' does not exist.", template_name);
        };
    }
}
