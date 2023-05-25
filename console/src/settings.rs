use std::{env, fs, path::PathBuf};
use toml::Table;

fn get_current_exe_path() -> String {
    let exe = env::current_exe().unwrap();
    let dir = exe.parent().expect("Executable must be in some directory");
    dir.display().to_string()
}

pub fn get_config_filename(config_file: &str) -> PathBuf {
    let mut file_path = PathBuf::new();
    file_path.push(get_current_exe_path());
    file_path.push(config_file);
    file_path
}

pub fn get_setting_from_config(config_file: &str, key: &str) -> String {
    // Build up config file name
    let config_file_path = get_config_filename(config_file);

    // Read key from config file
    let contents = fs::read_to_string(config_file_path.clone())
        .unwrap_or_else(|_| panic!("Error reading config file - '{}'", config_file_path.display()));
    let values = contents.parse::<Table>().unwrap();

    // Get the value
    let value: &str = values[key].as_str().unwrap(); // removing ' around value

    value.to_string()
}
