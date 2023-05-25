use std::process::Command;

use crate::constants;
use crate::settings;

pub fn play() {
    let doom_exe = settings::get_setting_from_config(constants::CONFIG_FILE, constants::KEY_DOOM_EXE);
    let iwad = settings::get_setting_from_config(constants::CONFIG_FILE, constants::KEY_IWAD);
    let file = settings::get_setting_from_config(constants::CONFIG_FILE, constants::KEY_FILE);

    println!("{} - {} - {}", doom_exe, iwad, file);

    // Change the current working dir
    /*
    let path = Path::new(&doom_exe);
    let directory = path.parent().unwrap();
    println!("Directory path: {}", &directory.display());
    env::set_current_dir(directory).unwrap();
    */

    // Open Doom
    let result = Command::new(doom_exe)
        .arg("-iwad")
        .arg(iwad)
        .arg("-file")
        .arg(&file)
        .status();

    match result {
        Ok(_) => println!("Opened the following file in Doom! - {}", file),
        Err(e) => println!("Failed to Doom! {:?}", e),
    }
}