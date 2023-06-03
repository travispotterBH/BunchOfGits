use std::fs;
use std::fs::File;
use std::io::BufReader;

use serde_derive::{Serialize, Deserialize};

pub fn init() -> std::io::Result<()> {

    // need to define the best place for this. operating system agnostic. maybe allow
    // user to specifiy on install
    let path = "/Users/travispotter/Desktop/Code/Rust/BOG_Test".to_string();
    let bog_path = path + "/.bog";
    let path = String::from(bog_path.clone() + "/.bog_settings");

     //Testing out serialization and reading/writing to files.
    let mut things: Vec<TestStruct> = Vec::new();

    let mut index = 0;
    while index < 5 {
        things.push(TestStruct{item: String::from("one thing")});
        index += 1;
    }

    let settings = Settings{current_dir: path.clone(), items: things};

    let new_serialized_string = toml::to_string(&settings);

    if let Err(e) = fs::write(&path, &new_serialized_string.unwrap_or("fail".to_string())){
        panic!("{}",e);
    }

    let file = std::fs::read_to_string(&path)?;

    let read_file: Settings = toml::from_str(&file).unwrap_or(Settings{current_dir: path.clone(), items: Vec::new()});
    println!("{:?}", read_file);
    Ok(()) 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestStruct {
    item: String
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Settings{
    current_dir: String,
    items: Vec<TestStruct>,
}

