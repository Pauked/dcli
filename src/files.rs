use std::{
    fs::{self, File},
    io::Read,
    path::Path,
};

use log::debug;
use regex::Regex;
use strsim::levenshtein;

use crate::{constants, data, doom_data, finder, paths};

pub fn get_map_readme_file_name(map_path: &str) -> Result<Option<String>, eyre::Report> {
    let path = Path::new(map_path);
    let extension = path
        .extension()
        .unwrap_or_default()
        .to_str()
        .unwrap()
        .to_lowercase();

    let valid_extension = doom_data::GAME_FILES
        .iter()
        .any(|&e| e.to_lowercase() == extension);

    if valid_extension {
        let readme = map_path.to_lowercase().replace(
            &format!(".{}", extension),
            &format!(".{}", doom_data::EXT_TXT),
        );
        if paths::file_exists(&readme) {
            return Ok(Some(readme));
        }

        // Still not found, so lets try something more complicated...
        let find_result = get_map_readme_using_levenshtein(map_path)?;
        if let Some(readme) = find_result {
            return Ok(Some(readme));
        }
    }

    Ok(None)
}

fn get_map_readme_using_levenshtein(map_path: &str) -> Result<Option<String>, eyre::Report> {
    let input_file = Path::new(map_path);
    if let Some(directory) = input_file.parent() {
        // Get the file name without the extension or folder
        // also strip any file versioning out
        let base_name = strip_versioning(&paths::get_base_name(input_file.to_str().unwrap()));

        // Get a list of txt files in the same folder as the map
        // score them using levenshtein and return the closest match (could be interesting if way off!)
        let closest_match = find_txt_files(directory).ok().and_then(|txt_files| {
            txt_files
                .into_iter()
                .map(|file| {
                    (
                        file.clone(),
                        levenshtein(&base_name, &paths::get_base_name(&file)),
                    )
                })
                .filter(|&(_, score)| score <= similarity_threshold(&base_name))
                .min_by_key(|&(_, score)| score)
                .map(|(filename, _)| filename)
        });

        if let Some(filename) = &closest_match {
            log::debug!("Closest readme file: {}", filename);
        } else {
            log::debug!("No readme file found for {}", input_file.display());
        }

        Ok(closest_match)
    } else {
        Ok(None)
    }
}

fn similarity_threshold(base_name: &str) -> usize {
    // Set the threshold as a function of the length of the base name.
    (base_name.len() as f64 * 0.5).round() as usize // 50% of the length
}

fn strip_versioning(name: &str) -> String {
    // Trying to strip out examples like "-2.29-RC1" or "-RC1"
    let re = Regex::new(r"[-_][a-zA-Z0-9]+(\.\d+)*([-_][a-zA-Z]+)?").unwrap();
    re.replace_all(name, "").to_string()
}

fn find_txt_files(directory: &Path) -> Result<Vec<String>, eyre::Report> {
    let mut txt_files = Vec::new();

    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().is_some_and(|e| e == doom_data::EXT_TXT) {
            txt_files.push(path.to_string_lossy().into_owned());
        }
    }

    Ok(txt_files)
}

fn check_readme_line(line: &str, key: &str) -> Option<String> {
    if line.to_lowercase().starts_with(key) {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() > 1 {
            return Some(parts[1].trim().to_string());
        }
    }
    None
}

pub fn get_details_from_readme(map_path: &str) -> Result<(String, String), eyre::Report> {
    let mut title = paths::extract_file_name(map_path);
    let mut title_found = false;
    let mut author = constants::DEFAULT_UNKNOWN.to_string();
    let mut author_found = false;

    if let Some(readme) = get_map_readme_file_name(map_path)? {
        let lines = paths::lines_from_file("readme", &readme)?;
        for line in lines {
            if let Some(value) = check_readme_line(&line, "title") {
                title = value;
                title_found = true;
            }
            if let Some(value) = check_readme_line(&line, "author") {
                author = value;
                author_found = true;
            }
            if title_found && author_found {
                break;
            }
        }
    }

    Ok((title, author))
}

pub fn get_version_from_exe_name(
    exe_name: &str,
    game_engine_type: doom_data::GameEngineType,
) -> Result<data::FileVersion, eyre::Report> {
    match game_engine_type {
        doom_data::GameEngineType::PrBoomPlus => Ok(finder::get_prboom_file_version(exe_name)?),
        doom_data::GameEngineType::GzDoom
        | doom_data::GameEngineType::CrispyDoom
        | doom_data::GameEngineType::EternityEngine
        | doom_data::GameEngineType::DoomRetro
        | doom_data::GameEngineType::Woof => Ok(finder::get_file_version(exe_name)?),
        doom_data::GameEngineType::Unknown => Err(eyre::eyre!("Unknown game engine type")),
    }
}

pub fn get_game_engine_from_exe_name(
    engine_list: Vec<doom_data::GameEngine>,
    exe_name: &str,
) -> Result<doom_data::GameEngine, eyre::Report> {
    // Get the exe name from the path
    let exe_name = paths::extract_file_name(exe_name);

    // Search the engine list for the exe name
    for engine in engine_list {
        if engine.exe_name.to_lowercase() == exe_name.to_lowercase() {
            return Ok(engine);
        }
    }

    Err(eyre::eyre!(format!(
        "Unable to find engine type for exe name '{}'",
        exe_name
    )))
}

pub fn get_internal_wad_type_from_file_name(
    iwad_list: Vec<doom_data::InternalWad>,
    path: &str,
) -> Result<doom_data::InternalWadType, eyre::Report> {
    // Get the file from the path
    let file_name = paths::extract_file_name(path);

    // Search the engine list for the exe name
    for iwad in iwad_list {
        if iwad.file_name.to_lowercase() == file_name.to_lowercase() {
            return Ok(iwad.internal_wad_type);
        }
    }

    Err(eyre::eyre!(format!(
        "Unable to find internal wad type for file name '{}'",
        file_name
    )))
}

fn is_valid_wad(file: &str, wad_identifier: &[u8; 4]) -> Result<bool, eyre::Report> {
    let path = Path::new(file);
    let extension = path.extension().unwrap_or_default();

    if extension.to_ascii_lowercase() == doom_data::EXT_WAD && paths::file_exists(file) {
        debug!("Checking if '{}' is a valid WAD file", file);
        let mut file = File::open(path)?;
        let mut identifier = [0u8; 4];
        file.read_exact(&mut identifier)?;
        return Ok(&identifier == wad_identifier);
    }

    Ok(false)
}
pub fn is_iwad(file: &str) -> Result<bool, eyre::Report> {
    is_valid_wad(file, &doom_data::IWAD_IDENTIFIER)
}

fn is_pwad(file: &str) -> Result<bool, eyre::Report> {
    is_valid_wad(file, &doom_data::PWAD_IDENTIFIER)
}

pub fn map_file_extension(file: &str) -> Result<bool, eyre::Report> {
    let path = Path::new(file);
    let extension = path
        .extension()
        .unwrap_or_default()
        .to_str()
        .unwrap()
        .to_lowercase();

    let valid_extension = doom_data::GAME_FILES
        .iter()
        .any(|&e| e.to_lowercase() == extension);

    if valid_extension && extension.to_ascii_lowercase() == doom_data::EXT_WAD {
        return is_pwad(file);
    }

    Ok(valid_extension)
}
