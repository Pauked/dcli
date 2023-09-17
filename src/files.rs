use std::path::Path;

use crate::{data, doom_data, finder, paths};

pub fn get_map_readme_file_name(pwad: &str) -> Result<Option<String>, eyre::Report> {
    let path = Path::new(pwad);
    let extension = path.extension().unwrap_or_default();

    // Try for wad first
    if extension == doom_data::EXT_WAD {
        let readme = pwad.replace(
            &format!(".{}", doom_data::EXT_WAD),
            &format!(".{}", doom_data::EXT_TXT),
        );
        if paths::file_exists(&readme) {
            return Ok(Some(readme));
        }
    }

    // Try for pk3
    if extension == doom_data::EXT_PK3 {
        let readme = pwad.replace(
            &format!(".{}", doom_data::EXT_PK3),
            &format!(".{}", doom_data::EXT_TXT),
        );
        if paths::file_exists(&readme) {
            return Ok(Some(readme));
        }
    }

    Ok(None)
}

pub fn get_map_name_from_readme(pwad: &str) -> Result<String, eyre::Report> {
    if let Some(readme) = get_map_readme_file_name(pwad)? {
        let lines = paths::lines_from_file("readme", &readme)?;
        for line in lines {
            if line.to_lowercase().starts_with("title") {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() > 1 {
                    return Ok(parts[1].trim().to_string());
                }
            }
        }
    }

    Ok(paths::extract_file_name(pwad))
}

pub fn get_version_from_exe_name(
    exe_name: &str,
    game_engine_type: doom_data::GameEngineType,
) -> Result<data::FileVersion, eyre::Report> {
    match game_engine_type {
        doom_data::GameEngineType::Doom => todo!("Doom version not implemented yet!"),
        doom_data::GameEngineType::PrBoomPlus => Ok(finder::get_prboom_file_version(exe_name)?),
        doom_data::GameEngineType::GzDoom => Ok(finder::get_file_version(exe_name)?),
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

// #[cfg(test)]
// mod tests {
//     #[cfg(target_os = "windows")]
//     #[test]
//     fn get_map_readme_file_name_tntr_wad() {
//         // Arrange
//         use super::get_map_readme_file_name;
//         let pwad = r"C:\Doom\Maps\PWAD\tntr\tntr.wad";
//         let expected = r"C:\Doom\Maps\PWAD\tntr\tntr.txt";

//         // Act
//         let actual = get_map_readme_file_name(pwad);

//         // Assert
//         assert_eq!(actual.unwrap().unwrap(), expected);
//     }

//     #[cfg(target_os = "windows")]
//     #[test]
//     fn get_map_readme_file_name_lullaby_pk3() {
//         // Arrange
//         use super::get_map_readme_file_name;
//         let pwad = r"C:\Doom\Maps\PWAD\Lullaby\Lullaby.pk3";
//         let expected = r"C:\Doom\Maps\PWAD\Lullaby\Lullaby.txt";

//         // Act
//         let actual = get_map_readme_file_name(pwad);

//         // Assert
//         assert_eq!(actual.unwrap().unwrap(), expected);
//     }

//     #[cfg(target_os = "windows")]
//     #[test]
//     fn get_map_readme_file_name_ramppk3_no_readme() {
//         // Arrange
//         use super::get_map_readme_file_name;
//         let pwad = r"C:\Doom\Maps\PWAD\RAMP\RAMP.pk3";
//         let expected = None;

//         // Act
//         let actual = get_map_readme_file_name(pwad);

//         // Assert
//         assert_eq!(actual.unwrap(), expected);
//     }
// }
