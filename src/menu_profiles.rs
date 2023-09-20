use colored::Colorize;
use eyre::Context;
use tabled::settings::{object::Rows, Modify, Style, Width};

use crate::{data, db, menu_common, tui};

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
    let pwads = db::get_pwads()?;
    if pwads.is_empty() {
        return Ok("There are no PWADs to select. Please run 'init'"
            .red()
            .to_string());
    }

    let app_settings = db::get_app_settings()?;
    let engine_starting_cursor = match app_settings.default_engine_id {
        Some(ref s) => engines.iter().position(|x| x.id == *s).unwrap(),
        None => 0,
    };
    let iwad_starting_cursor = match app_settings.default_iwad_id {
        Some(ref s) => iwads.iter().position(|x| x.id == *s).unwrap(),
        None => 0,
    };

    // TODO: Validate if profile_name already exists
    let profile_name = inquire::Text::new("Enter a name for your Profile:")
        .with_validator(inquire::min_length!(5))
        .prompt()?;

    let engine_selection = inquire::Select::new("Pick the Engine you want to use:", engines)
        .with_starting_cursor(engine_starting_cursor)
        .with_page_size(tui::MENU_PAGE_SIZE)
        .prompt()?;

    let iwad_selection = inquire::Select::new("Pick the IWAD you want to use:", iwads)
        .with_starting_cursor(iwad_starting_cursor)
        .with_page_size(tui::MENU_PAGE_SIZE)
        .prompt()?;

    let pwad_selection = menu_common::get_pwad_selection(pwads, vec![])?;
    let pwad_id = Some(pwad_selection[0].id).filter(|&id| id > 0);
    let pwad_id2 = Some(pwad_selection[1].id).filter(|&id| id > 0);
    let pwad_id3 = Some(pwad_selection[2].id).filter(|&id| id > 0);
    let pwad_id4 = Some(pwad_selection[3].id).filter(|&id| id > 0);
    let pwad_id5 = Some(pwad_selection[4].id).filter(|&id| id > 0);

    let additional_arguments =
        inquire::Text::new("Enter any additional arguments (optional):").prompt_skippable()?;

    let profile = data::Profile {
        id: 0,
        name: profile_name,
        engine_id: Some(engine_selection.id),
        iwad_id: Some(iwad_selection.id),
        pwad_id,
        pwad_id2,
        pwad_id3,
        pwad_id4,
        pwad_id5,
        additional_arguments,
    };
    let add_result = db::add_profile(profile)?;
    let new_profile_id: i32 = add_result.last_insert_rowid().try_into().unwrap();
    set_profile_as_default(new_profile_id)?;

    Ok("Successfully created a new Profile".to_string())
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
    let pwads = db::get_pwads()?;
    if pwads.is_empty() {
        return Ok("There are no PWADs to select. Please run 'init'"
            .red()
            .to_string());
    }

    let profile = inquire::Select::new("Pick the Profile to Edit:", profile_list)
        .with_page_size(tui::MENU_PAGE_SIZE)
        .prompt()?;

    let engine_starting_cursor = engines
        .iter()
        .position(|engine| profile.engine_id == engine.id)
        .unwrap_or(0);

    let iwad_starting_cursor = iwads
        .iter()
        .position(|iwad| profile.iwad_id == iwad.id)
        .unwrap_or(0);

    let pwad_starting_cursor = pwads
        .iter()
        .position(|pwad| profile.pwad_ids.0 == pwad.id)
        .unwrap_or(0);
    let pwad_starting_cursor2 = pwads
        .iter()
        .position(|pwad| profile.pwad_ids.1 == pwad.id)
        .unwrap_or(0);
    let pwad_starting_cursor3 = pwads
        .iter()
        .position(|pwad| profile.pwad_ids.2 == pwad.id)
        .unwrap_or(0);
    let pwad_starting_cursor4 = pwads
        .iter()
        .position(|pwad| profile.pwad_ids.3 == pwad.id)
        .unwrap_or(0);
    let pwad_starting_cursor5 = pwads
        .iter()
        .position(|pwad| profile.pwad_ids.4 == pwad.id)
        .unwrap_or(0);
    let default_pwads = vec![
        pwad_starting_cursor,
        pwad_starting_cursor2,
        pwad_starting_cursor3,
        pwad_starting_cursor4,
        pwad_starting_cursor5,
    ];

    // TODO: Validate if profile_name already exists
    let profile_name = inquire::Text::new("Enter a name for your Profile:")
        .with_validator(inquire::min_length!(5))
        .with_default(&profile.name)
        .prompt()?;

    let engine_selection = inquire::Select::new("Pick the Engine you want to use:", engines)
        .with_starting_cursor(engine_starting_cursor)
        .with_page_size(tui::MENU_PAGE_SIZE)
        .prompt()?;

    let iwad_selection = inquire::Select::new("Pick the IWAD you want to use:", iwads)
        .with_starting_cursor(iwad_starting_cursor)
        .with_page_size(tui::MENU_PAGE_SIZE)
        .prompt()?;

    let pwad_selection = menu_common::get_pwad_selection(pwads, default_pwads)?;
    let pwad_id = Some(pwad_selection[0].id).filter(|&id| id > 0);
    let pwad_id2 = Some(pwad_selection[1].id).filter(|&id| id > 0);
    let pwad_id3 = Some(pwad_selection[2].id).filter(|&id| id > 0);
    let pwad_id4 = Some(pwad_selection[3].id).filter(|&id| id > 0);
    let pwad_id5 = Some(pwad_selection[4].id).filter(|&id| id > 0);

    let additional_arguments = inquire::Text::new("Enter any additional arguments (optional):")
        .with_default(&profile.additional_arguments)
        .prompt_skippable()?;

    let profile = data::Profile {
        id: profile.id,
        name: profile_name.clone(),
        engine_id: Some(engine_selection.id),
        iwad_id: Some(iwad_selection.id),
        pwad_id,
        pwad_id2,
        pwad_id3,
        pwad_id4,
        pwad_id5,
        additional_arguments,
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

pub fn list_profiles() -> Result<String, eyre::Report> {
    let profiles =
        db::get_profile_display_list().wrap_err("Unable to profile listing".to_string())?;

    let table = tabled::Table::new(profiles)
        .with(Modify::new(Rows::new(1..)).with(Width::wrap(30).keep_words()))
        .with(Style::modern())
        .to_string();
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
    };
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
