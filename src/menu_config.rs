use color_eyre::eyre;
use colored::Colorize;
use eyre::Context;
use inquire::validator::Validation;
use log::{info, debug};
use tabled::settings::{object::Rows, Modify, Style, Width};

use crate::{
    data::{self},
    db, doom_data, finder, paths, tui,
};

pub async fn config_menu() -> Result<String, eyre::Report> {
    loop {
        let menu_command = tui::config_menu_prompt();
        if let tui::ConfigCommand::Back = menu_command {
            return Ok("Back to main menu".to_string());
        }
        run_config_menu_option(menu_command).await?;
    }
}

pub async fn run_config_menu_option(
    menu_command: tui::ConfigCommand,
) -> Result<String, eyre::Report> {
    match menu_command {
        tui::ConfigCommand::List => list_settings().await,
        tui::ConfigCommand::Init => init().await,
        tui::ConfigCommand::UpdateEngines => {
            let settings = db::get_settings().await?;
            init_engines(&settings.exe_search_folder.unwrap_or("".to_string())).await
        }
        tui::ConfigCommand::UpdateIwads => {
            let settings = db::get_settings().await?;
            init_iwads(&settings.iwad_search_folder.unwrap_or("".to_string())).await
        }
        tui::ConfigCommand::UpdatePwads => {
            let settings = db::get_settings().await?;
            init_pwads(&settings.pwad_search_folder.unwrap_or("".to_string())).await
        }
        tui::ConfigCommand::Reset => reset(false).await,
        tui::ConfigCommand::Back => Ok("Back to main menu".to_string()),
        tui::ConfigCommand::Unknown => Ok("Unknown command".to_string()),
    }
}

pub async fn init() -> Result<String, eyre::Report> {
    db::create_db().await?;
    // TODO: Block running if there is data. Or perhaps prompt to reset?

    info!("We'll ask you some questions, and then you'll be ready to go.");

    let exe_search_folder = init_engines("").await?;
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

pub async fn init_engines(default_folder: &str) -> Result<String, eyre::Report> {
    let exe_search_folder: String = inquire::Text::new("Folder to search for Doom engines")
        .with_validator(|input: &str| {
            if paths::folder_exists(input) {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid("This is not a valid folder".into()))
            }
        })
        .with_default(default_folder)
        .prompt()?;

    // TODO: User filter for exists (what do you want to search for?)
    // TODO: List for Windows, list for Mac
    let doom_engine_list = doom_data::get_engine_list();
    let doom_engine_files = doom_engine_list
        .iter()
        .map(|e| e.exe_name.as_str())
        .collect::<Vec<&str>>();

    let engines = paths::find_file_in_folders(&exe_search_folder, doom_engine_files);
    if engines.is_empty() {
        return Err(eyre::eyre!(format!(
            "No matches found using recursive search in folder '{}'",
            &exe_search_folder
        )));
    }

    // Work out the indexes of what is already selected
    let db_engines = db::get_engines().await?;
    let mut db_defaults = vec![];
    for (index, engine) in engines.iter().enumerate() {
        if db_engines.iter().any(|e| &e.path == engine) {
            db_defaults.push(index);
        }
    }

    // Multi-select prompt to user
    let selections = inquire::MultiSelect::new("Pick the engines you want to use", engines)
        .with_default(&db_defaults)
        .prompt()?;

    // Remove entries that were not selected but have entries in the database
    for db_engine in &db_engines {
        if !selections.contains(&db_engine.path) {
            db::delete_engine(&db_engine.path).await?;
            debug!("Deleted engine: {:?}", db_engine)
        }
    }

    // Save engines to  engines table
    for selection in selections {
        let game_engine_type =
            get_game_engine_type_from_exe_name(doom_engine_list.clone(), &selection)?;
        let selection_version = get_version_from_exe_name(&selection, game_engine_type.clone())?;

        let existing_engine = db_engines.iter().find(|e| e.path == selection);
        if let Some(existing_engine) = existing_engine {
            debug!("Engine already exists, no need to add: {}", selection);
            if existing_engine.version != selection_version {
                debug!("Updating engine version from '{}' to '{}'", existing_engine.version, selection_version);
                db::update_engine_version(existing_engine.id, &selection_version).await?;
            }
        } else {
            let engine = data::Engine {
                path: selection.clone(),
                version: selection_version,
                game_engine_type,
                id: 0,
            };
            db::add_engine(&engine).await?;
            debug!("Added engine: {:?}", engine);
        }
    }

    info!("{}", display_engines().await?);

    Ok(exe_search_folder)
}

pub async fn init_iwads(default_folder: &str) -> Result<String, eyre::Report> {
    // Search for IWADs
    // Use the same folder as the engines, but given option to change
    // Save to IWADs table
    let iwad_search_folder: String =
        inquire::Text::new("Folder to search for IWADs (Internal WAD files)")
            .with_validator(|input: &str| {
                if paths::folder_exists(input) {
                    Ok(Validation::Valid)
                } else {
                    Ok(Validation::Invalid("This is not a valid folder".into()))
                }
            })
            .with_default(default_folder)
            .prompt()?;

    // TODO: User filter for exists (what do you want to search for?)
    // TODO: List for Windows, list for Mac
    // TODO: If editing (not init), then we can search but not overwrite existing IWADs
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

    // TODO: Mark the IWADs that have been picked previously
    let selections = inquire::MultiSelect::new("Pick the IWADs you want to use", iwads).prompt()?;

    // Save engines to  engines table
    for selection in selections {
        let internal_wad_type =
            get_internal_wad_type_from_file_name(iwad_list.clone(), &selection)?;

        let iwad = data::Iwad {
            path: selection,
            internal_wad_type,
            id: 0,
        };

        // TODO: Check iwad doesn't exist
        db::add_iwad(&iwad).await?;
    }

    info!("{}", display_iwads().await?);

    Ok(iwad_search_folder)
}

pub async fn init_pwads(default_folder: &str) -> Result<String, eyre::Report> {
    let pwad_search_folder: String =
        inquire::Text::new("Folder to search for PWADs (Patch WAD files)")
            .with_validator(|input: &str| {
                if paths::folder_exists(input) {
                    Ok(Validation::Valid)
                } else {
                    Ok(Validation::Invalid("This is not a valid folder".into()))
                }
            })
            .with_default(default_folder)
            .prompt()?;

    // TODO: Extend to go find PWADs. Need a wildcard search method for this. And to remove IWADS from list.
    let pwads = paths::find_files_with_extension_in_folders(&pwad_search_folder, "wad");
    if pwads.is_empty() {
        return Err(eyre::eyre!(format!(
            "No matches found using recursive search in folder '{}'",
            &pwad_search_folder
        )));
    }

    // TODO: Loading existing PWADs up, see if the listing matches. Remove any that don't exist anymore. Add any new ones.
    for pwad in pwads {
        let pwad = data::Pwad {
            name: get_map_name_from_readme(&pwad)?,
            path: pwad.clone(),
            id: 0,
        };
        // TODO: Check if PWAD already exists
        db::add_pwad(&pwad).await?;
    }

    info!("{}", display_pwads().await?);

    Ok(pwad_search_folder)
}

fn get_map_name_from_readme(pwad: &str) -> Result<String, eyre::Report> {
    // TODO: Write method to get map name from associated map readme!

    //let path = paths::extract_path(pwad);
    let file_name = paths::extract_file_name(pwad);
    // replace the wad extension with readme
    let readme = pwad.replace(".wad", ".txt");
    if !paths::file_exists(&readme) {
        return Ok(file_name);
    }

    let lines = paths::lines_from_file("readme", &readme)?;
    for line in lines {
        if line.starts_with("Title") {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() > 1 {
                return Ok(parts[1].trim().to_string());
            }
        }
    }

    Ok(file_name)
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

pub async fn list_settings() -> Result<String, eyre::Report> {
    info!("{}", display_engines().await?);
    info!("{}", display_iwads().await?);
    info!("{}", display_pwads().await?);
    info!("{}", display_settings().await?);
    // info!("{}", display_profiles().await?);
    Ok("".to_string())
}

pub async fn display_engines() -> Result<String, eyre::Report> {
    let engines = db::get_engines()
        .await
        .wrap_err("Unable to generate engine listing".to_string())?;

    let table = tabled::Table::new(engines)
        .with(Modify::new(Rows::new(1..)).with(Width::wrap(50).keep_words()))
        .with(Style::modern())
        .to_string();
    Ok(table)
}

pub async fn display_iwads() -> Result<String, eyre::Report> {
    let iwads = db::get_iwads()
        .await
        .wrap_err("Unable to iwad listing".to_string())?;

    let table = tabled::Table::new(iwads)
        .with(Modify::new(Rows::new(1..)).with(Width::wrap(30).keep_words()))
        .with(Style::modern())
        .to_string();
    Ok(table)
}

pub async fn display_pwads() -> Result<String, eyre::Report> {
    let pwads = db::get_pwads()
        .await
        .wrap_err("Unable to iwad listing".to_string())?;

    let table = tabled::Table::new(pwads)
        .with(Modify::new(Rows::new(1..)).with(Width::wrap(30).keep_words()))
        .with(Style::modern())
        .to_string();
    Ok(table)
}

pub async fn display_settings() -> Result<String, eyre::Report> {
    let settings = db::get_settings()
        .await
        .wrap_err("Unable to settings listing".to_string())?;

    let table = tabled::Table::new(vec![settings])
        .with(Modify::new(Rows::new(1..)).with(Width::wrap(30).keep_words()))
        .with(Style::modern())
        .to_string();
    Ok(table)
}

async fn reset(force: bool) -> Result<String, eyre::Report> {
    if !db::database_exists().await {
        return Ok("Database does not exist, nothing to reset.".to_string());
    }

    // Prompt the user for confirmation to delete the file
    if force
        || inquire::Confirm::new("Do you want to reset the database? All data will be deleted.")
            .with_default(false)
            .prompt()
            .unwrap()
    {
        db::reset_db().wrap_err("Failed to reset database.")?;
        Ok("Successfully reset database.".green().to_string())
    } else {
        Ok("Database reset not confirmed.".to_string())
    }
}
