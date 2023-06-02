use std::{process::Command, path::PathBuf};

pub fn config(config_file_path: PathBuf) {
    // Open the App.toml file in notepad
    let result = Command::new("notepad.exe")
        .arg(config_file_path.clone())
        .spawn();

    match result {
        Ok(_) => println!("Opened the following file in Notepad! - '{}'", config_file_path.display()),
        Err(e) => println!("Failed to open file in Notepad: {:?}", e),
    }
}