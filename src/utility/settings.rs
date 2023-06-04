use directories::*;
use std::fs;
use std::io::*;
use std::path::Path;

use serde_derive::{Deserialize, Serialize};

pub fn init() -> std::io::Result<()> {
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
                fs::create_dir(&bog_path)?;
            }
        }

        let settings = create_test_data(&path);
        let serialized_string = toml::to_string(&settings);

        if let Err(e) = fs::write(&path, &serialized_string.unwrap_or("fail".to_string())) {
            panic!("{}", e);
        }

        let file = std::fs::read_to_string(&path)?;

        let read_file: Settings = toml::from_str(&file).unwrap_or(Settings {
            current_dir: path.clone(),
            items: Vec::new(),
        });

        println!("{:?}", read_file);
        Ok(())
    } else {
        let error = Error::new(ErrorKind::Other, "OS specific root directory not found.");
        Err(error)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestStruct {
    item: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    current_dir: String,
    items: Vec<TestStruct>,
}

fn create_test_data(path: &String) -> Settings {
    let mut things: Vec<TestStruct> = Vec::new();

    let mut index = 0;
    while index < 5 {
        things.push(TestStruct {
            item: String::from("one thing"),
        });
        index += 1;
    }

    Settings {
        current_dir: path.clone(),
        items: things,
    }
}
