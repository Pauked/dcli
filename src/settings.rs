use serde::Deserialize;
use std::{env, fs, path::PathBuf, process};

#[derive(Deserialize)]
pub struct Settings {
    pub doom_exe: String,
    pub iwad: String,
    pub file: String,
    pub editor_exe: String,
    pub save_game: Option<String>,
}

pub fn get(config_file_path: PathBuf) -> Settings {
    let contents = match fs::read_to_string(config_file_path.clone()) {
        // If successful return the files text as `contents`.
        // `c` is a local variable.
        Ok(c) => c,
        // Handle the `error` case.
        Err(_) => {
            // Write `msg` to `stderr`.
            eprintln!("Could not read file '{}'", config_file_path.display());
            // Exit the program with exit code `1`.
            process::exit(1)
        }
    };

    let settings: Settings = match toml::from_str(&contents) {
        // If successful, return data as `Data` struct.
        // `d` is a local variable.
        Ok(d) => d,
        // Handle the `error` case.
        Err(_) => {
            // Write `msg` to `stderr`.
            eprintln!("Unable to load data from `{}`", config_file_path.display());
            // Exit the program with exit code `1`.
            process::exit(1);
        }
    };

    settings
}

pub fn get_config_filename(config_file: &str) -> PathBuf {
    let mut file_path = PathBuf::new();
    file_path.push(get_current_exe_path());
    file_path.push(config_file);
    file_path
}

fn get_current_exe_path() -> String {
    let exe = env::current_exe().unwrap();
    let dir = exe.parent().expect("Executable must be in some directory");
    dir.display().to_string()
}
