use std::process::Command;

use crate::settings;

pub fn editor(settings: settings::Settings) {
    //let editor_exe = settings::get_setting_from_config(constants::CONFIG_FILE, constants::KEY_EDITOR_EXE);
    //let file = settings::get_setting_from_config(constants::CONFIG_FILE, constants::KEY_FILE);

    //println!("{} - {}", editor_exe, file);

    // Open Editor
    let result = Command::new(settings.editor_exe)
        .arg(&settings.file)
        //.arg(format!("'{}'", &file))
        .spawn();

    match result {
        Ok(_) => println!("Opened the following file in Editor! - '{}'", settings.file),
        Err(e) => println!("Failed to Editor! {:?}", e),
    }
}