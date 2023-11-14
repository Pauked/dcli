use std::{
    env,
    fs::{self, File},
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
};

use eyre::Context;
use indicatif::{ProgressBar, ProgressStyle};
use owo_colors::OwoColorize;
use walkdir::WalkDir;

use crate::constants;

pub fn get_current_exe() -> String {
    let exe_result = env::current_exe();
    match exe_result {
        Ok(exe) => {
            return exe.display().to_string();
        }
        Err(e) => {
            log::error!("Failed to get current exe: {:?}", e);
        }
    }
    String::new()
}

pub fn resolve_path(folder_path: &str) -> String {
    if env::consts::OS == constants::OS_MACOS && folder_path.starts_with("~/") {
        if let Some(home) = dirs::home_dir() {
            // Replace ~ with the home directory
            return folder_path.replacen('~', &home.to_string_lossy(), 1);
        }
    }

    folder_path.to_string()
}

pub fn resolve_path_opt(folder_path: Option<String>) -> Option<String> {
    folder_path.map(|path| resolve_path(&path))
}

pub fn folder_exists(folder_path: &str) -> bool {
    let path = Path::new(folder_path);
    path.exists() && path.is_dir()
}

pub fn file_exists(file_path: &str) -> bool {
    let path = Path::new(file_path);
    path.exists() && path.is_file()
}

pub fn create_folder(folder_path: &str) -> io::Result<()> {
    fs::create_dir_all(folder_path)
}

pub fn create_file(file_name: &str) -> io::Result<File> {
    fs::File::create(file_name)
}

pub fn open_file(file_name: &str) -> io::Result<File> {
    fs::File::open(file_name)
}

pub fn delete_folder(folder_path: &str) -> io::Result<()> {
    fs::remove_dir_all(folder_path)
}

pub fn delete_file(file_path: &str) -> io::Result<()> {
    fs::remove_file(file_path)
}

pub fn extract_path(full_path: &str) -> String {
    let path = Path::new(full_path);

    if let Some(parent_path) = path.parent() {
        return parent_path.display().to_string();
    }

    String::new()
}

pub fn extract_file_name(full_path: &str) -> String {
    let path = Path::new(full_path);
    path.file_name()
        .and_then(|file_name| file_name.to_str())
        .map(|s| s.to_string())
        .unwrap_or_default()
}

pub fn extract_file_name_no_extension(full_path: &str) -> String {
    let path = Path::new(full_path);
    path.file_stem()
        .and_then(|name| name.to_str())
        .map(|s| s.to_string())
        .unwrap_or_default()
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

pub fn get_base_name(file_path: &str) -> String {
    Path::new(file_path)
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned()
}

pub fn find_file_in_folders(
    root_folder: &str,
    find_files: Vec<&str>,
    search_message: &str,
) -> Vec<String> {
    log::debug!(
        "find_file_in_folders: '{}', '{:?}', '{}'",
        root_folder,
        find_files,
        search_message
    );
    let mut results: Vec<String> = Vec::new();

    // Create a new progress bar
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::with_template("{spinner:.blue} [{elapsed_precise}] {msg}").unwrap(),
    );

    let mut found_count = 0;

    for entry in WalkDir::new(root_folder) {
        match entry {
            Ok(entry) => {
                // Check if the file name matches
                for file in &find_files {
                    if entry.file_name().to_string_lossy().to_lowercase() == file.to_lowercase() {
                        found_count += 1;
                        results.push(entry.path().display().to_string());
                    }
                }

                // Set the message to the currently-searched directory
                pb.set_message(format!(
                    "({}) Searching for {}: '{}'",
                    get_matches_count(found_count),
                    search_message,
                    truncate_middle(&entry.path().display().to_string(), 80)
                ));
            }
            Err(e) => {
                log::debug!("Error reading directory entry: {:?}", e);
            }
        }

        pb.inc(1); // Increase the spinner's step
    }

    log::debug!("Match files found - {:?}", results);
    results
}

pub fn find_files_with_extensions_in_folders(
    root_folder: &str,
    extensions: Vec<&str>,
    search_message: &str,
) -> Vec<String> {
    log::debug!(
        "find_files_with_extensions_in_folders: '{}' / '{:?}', '{}'",
        root_folder,
        extensions,
        search_message
    );
    let mut results: Vec<String> = Vec::new();

    // Create a new progress bar
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::with_template("{spinner:.blue} [{elapsed_precise}] {msg}").unwrap(),
    );

    let mut found_count = 0;

    for entry in WalkDir::new(root_folder) {
        match entry {
            Ok(entry) => {
                if let Some(ext) = entry.path().extension() {
                    for extension in extensions.clone() {
                        if ext.to_string_lossy().to_lowercase() == extension.to_lowercase() {
                            found_count += 1;
                            results.push(entry.path().display().to_string());
                        }
                    }
                }

                // Set the message to the currently-searched directory
                pb.set_message(format!(
                    "({}) Searching for {}: '{}'",
                    get_matches_count(found_count),
                    search_message,
                    truncate_middle(&entry.path().display().to_string(), 80)
                ));
            }
            Err(e) => {
                log::debug!("Error reading directory entry: {:?}", e);
            }
        }

        pb.inc(1); // Increase the spinner's step
    }

    log::debug!("Match files found - {:?}", results);
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
    // Yes this method is horrible. It is coping with folder paths that may
    // contain unicode characters and if truncated incorrectly will cause a
    // "assertion failed: self.is_char_boundary(n)" error.

    if input.len() <= size_limit {
        // No need to truncate, return the original string.
        return input.to_string();
    }

    // debug!("truncate_middle: '{}' / {}", input, size_limit);
    let half_size_limit = size_limit.saturating_sub(2) / 2; // Make space for ".."

    // Find the start and end byte indices directly, adjusting for character boundaries.
    let mut start_byte_index = input.len() / 2 - half_size_limit;
    while !input.is_char_boundary(start_byte_index) && start_byte_index < input.len() {
        start_byte_index += 1;
    }

    let mut end_byte_index = input.len() / 2 + half_size_limit;
    while !input.is_char_boundary(end_byte_index) && end_byte_index > 0 {
        end_byte_index -= 1;
    }

    // Construct the truncated string.
    let mut output = String::with_capacity(size_limit);
    output.push_str(&input[..start_byte_index]);
    output.push_str("..");
    output.push_str(&input[end_byte_index..]);
    output
}

pub fn lines_from_file(description: &str, filename: &str) -> Result<Vec<String>, eyre::Report> {
    log::debug!("Attempting {} file read '{}'", description, filename);

    // Build up file name
    let mut file_path = PathBuf::new();
    file_path.push(filename);

    // Open the file...
    let file_result = File::open(file_path.clone()).wrap_err(format!(
        "Error opening {} file '{}'",
        description,
        file_path.display()
    ))?;
    let buf = BufReader::new(file_result);
    log::debug!("  File successfully read");
    Ok(buf.lines().map(|l| l.unwrap_or_default()).collect())
}
