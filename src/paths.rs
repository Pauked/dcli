use std::{env, path::{Path, PathBuf}};

use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use log::{error, debug};
use walkdir::WalkDir;

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

pub fn folder_exists(folder_path: &str) -> bool {
    let path = Path::new(folder_path);
    path.is_dir()
}

pub fn file_exists(file_path: &str) -> bool {
    let path = Path::new(file_path);
    path.is_file()
}

pub fn extract_file_name(full_path: &str) -> String {
    let path = Path::new(full_path);
    path.file_name().and_then(|file_name| file_name.to_str())
        .map(|s| s.to_string())
        .unwrap_or_else(String::new)
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

pub fn find_file_in_folders(root_folder: &str, find_files: Vec<&str>) -> Vec<String> {
    debug!("find_file_in_folders: '{}'", root_folder);
    let mut results: Vec<String> = Vec::new();

    // Create a new progress bar
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::with_template("{spinner:.blue} [{elapsed_precise}] {msg}").unwrap(),
    );

    let mut found_count = 0;

    for entry in WalkDir::new(root_folder) {
        if let Ok(entry) = entry {
            // Check if the file name matches
            for file in &find_files {
                if entry.file_name().to_string_lossy().to_lowercase() == file.to_lowercase() {
                    found_count += 1;
                    results.push(entry.path().display().to_string());
                }
            }

            // Set the message to the currently-searched directory
            pb.set_message(format!(
                "({}) Searching: '{}'",
                get_matches_count(found_count),
                truncate_middle(&entry.path().display().to_string(), 80)
            ));
        }

        pb.inc(1); // Increase the spinner's step
    }

    debug!("Match files found - {:?}", results);
    results
}

fn get_matches_count(found_count: i32) -> String {
    if found_count == 0 {
        return "0 matches".to_string();
    }
    let result = format!("{} matches", found_count);
    result.green().to_string()
}

fn truncate_middle(input: &str, size_limit: usize) -> String {
    let input_len = input.len();

    if input_len <= size_limit {
        // No need to truncate, return the original string.
        return input.to_string();
    }

    let middle_index = input_len / 2;
    let half_size_limit = size_limit / 2;
    let start_index = middle_index - half_size_limit;
    let end_index = middle_index + half_size_limit;

    // Remove the middle section from the string.
    let mut output: String = input.to_string();
    output.replace_range(start_index..end_index, "..");
    output
}