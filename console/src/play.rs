use std::process::Command;
//use subprocess::{Exec, Redirection};

use crate::constants;
use crate::settings;

pub fn play() {
    let doom_exe =
        settings::get_setting_from_config(constants::CONFIG_FILE, constants::KEY_DOOM_EXE);
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

    /*
        let gzdoom = Exec::cmd(doom_exe)
            .arg("-iwad")
            .arg(iwad) // Replace with the path to your IWAD file
            .arg("-file")
            .arg(&file) // Replace with the path to your MOD file
            .stdout(Redirection::Merge)
            .stderr(Redirection::Merge)
            .join()
            .expect("Failed to run GZDoom.");

        if !gzdoom.success() {
            eprintln!("GZDoom execution failed.");
        }
    */

    // Open Doom
    let result = Command::new(doom_exe)
        .arg("-iwad")
        .arg(iwad)
        .arg("-file")
        .arg(&file)
        .status();
        //.spawn();

    match result {
        Ok(_) => println!("Opened the following file in Doom! - '{}'", file),
        Err(e) => println!("Failed to run Doom! {:?}", e),
    }
}


/*
https://forum.zdoom.org/viewtopic.php?f=50&t=71819

Plus ChatGPT did tell me...
 */