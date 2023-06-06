use std::io::{Result, Write};

pub fn request_information(request: &str) -> Result<String>{

    let mut additional_info = String::new();
    print!("{}", request);
    std::io::stdout().flush().unwrap(); // Make sure the prompt is immediately displayed
    std::io::stdin().read_line(&mut additional_info).unwrap();

    println!("Got additional info: {}", additional_info.trim());
    Ok(additional_info.to_string())
} 

//use
/*
*
    let request = "This request is for information. Give it to me.";
    request_information(&request)?;
*
*/


