use chrono::Utc;
use colored::Colorize;
use eyre::Context;
use inquire::validator::Validation;
use tabled::{
    builder::Builder,
    settings::{object::Rows, Modify, Style, Width},
};

use crate::{data, db, menu_common, paths, tui};

pub fn new_profile() -> Result<String, eyre::Report> {
    let engines = db::get_engines()?;
    if engines.is_empty() {
        return Ok("There are no Engines to select. Please run 'init'"
            .red()
            .to_string());
    }
    let iwads = db::get_iwads()?;
    if iwads.is_empty() {
        return Ok("There are no IWADs to select. Please run 'init"
            .red()
            .to_string());
    }
    let maps = db::get_maps()?;
    if maps.is_empty() {
        return Ok("There are no Maps to select. Please run 'init'"
            .red()
            .to_string());
    }

    let app_settings = db::get_app_settings()?;
    let engine_starting_cursor = match app_settings.default_engine_id {
        Some(ref i) => engines.iter().position(|x| x.id == *i).unwrap(),
        None => 0,
    };
    let iwad_starting_cursor = match app_settings.default_iwad_id {
        Some(ref i) => iwads.iter().position(|x| x.id == *i).unwrap(),
        None => 0,
    };

    let profile_name = inquire::Text::new("Enter a name for your Profile:")
        .with_validator(|input: &str| {
            let profile_result = db::get_profile_by_name(input);
            if let Ok(profile) = profile_result {
                if profile.name == input {
                    return Ok(Validation::Invalid("Profile name already exists.".into()));
                }
            }

            if input.len() < 5 {
                Ok(Validation::Invalid(
                    "Profile name must be at least 5 characters.".into(),
                ))
            } else {
                Ok(Validation::Valid)
            }
        })
        .prompt()?;

    let engine_selection = inquire::Select::new("Pick the Engine you want to use:", engines)
        .with_starting_cursor(engine_starting_cursor)
        .with_page_size(tui::MENU_PAGE_SIZE)
        .prompt()?;

    let iwad_selection = inquire::Select::new("Pick the IWAD you want to use:", iwads)
        .with_starting_cursor(iwad_starting_cursor)
        .with_page_size(tui::MENU_PAGE_SIZE)
        .prompt()?;

    let map_selection = menu_common::get_map_selection(maps, vec![])?;
    let map_id = Some(map_selection[0].id).filter(|&id| id > 0);
    let map_id2 = Some(map_selection[1].id).filter(|&id| id > 0);
    let map_id3 = Some(map_selection[2].id).filter(|&id| id > 0);
    let map_id4 = Some(map_selection[3].id).filter(|&id| id > 0);
    let map_id5 = Some(map_selection[4].id).filter(|&id| id > 0);

    let additional_arguments =
        inquire::Text::new("Enter any additional arguments (optional):").prompt_skippable()?;

    let profile = data::Profile {
        id: 0,
        name: profile_name,
        engine_id: Some(engine_selection.id),
        iwad_id: Some(iwad_selection.id),
        map_id,
        map_id2,
        map_id3,
        map_id4,
        map_id5,
        additional_arguments,
        date_created: Utc::now(),
        date_edited: Utc::now(),
        date_last_run: None,
    };
    let add_result = db::add_profile(profile.clone())?;
    let new_profile_id: i32 = add_result.last_insert_rowid().try_into().unwrap();
    set_profile_as_default(new_profile_id)?;

    Ok(format!(
        "Successfully created a new Profile - '{}'",
        profile.name.green()
    ))
}

pub fn cli_new_profile(
    name: &str,
    engine: &str,
    iwad: &str,
    maps_in: Option<Vec<String>>,
    args: Option<Vec<String>>,
) -> Result<String, eyre::Report> {
    let engines = db::get_engines()?;
    if engines.is_empty() {
        return Ok("There are no Engines to select. Please run 'init'"
            .red()
            .to_string());
    }
    let iwads = db::get_iwads()?;
    if iwads.is_empty() {
        return Ok("There are no IWADs to select. Please run 'init"
            .red()
            .to_string());
    }
    let maps = db::get_maps()?;
    if maps.is_empty() {
        return Ok("There are no Maps to select. Please run 'init'"
            .red()
            .to_string());
    }

    // Check profile name is unique
    let profile_result = db::get_profile_by_name(name);
    if let Ok(profile) = profile_result {
        if profile.name == name {
            return Ok("Profile name already exists.".into());
        }
    }
    if name.len() < 5 {
        return Ok("Profile name must be at least 5 characters.".into());
    }

    let engine_selection = match engines
        .iter()
        .find(|&x| x.path.to_lowercase() == engine.to_lowercase())
    {
        Some(engine) => engine,
        None => return Ok("Engine not found".into()),
    };

    let iwad_selection = match iwads
        .iter()
        .find(|&x| x.path.to_lowercase() == iwad.to_lowercase())
    {
        Some(iwad) => iwad,
        None => return Ok("IWAD not found".into()),
    };

    let map_ids: Vec<Option<i32>> = match maps_in {
        Some(maps_unwrapped) => {
            if maps_unwrapped.len() > 5 {
                return Ok("Only up to 5 maps can be specified".into());
            }
            let mut map_ids: Vec<Option<i32>> = Vec::new();
            for map in maps_unwrapped {
                match maps.iter().find(|&x| {
                    paths::extract_file_name(&x.path).to_lowercase() == map.to_lowercase()
                }) {
                    Some(map) => map_ids.push(Some(map.id)),
                    None => map_ids.push(None),
                };
            }

            if map_ids.len() < 5 {
                for _ in map_ids.len()..5 {
                    map_ids.push(None);
                }
            }

            map_ids
        }
        None => {
            vec![None, None, None, None, None]
        }
    };

    let additional_arguments = args.map(|args_unwrapped| args_unwrapped.join(" "));

    let profile = data::Profile {
        id: 0,
        name: name.to_string(),
        engine_id: Some(engine_selection.id),
        iwad_id: Some(iwad_selection.id),
        map_id: map_ids[0],
        map_id2: map_ids[1],
        map_id3: map_ids[2],
        map_id4: map_ids[3],
        map_id5: map_ids[4],
        additional_arguments,
        date_created: Utc::now(),
        date_edited: Utc::now(),
        date_last_run: None,
    };
    db::add_profile(profile.clone())?;

    Ok(format!(
        "Successfully created a new Profile - '{}'",
        profile.name.green()
    ))
}

pub fn set_profile_as_default(profile_id: i32) -> Result<String, eyre::Report> {
    if inquire::Confirm::new("Would you like to set this as your Default Profile?")
        .with_default(false)
        .prompt()
        .unwrap()
    {
        let mut app_settings = db::get_app_settings()?;
        app_settings.default_profile_id = Some(profile_id);
        db::save_app_settings(app_settings).wrap_err("Failed to set Default profile")?;
        return Ok("Successfully set Profile as Default".to_string());
    }

    Ok("No changes made to setting Profile as Default".to_string())
}

pub fn edit_profile() -> Result<String, eyre::Report> {
    let profile_list = db::get_profile_display_list()?;
    if profile_list.is_empty() {
        return Ok("There are no profiles to edit.".red().to_string());
    }
    let engines = db::get_engines()?;
    if engines.is_empty() {
        return Ok("There are no Engines to select. Please run 'init'"
            .red()
            .to_string());
    }
    let iwads = db::get_iwads()?;
    if iwads.is_empty() {
        return Ok("There are no IWADs to select. Please run 'init"
            .red()
            .to_string());
    }
    let maps = db::get_maps()?;
    if maps.is_empty() {
        return Ok("There are no Maps to select. Please run 'init'"
            .red()
            .to_string());
    }

    let profile_display = inquire::Select::new("Pick the Profile to Edit:", profile_list)
        .with_page_size(tui::MENU_PAGE_SIZE)
        .prompt()?;

    let engine_starting_cursor = engines
        .iter()
        .position(|engine| profile_display.engine_id == engine.id)
        .unwrap_or(0);

    let iwad_starting_cursor = iwads
        .iter()
        .position(|iwad| profile_display.iwad_id == iwad.id)
        .unwrap_or(0);

    let ids = &profile_display.map_ids;
    let default_maps: Vec<usize> = [ids.0, ids.1, ids.2, ids.3, ids.4]
        .iter()
        .filter_map(|&id| maps.iter().position(|map| map.id == id))
        .collect();

    let profile_name = inquire::Text::new("Enter a name for your Profile:")
        .with_validator(move |input: &str| {
            let profile_result = db::get_profile_by_name(input);
            if let Ok(profile) = profile_result {
                if profile.id != profile_display.id && profile.name == input {
                    return Ok(Validation::Invalid("Profile name already exists.".into()));
                }
            }

            if input.len() < 5 {
                Ok(Validation::Invalid(
                    "Profile name must be at least 5 characters.".into(),
                ))
            } else {
                Ok(Validation::Valid)
            }
        })
        .with_default(&profile_display.name)
        .prompt()?;

    let engine_selection = inquire::Select::new("Pick the Engine you want to use:", engines)
        .with_starting_cursor(engine_starting_cursor)
        .with_page_size(tui::MENU_PAGE_SIZE)
        .prompt()?;

    let iwad_selection = inquire::Select::new("Pick the IWAD you want to use:", iwads)
        .with_starting_cursor(iwad_starting_cursor)
        .with_page_size(tui::MENU_PAGE_SIZE)
        .prompt()?;

    let map_selection = menu_common::get_map_selection(maps, default_maps)?;
    let map_id = Some(map_selection[0].id).filter(|&id| id > 0);
    let map_id2 = Some(map_selection[1].id).filter(|&id| id > 0);
    let map_id3 = Some(map_selection[2].id).filter(|&id| id > 0);
    let map_id4 = Some(map_selection[3].id).filter(|&id| id > 0);
    let map_id5 = Some(map_selection[4].id).filter(|&id| id > 0);

    let additional_arguments = inquire::Text::new("Enter any additional arguments (optional):")
        .with_default(&profile_display.additional_arguments)
        .prompt_skippable()?;

    let profile = data::Profile {
        id: profile_display.id,
        name: profile_name.clone(),
        engine_id: Some(engine_selection.id),
        iwad_id: Some(iwad_selection.id),
        map_id,
        map_id2,
        map_id3,
        map_id4,
        map_id5,
        additional_arguments,
        date_created: profile_display.date_created,
        date_edited: Utc::now(),
        date_last_run: profile_display.date_last_run,
    };
    db::update_profile(profile)?;

    Ok(format!("Successfully updated Profile - '{}'", profile_name))
}

pub fn delete_profile() -> Result<String, eyre::Report> {
    let profile_list = db::get_profile_display_list()?;
    if profile_list.is_empty() {
        return Ok("There are no Profiles to delete.".red().to_string());
    }

    let profile_selection =
        inquire::Select::new("Pick the Profile to Delete:", profile_list).prompt_skippable()?;

    if let Some(profile) = profile_selection {
        if inquire::Confirm::new(&format!(
            "Are you sure you want to delete this Profile - '{}'? This cannot be undone.",
            profile.name
        ))
        .with_default(false)
        .prompt()
        .unwrap()
        {
            // Check if "Default Profile" and remove link if so
            remove_profile_from_app_settings(profile.id)?;

            // Now delete the profile
            db::delete_profile(profile.id)
                .wrap_err(format!("Failed to delete Profile - '{}", profile))?;
            return Ok(format!("Successfully deleted Profile '{}'", profile));
        }
    }

    Ok("Cancelled Profile deletion.".yellow().to_string())
}

pub fn set_default_profile() -> Result<String, eyre::Report> {
    let profile_list = db::get_profile_display_list()?;
    if profile_list.is_empty() {
        return Ok(
            "Cannot set Default Profile. There are no Profiles found. Please create one."
                .red()
                .to_string(),
        );
    }

    let mut app_settings = db::get_app_settings()?;
    let starting_cursor = match app_settings.default_profile_id {
        Some(ref s) => profile_list.iter().position(|x| x.id == *s).unwrap(),
        None => 0,
    };

    let profile = inquire::Select::new("Pick the Profile to mark as Default:", profile_list)
        .with_starting_cursor(starting_cursor)
        .with_page_size(tui::MENU_PAGE_SIZE)
        .prompt_skippable()?;

    match profile {
        Some(profile) => {
            app_settings.default_profile_id = Some(profile.id);
            db::save_app_settings(app_settings).wrap_err("Failed to set Default Profile")?;
            Ok(format!("Marked Profile '{}' as Default", profile))
        }
        None => Ok("No changes made to setting Profile as Default".to_string()),
    }
}

pub fn list_profiles(list_type: data::ListType) -> Result<String, eyre::Report> {
    let profiles =
        db::get_profile_display_list().wrap_err("Unable to profile listing".to_string())?;

    let table = match list_type {
        data::ListType::Full => tabled::Table::new(profiles)
            .with(Modify::new(Rows::new(1..)).with(Width::wrap(30).keep_words()))
            .with(Style::modern())
            .to_string(),
        data::ListType::Summary => {
            let mut builder = Builder::default();
            builder.set_header([
                "Name",
                "Engine App Name",
                "Engine Version",
                "IWAD File",
                "Map Files",
                "Additional Args",
                "Date Last Run",
            ]);
            for profile in profiles {
                builder.push_record([
                    profile.name,
                    profile.engine_app_name,
                    profile.engine_version,
                    profile.iwad_file,
                    data::display_combined_tabled_map_strings(&profile.map_files),
                    profile.additional_arguments,
                    data::display_option_utc_datetime_to_local(&profile.date_last_run),
                ]);
            }
            let mut table = builder.build();
            table
                .with(Modify::new(Rows::new(1..)).with(Width::wrap(50).keep_words()))
                .with(Style::modern())
                .to_string()
        }
    };
    Ok(table)
}

fn remove_profile_from_app_settings(profile_id: i32) -> Result<String, eyre::Report> {
    // TODO: Make this "profile in use" check less ugly
    let mut app_settings = db::get_app_settings()?;
    let mut default_profile_tidied = false;
    if let Some(default_profile_id) = app_settings.default_profile_id {
        if default_profile_id == profile_id {
            app_settings.default_profile_id = None;
            default_profile_tidied = true;
        }
    }
    let mut last_profile_tidied = false;
    if let Some(last_profile_id) = app_settings.last_profile_id {
        if last_profile_id == profile_id {
            app_settings.last_profile_id = None;
            last_profile_tidied = true;
        }
    }
    if default_profile_tidied || last_profile_tidied {
        db::save_app_settings(app_settings).wrap_err("Failed to remove Default Profile")?;
    }

    Ok("Successfully removed Profile from App Settings".to_string())
}

// fn remove_engine_from_app_settings(engine_id: i32) -> Result<String, eyre::Report> {
// }
