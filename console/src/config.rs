use std::process::Command;

use crate::{settings, constants};

pub fn config() {
    // Get config file path
    let config_file_path = settings::get_config_filename(constants::CONFIG_FILE);

    // Open the App.toml file in notepad
    let result = Command::new("notepad.exe")
        .arg(config_file_path.clone())
        .spawn();

    match result {
        Ok(_) => println!("Opened the following file in Notepad! - '{}'", config_file_path.display()),
        Err(e) => println!("Failed to open file in Notepad: {:?}", e),
    }
}