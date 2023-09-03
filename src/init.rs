use color_eyre::eyre;
use dialoguer::{theme::ColorfulTheme, Input, MultiSelect};
use log::info;

use crate::{
    actions, constants,
    data::{self},
    db, doom_data, finder, paths,
};

pub async fn init() -> Result<String, eyre::Report> {
    // TODO: Block running if there is data. Or perhaps prompt to reset?

    info!("Weclome to {}.", constants::APP_NAME);
    info!("We'll ask you some questions, and then you'll be ready to go.");

    let exe_search_folder = init_engines().await?;
    let iwad_search_folder = init_iwads(&exe_search_folder).await?;
    let pwad_search_folder = init_pwads(&iwad_search_folder).await?;

    let settings = data::Settings {
        id: 0,
        active_profile_id: None,
        exe_search_folder: Some(exe_search_folder),
        iwad_search_folder: Some(iwad_search_folder),
        pwad_search_folder: Some(pwad_search_folder),
    };
    db::add_settings(&settings).await?;

    Ok("Succesfully configured!".to_string())
}

async fn init_engines() -> Result<String, eyre::Report> {
    let exe_search_folder: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Folder to search for Doom engines")
        .validate_with({
            move |input: &String| -> Result<(), &str> {
                if paths::folder_exists(input) {
                    Ok(())
                } else {
                    Err("This is not a valid folder")
                }
            }
        })
        .interact_text()
        .unwrap();

    // TODO: User filter for exists (what do you want to search for?)
    // TODO: List for Windows, list for Mac
    let engine_list = doom_data::get_engine_list();
    let engine_files = engine_list
        .iter()
        .map(|e| e.exe_name.as_str())
        .collect::<Vec<&str>>();

    let engines = paths::find_file_in_folders(&exe_search_folder, engine_files);
    if engines.is_empty() {
        return Err(eyre::eyre!(format!(
            "No matches found using recursive search in folder '{}'",
            &exe_search_folder
        )));
    }

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick the engines you want to use")
        .items(&engines[..])
        .interact()
        .unwrap();

    // if selections.is_empty() {
    //     println!("You did not select anything :(");
    // } else {
    //     println!("You selected these things:");
    //     for selection in selections {
    //         println!("  {}", engines[selection]);
    //     }
    // }

    // Save engines to  engines table
    for selection in selections {
        let game_engine_type =
            get_game_engine_type_from_exe_name(engine_list.clone(), &engines[selection])?;

        let engine = data::Engine {
            path: engines[selection].clone(),
            version: get_version_from_exe_name(&engines[selection], game_engine_type.clone())?,
            game_engine_type,
            id: 0,
        };
        // TODO: Check engine doesn't already exist
        db::add_engine(&engine).await?;
    }

    info!("{}", actions::display_engines().await?);

    Ok(exe_search_folder)
}

async fn init_iwads(default_folder: &str) -> Result<String, eyre::Report> {
    // Search for IWADs
    // Use the same folder as the engines, but given option to change
    // Save to IWADs table
    let iwad_search_folder: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Folder to search for IWADs (Internal WAD files)")
        .validate_with({
            move |input: &String| -> Result<(), &str> {
                if paths::folder_exists(input) {
                    Ok(())
                } else {
                    Err("This is not a valid folder")
                }
            }
        })
        .default(default_folder.to_string())
        .interact_text()
        .unwrap();

    // TODO: User filter for exists (what do you want to search for?)
    // TODO: List for Windows, list for Mac
    let iwad_list = doom_data::get_internal_wad_list();
    let iwad_files = iwad_list
        .iter()
        .map(|e| e.file_name.as_str())
        .collect::<Vec<&str>>();

    let iwads = paths::find_file_in_folders(&iwad_search_folder, iwad_files);
    if iwads.is_empty() {
        return Err(eyre::eyre!(format!(
            "No matches found using recursive search in folder '{}'",
            &iwad_search_folder
        )));
    }

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick the IWADs you want to use")
        .items(&iwads[..])
        .interact()
        .unwrap();

    // Save engines to  engines table
    for selection in selections {
        let internal_wad_type =
            get_internal_wad_type_from_file_name(iwad_list.clone(), &iwads[selection])?;

        let iwad = data::Iwad {
            path: iwads[selection].clone(),
            internal_wad_type,
            id: 0,
        };

        // TODO: Check iwad doesn't exist
        db::add_iwad(&iwad).await?;
    }

    info!("{}", actions::display_iwads().await?);

    Ok(iwad_search_folder)
}

async fn init_pwads(default_folder: &str) -> Result<String, eyre::Report> {
    let pwad_search_folder: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Folder to search for PWADs (Patch WAD files)")
        .validate_with({
            move |input: &String| -> Result<(), &str> {
                if paths::folder_exists(input) {
                    Ok(())
                } else {
                    Err("This is not a valid folder")
                }
            }
        })
        .default(default_folder.to_string())
        .interact_text()
        .unwrap();

    // TODO: Extend to go find PWADs. Need a wildcard search method for this. And to remove IWADS from list.
    let pwads = paths::find_files_with_extension_in_folders(&pwad_search_folder, "wad");
    if pwads.is_empty() {
        return Err(eyre::eyre!(format!(
            "No matches found using recursive search in folder '{}'",
            &pwad_search_folder
        )));
    }

    for pwad in pwads {
        let pwad = data::Pwad {
            name: paths::extract_file_name(&pwad),
            path: pwad.clone(),
            id: 0,
        };
        db::add_pwad(&pwad).await?;
    }

    info!("{}", actions::display_pwads().await?);

    Ok(pwad_search_folder)
}

fn get_version_from_exe_name(
    exe_name: &str,
    game_engine_type: doom_data::GameEngineType,
) -> Result<String, eyre::Report> {
    match game_engine_type {
        doom_data::GameEngineType::Doom => todo!("Doom version not implemented yet!"),
        doom_data::GameEngineType::PrBoomPlus => {
            let file_version_result = finder::get_prboom_file_version(exe_name)?;
            Ok(format!(
                "{}.{}.{}.{}",
                file_version_result.major,
                file_version_result.minor,
                file_version_result.build,
                file_version_result.revision
            ))
        }
        doom_data::GameEngineType::GzDoom => {
            let file_version_result = finder::get_file_version(exe_name)?;
            Ok(format!(
                "{}.{}.{}.{}",
                file_version_result.major,
                file_version_result.minor,
                file_version_result.build,
                file_version_result.revision
            ))
        }
    }
}

fn get_game_engine_type_from_exe_name(
    engine_list: Vec<doom_data::GameEngine>,
    exe_name: &str,
) -> Result<doom_data::GameEngineType, eyre::Report> {
    // Get the exe name from the path
    let exe_name = paths::extract_file_name(exe_name);

    // Search the engine list for the exe name
    for engine in engine_list {
        if engine.exe_name.to_lowercase() == exe_name.to_lowercase() {
            return Ok(engine.game_engine_type);
        }
    }

    Err(eyre::eyre!(format!(
        "Unable to find engine type for exe name '{}'",
        exe_name
    )))
}

fn get_internal_wad_type_from_file_name(
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
