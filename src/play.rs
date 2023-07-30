use std::process::Command;
//use subprocess::{Exec, Redirection};

use crate::settings;

pub fn play(settings: settings::Settings) {
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

    let mut cmd = Command::new(settings.doom_exe);
    cmd.arg("-iwad").arg(settings.iwad).arg("-file").arg(&settings.file);
    if let Some(save_game) = settings.save_game {
        cmd.arg("-loadgame").arg(save_game);
    }

    // Open Doom
    /*
    let result = Command::new(settings.doom_exe)
        .arg("-iwad")
        .arg(settings.iwad)
        .arg("-file")
        .arg(&settings.file)
        .status();
        //.spawn();
    */

    let result = cmd.status();

    match result {
        Ok(_) => println!("Opened the following file in Doom! - '{}'", settings.file),
        Err(e) => println!("Failed to run Doom! {:?}", e),
    }
}

/*
https://forum.zdoom.org/viewtopic.php?f=50&t=71819

Plus ChatGPT did tell me...
 */
