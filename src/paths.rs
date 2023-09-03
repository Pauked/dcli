use std::{env, path::{Path, PathBuf}};

use log::error;

pub fn get_current_exe() -> String {
    let exe_result = env::current_exe();
    match exe_result {
        Ok(exe) => {
            return exe.display().to_string();
        }
        Err(e) => {
            error!("Failed to get current exe: {:?}", e);
        }
    }
    String::new()
}

// pub fn folder_exists(folder_path: &str) -> bool {
//     let path = Path::new(folder_path);
//     path.is_dir()
// }

pub fn file_exists(file_path: &str) -> bool {
    let path = Path::new(file_path);
    path.is_file()
}

pub fn get_full_path(base_path: &str, file_name: &str) -> String {
    let mut file_path = PathBuf::new();
    file_path.push(base_path);
    file_path.push(file_name);
    file_path.display().to_string()
}

pub fn get_temp_dir() -> String {
    let temp_dir = env::temp_dir();
    temp_dir.display().to_string()
}
