use chrono::Utc;
use eyre::Context;
use inquire::validator::Validation;
use owo_colors::OwoColorize;
use tabled::{
    builder::Builder,
    settings::{object::Rows, Modify, Style, Width},
};

use crate::{constants, data, db, menu_app_settings, menu_common, menu_queues, paths, tui};

pub fn add_profile(
    map_id: Option<i32>,
    profile_name: Option<String>,
) -> Result<String, eyre::Report> {
    let engines = db::get_engines()?;
    if engines.is_empty() {
        return Ok("There are no Engines to select. Please run 'init'".to_string());
    }
    let iwads = db::get_iwads()?;
    if iwads.is_empty() {
        return Ok("There are no IWADs to select. Please run 'init".to_string());
    }
    let maps = db::get_maps()?;
    if maps.is_empty() {
        return Ok("There are no Maps to select. Please run 'init'".to_string());
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
            if profile_result.is_ok() {
                return Ok(Validation::Invalid("Profile name already exists".into()));
            }

            if input.len() < constants::MIN_NAME_LENGTH {
                Ok(Validation::Invalid(
                    format!(
                        "Profile name must be at least {} characters",
                        constants::MIN_NAME_LENGTH
                    )
                    .into(),
                ))
            } else {
                Ok(Validation::Valid)
            }
        })
        .with_default(&profile_name.unwrap_or_default())
        .prompt()?;

    let engine_selection = inquire::Select::new("Pick the Engine you want to use:", engines)
        .with_starting_cursor(engine_starting_cursor)
        .with_page_size(tui::MENU_PAGE_SIZE)
        .with_formatter(&|i| i.value.simple_display())
        .prompt()?;

    let iwad_selection = inquire::Select::new("Pick the IWAD you want to use:", iwads)
        .with_starting_cursor(iwad_starting_cursor)
        .with_page_size(tui::MENU_PAGE_SIZE)
        .with_formatter(&|i| i.value.simple_display())
        .prompt()?;

    let (map_id, map_id2, map_id3, map_id4, map_id5) = match map_id {
        Some(map_id) => (Some(map_id), None, None, None, None),
        None => {
            let map_selection = menu_common::get_map_selection(maps, vec![])?;
            (
                Some(map_selection[0].id).filter(|&id| id > 0),
                Some(map_selection[1].id).filter(|&id| id > 0),
                Some(map_selection[2].id).filter(|&id| id > 0),
                Some(map_selection[3].id).filter(|&id| id > 0),
                Some(map_selection[4].id).filter(|&id| id > 0),
            )
        }
    };

    let save_game =
        inquire::Text::new("Enter save game file name you want to automatically load (optional):")
            .with_help_message("For example with GZDoom 'save26.zds'")
            .prompt_skippable()?;

    let additional_arguments =
        inquire::Text::new("Enter any additional arguments (optional):").prompt_skippable()?;

    let profile = data::Profile {
        id: 0,
        name: profile_name.clone(),
        engine_id: Some(engine_selection.id),
        iwad_id: Some(iwad_selection.id),
        map_id,
        map_id2,
        map_id3,
        map_id4,
        map_id5,
        save_game,
        additional_arguments,
        date_created: Utc::now(),
        date_edited: Utc::now(),
        date_last_run: None,
        run_count: 0,
    };
    let add_result = db::add_profile(profile.clone())?;
    let add_profile_id: i32 = add_result.last_insert_rowid().try_into().unwrap();
    set_profile_as_default(add_profile_id, &profile.name, false)?;

    // Add new Profile to existing Queue?
    let queue_prompt_result = inquire::Confirm::new(&format!(
        "Would you like to add this Profile '{}' to a Queue?",
        profile_name
    ))
    .with_default(false)
    .prompt_skippable()?;

    if let Some(true) = queue_prompt_result {
        let profile_display = db::get_profile_display_by_id(add_profile_id)?;
        let queue_result = menu_queues::add_profile_to_queue(Some(profile_display));
        match queue_result {
            Ok(success_message) => {
                log::info!("  {}", success_message);
            }
            Err(e) => {
                log::error!("  Failed to add Profile to Queue: {}", e);
            }
        }
    }

    Ok(format!(
        "Successfully created a new Profile - '{}'",
        profile.name
    ))
}

pub fn cli_add_profile(
    name: &str,
    engine: &str,
    iwad: &str,
    maps_in: Option<Vec<String>>,
    save_game: Option<String>,
    args: Option<Vec<String>>,
) -> Result<String, eyre::Report> {
    let engines = db::get_engines()?;
    if engines.is_empty() {
        return Ok(format!(
            "Cannot add Profile '{}', There are no Engines to select. Please run 'init'",
            name
        ));
    }
    let iwads = db::get_iwads()?;
    if iwads.is_empty() {
        return Ok(format!(
            "Cannot add Profile '{}', There are no IWADs to select. Please run 'init",
            name
        ));
    }
    let maps = db::get_maps()?;
    if maps.is_empty() {
        return Ok(format!(
            "Cannot add Profile '{}', There are no Maps to select. Please run 'init'",
            name
        ));
    }

    // Check profile name is unique
    let profile_result = db::get_profile_by_name(name);
    if profile_result.is_ok() {
        return Ok(format!(
            "Cannot add Profile '{}'. Profile name already exists",
            name
        ));
    }
    if name.len() < constants::MIN_NAME_LENGTH {
        return Ok(format!(
            "Cannot add Profile '{}'. Profile name must be at least {} characters",
            name,
            constants::MIN_NAME_LENGTH
        ));
    }

    let engine_selection = match engines
        .iter()
        .find(|&x| x.path.to_lowercase() == engine.to_lowercase())
    {
        Some(engine) => engine,
        None => {
            return Ok(format!(
                "Cannot add Profile '{}'. Engine not found - '{}'",
                name, engine
            ))
        }
    };

    let iwad_selection = match iwads
        .iter()
        .find(|&x| x.path.to_lowercase() == iwad.to_lowercase())
    {
        Some(iwad) => iwad,
        None => {
            return Ok(format!(
                "Cannot add Profile '{}'. IWAD not found - '{}'",
                name, iwad
            ))
        }
    };

    let map_ids: Vec<Option<i32>> = match maps_in {
        Some(maps_unwrapped) => {
            if maps_unwrapped.len() > 5 {
                return Ok(format!(
                    "Cannot add Profile '{}'. A max of 5 maps can be specified per Profile",
                    name
                ));
            }
            let mut map_ids: Vec<Option<i32>> = Vec::new();
            for map in maps_unwrapped {
                match maps.iter().find(|&x| {
                    // ASSUMPTION CORNER: We match on the first file we find,
                    // so if JEFF.WAD is in db 10 times, we'll match on the first one.
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
        save_game,
        additional_arguments,
        date_created: Utc::now(),
        date_edited: Utc::now(),
        date_last_run: None,
        run_count: 0,
    };
    db::add_profile(profile.clone())?;

    Ok(format!(
        "Successfully created a new Profile - '{}'",
        profile.name
    ))
}

pub fn set_profile_as_default(
    profile_id: i32,
    profile_name: &str,
    force: bool,
) -> Result<String, eyre::Report> {
    if force
        || inquire::Confirm::new("Would you like to set this as your Default Profile?")
            .with_default(false)
            .prompt()?
    {
        let mut app_settings = db::get_app_settings()?;
        app_settings.default_profile_id = Some(profile_id);
        db::save_app_settings(app_settings).wrap_err("Failed to set Default profile")?;
        return Ok(format!(
            "Successfully set Default Profile as '{}'",
            profile_name
        ));
    }

    Ok("No changes made to setting Profile as Default".to_string())
}

pub fn cli_set_default_profile(name: &str) -> Result<String, eyre::Report> {
    let profile_result = db::get_profile_by_name(name);
    if let Ok(profile) = profile_result {
        set_profile_as_default(profile.id, &profile.name, true)
    } else {
        Ok(format!(
            "Cannot set Default Profile. Profile not found - '{}'",
            name
        ))
    }
}

pub fn edit_profile() -> Result<String, eyre::Report> {
    let profile_list = db::get_profile_display_list(data::ProfileOrder::Name)?;
    if profile_list.is_empty() {
        return Ok("There are no profiles to edit".red().to_string());
    }
    let engines = db::get_engines()?;
    if engines.is_empty() {
        return Ok("There are no Engines to select. Please run 'init'".to_string());
    }
    let iwads = db::get_iwads()?;
    if iwads.is_empty() {
        return Ok("There are no IWADs to select. Please run 'init".to_string());
    }
    let maps = db::get_maps()?;
    if maps.is_empty() {
        return Ok("There are no Maps to select. Please run 'init'".to_string());
    }

    let profile_display = inquire::Select::new("Pick the Profile to Edit:", profile_list)
        .with_page_size(tui::MENU_PAGE_SIZE)
        .with_formatter(&|i| i.value.simple_display())
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
                if profile.id != profile_display.id {
                    return Ok(Validation::Invalid("Profile name already exists".into()));
                }
            }

            if input.len() < constants::MIN_NAME_LENGTH {
                Ok(Validation::Invalid(
                    format!(
                        "Profile name must be at least {} characters",
                        constants::MIN_NAME_LENGTH
                    )
                    .into(),
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
        .with_formatter(&|i| i.value.simple_display())
        .prompt()?;

    let iwad_selection = inquire::Select::new("Pick the IWAD you want to use:", iwads)
        .with_starting_cursor(iwad_starting_cursor)
        .with_page_size(tui::MENU_PAGE_SIZE)
        .with_formatter(&|i| i.value.simple_display())
        .prompt()?;

    let map_selection = menu_common::get_map_selection(maps, default_maps)?;
    let map_id = Some(map_selection[0].id).filter(|&id| id > 0);
    let map_id2 = Some(map_selection[1].id).filter(|&id| id > 0);
    let map_id3 = Some(map_selection[2].id).filter(|&id| id > 0);
    let map_id4 = Some(map_selection[3].id).filter(|&id| id > 0);
    let map_id5 = Some(map_selection[4].id).filter(|&id| id > 0);

    let save_game =
        inquire::Text::new("Enter save game file name you want to automatically load (optional):")
            .with_help_message("For example with GZDoom 'save26.zds'")
            .with_default(&profile_display.save_game)
            .prompt_skippable()?;

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
        save_game,
        additional_arguments,
        date_created: profile_display.date_created,
        date_edited: Utc::now(),
        date_last_run: profile_display.date_last_run,
        run_count: profile_display.run_count,
    };
    db::update_profile(profile)?;

    Ok(format!("Successfully updated Profile - '{}'", profile_name))
}

pub fn change_engine_on_profile() -> Result<String, eyre::Report> {
    let engine_list = db::get_engines()?;
    if engine_list.is_empty() {
        return Ok(
            "Cannot change Engine on Profiles because there are no Engines to select".to_string(),
        );
    }

    let display_profile_list = db::get_profile_display_list(data::ProfileOrder::Name)?;
    if display_profile_list.is_empty() {
        return Ok("There are no Profiles to change the Engine on".to_string());
    }

    let before_engine =
        inquire::Select::new("Pick the Engine to change from:", engine_list.clone())
            .with_page_size(tui::MENU_PAGE_SIZE)
            .with_formatter(&|i| i.value.simple_display())
            .prompt()?;

    // Get a list of profiles that are using the before_engine
    let filtered_display_profiles = display_profile_list
        .iter()
        .filter(|&profile| profile.engine_id == before_engine.id)
        .collect::<Vec<&data::ProfileDisplay>>();

    if filtered_display_profiles.is_empty() {
        return Ok(format!(
            "No changes made to changing Engine on Profiles. No Profiles are using the Engine - '{}'",
            before_engine.short_display()
        ));
    }

    let after_engine = inquire::Select::new("Pick the Engine to change to:", engine_list)
        .with_page_size(tui::MENU_PAGE_SIZE)
        .with_formatter(&|i| i.value.simple_display())
        .prompt()?;

    // Which profiles do we want to change?
    let selected_display_profiles = inquire::MultiSelect::new(
        "Pick the Profiles you want to change:",
        filtered_display_profiles,
    )
    .with_page_size(tui::MENU_PAGE_SIZE)
    .with_formatter(&|i| {
        i.iter()
            .map(|e| e.value.short_display())
            .collect::<Vec<String>>()
            .join(", ")
    })
    .with_help_message(&format!(
        "These Profiles are using the Engine - '{}'",
        before_engine.short_display()
    ))
    .prompt()?;

    // Abort if nothing picked
    if selected_display_profiles.is_empty() {
        return Ok(
            "No changes made to changing Engine on Profiles. No Profiles selected".to_string(),
        );
    }

    // Update the Profiles with the new Engine
    for display_profile in &selected_display_profiles {
        let mut profile = db::get_profile_by_id(display_profile.id)?;
        profile.engine_id = Some(after_engine.id);
        db::update_profile(profile)?;
    }

    Ok(format!(
        "Successfully changed Engine from '{}' to '{}' on {} Profiles",
        before_engine.short_display(),
        after_engine.short_display(),
        selected_display_profiles.len()
    ))
}

pub fn change_save_game_on_profile() -> Result<String, eyre::Report> {
    let display_profile_list = db::get_profile_display_list(data::ProfileOrder::Name)?;
    if display_profile_list.is_empty() {
        return Ok("There are no Profiles to edit the Save Game on".to_string());
    }

    let profile_display = inquire::Select::new(
        "Pick the Profile to edit the Save Game on:",
        display_profile_list,
    )
    .with_page_size(tui::MENU_PAGE_SIZE)
    .with_formatter(&|i| i.value.simple_display())
    .prompt()?;

    let save_game =
        inquire::Text::new("Enter save game file name you want to automatically load (optional):")
            .with_help_message("For example with GZDoom 'save26.zds'")
            .with_default(&profile_display.save_game)
            .prompt_skippable()?;

    if save_game.clone().unwrap_or_default() != profile_display.save_game {
        db::update_profile_save_game(profile_display.id, save_game.clone())?;

        Ok(format!(
            "Successfully updated Save Game to '{}' on Profile '{}'",
            save_game.unwrap_or(constants::DEFAULT_NOT_SET.to_string()),
            profile_display.name
        ))
    } else {
        Ok(format!(
            "No changes made to Save Game on Profile '{}'",
            profile_display.name
        ))
    }
}

fn delete_profile_core(
    profile_id: i32,
    profile_name: &str,
    force: bool,
) -> Result<String, eyre::Report> {
    if force
        || inquire::Confirm::new(&format!(
            "Are you sure you want to delete this Profile - '{}'? This cannot be undone",
            profile_name
        ))
        .with_default(false)
        .prompt()?
    {
        // Check if "Default Profile" and remove link if so
        menu_app_settings::remove_profile_from_app_settings(profile_id)?;

        // Remove from any queues...
        db::delete_profile_from_queues(profile_id)?;

        // Now delete the profile
        db::delete_profile(profile_id)
            .wrap_err(format!("Failed to delete Profile - '{}", profile_name))?;
        return Ok(format!("Successfully deleted Profile '{}'", profile_name));
    }

    Ok("Canceled Profile deletion".to_string())
}

pub fn delete_profile() -> Result<String, eyre::Report> {
    let profile_list = db::get_profile_display_list(data::ProfileOrder::Name)?;
    if profile_list.is_empty() {
        return Ok("There are no Profiles to delete".to_string());
    }

    let profile_selection = inquire::Select::new("Pick the Profile to Delete:", profile_list)
        .with_page_size(tui::MENU_PAGE_SIZE)
        .with_formatter(&|i| i.value.simple_display())
        .prompt_skippable()?;

    if let Some(profile) = profile_selection {
        delete_profile_core(profile.id, &profile.name, false)
    } else {
        Ok("No changes made to deleting Profile".to_string())
    }
}

pub fn cli_delete_profile(profile_name: &str, force: bool) -> Result<String, eyre::Report> {
    let profile_result = db::get_profile_by_name(profile_name);
    if let Ok(profile) = profile_result {
        delete_profile_core(profile.id, &profile.name, force)
    } else {
        Ok(format!(
            "Cannot delete Profile. Profile not found - '{}'",
            profile_name
        ))
    }
}

pub fn set_default_profile() -> Result<String, eyre::Report> {
    let profile_list = db::get_profile_display_list(data::ProfileOrder::Name)?;
    if profile_list.is_empty() {
        return Ok(
            "Cannot set Default Profile. There are no Profiles found. Please create one"
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
        .with_formatter(&|i| i.value.simple_display())
        .prompt_skippable()?;

    match profile {
        Some(profile) => {
            app_settings.default_profile_id = Some(profile.id);
            db::save_app_settings(app_settings).wrap_err("Failed to set Default Profile")?;
            Ok(format!(
                "Marked Profile '{}' as Default",
                profile.simple_display()
            ))
        }
        None => Ok("No changes made to setting Profile as Default".to_string()),
    }
}

pub fn list_profiles(list_type: data::ListType) -> Result<String, eyre::Report> {
    let profiles = db::get_profile_display_list(data::ProfileOrder::Name)
        .wrap_err("Unable to profile listing".to_string())?;

    if profiles.is_empty() {
        return Ok("There are no Profiles to list".to_string());
    }

    let table = match list_type {
        data::ListType::Full => tabled::Table::new(profiles)
            .with(Modify::new(Rows::new(1..)).with(Width::wrap(30)))
            .with(Style::modern())
            // .with(Rotate::Left)
            // .with(Rotate::Top)
            .to_string(),
        data::ListType::Summary => {
            let mut builder = Builder::default();
            builder.push_record([
                "Name",
                "Engine",
                "IWAD File",
                "Map Files",
                "Save Game",
                "Additional Args",
                //"Run Count",
                //"Date Last Run",
            ]);
            for profile in profiles {
                builder.push_record([
                    profile.name,
                    profile.engine_app_name,
                    //format!("{} ({})", profile.engine_app_name, profile.engine_version),
                    profile.iwad_file,
                    data::display_combined_tabled_map_strings(&profile.map_files),
                    profile.save_game,
                    profile.additional_arguments,
                    //profile.run_count.to_string(),
                    //data::display_option_utc_datetime_to_local(&profile.date_last_run),
                ]);
            }
            let mut table = builder.build();
            table
                .with(Modify::new(Rows::new(1..)).with(Width::wrap(25)))
                .with(Style::modern())
                .to_string()
        }
    };
    Ok(table)
}
