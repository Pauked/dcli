use std::{env, fs, path::PathBuf, process::Command};
use toml::Table;

const CONFIG_FILE: &str = "App.toml";
const KEY_DOOM_EXE: &str = "doom_exe";
const KEY_IWAD: &str = "iwad";
const KEY_FILE: &str = "file";

fn main() {
    let doom_exe = get_setting_from_config(CONFIG_FILE, KEY_DOOM_EXE);
    let iwad = get_setting_from_config(CONFIG_FILE, KEY_IWAD);
    let file = get_setting_from_config(CONFIG_FILE, KEY_FILE);

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

fn get_current_exe_path() -> String {
    let exe = env::current_exe().unwrap();
    let dir = exe.parent().expect("Executable must be in some directory");
    dir.display().to_string()
}

fn get_setting_from_config(config_file: &str, key: &str) -> String {
    // Build up config file name
    let mut file_path = PathBuf::new();
    file_path.push(get_current_exe_path());
    file_path.push(config_file);

    // Read key from config file
    let contents = fs::read_to_string(file_path.clone())
        .unwrap_or_else(|_| panic!("Error reading config file - '{}'", file_path.display()));
    let values = contents.parse::<Table>().unwrap();

    // Get the value
    let value: &str = values[key].as_str().unwrap(); // removing ' around value

    value.to_string()
}
