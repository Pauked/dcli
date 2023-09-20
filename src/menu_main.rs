use color_eyre::{
    eyre::{self},
    Result,
};
use colored::Colorize;
use uuid::Uuid;

use crate::{data, db, paths, runner, tui};

pub fn get_active_profile_text() -> Result<String, eyre::Report> {
    if !db::database_exists() {
        return Ok("No database found. Please run 'init'.".red().to_string());
    }

    if db::is_empty_app_settings_table()? {
        return Ok("No settings configured. Please run 'init'."
            .red()
            .to_string());
    }

    let app_settings = db::get_app_settings()?;

    if app_settings.active_profile_id.is_none() {
        return Ok(format!("Active - {}", "Please set one".yellow()));
    }

    let profile_display = db::get_profile_display_by_id(app_settings.active_profile_id.unwrap())?;
    Ok(format!(
        "Active - {}",
        profile_display.to_string().green().bold()
    ))
}

pub fn get_last_profile_text() -> Result<String, eyre::Report> {
    if db::is_empty_app_settings_table()? {
        return Ok("No settings configured. Please run 'init'."
            .red()
            .to_string());
    }

    let app_settings = db::get_app_settings()?;
    if app_settings.last_profile_id.is_none() {
        return Ok(format!("Last   - {}", "Run a profile to set as last run.".yellow()));
    }

    let profile_display = db::get_profile_display_by_id(app_settings.last_profile_id.unwrap())?;
    Ok(format!(
        "Last   - {}",
        profile_display.to_string().purple().bold()
    ))
}

pub fn play_active_profile() -> Result<String, eyre::Report> {
    let app_settings = db::get_app_settings()?;

    if app_settings.active_profile_id.is_none() {
        return Ok("No active profile found. Please set one.".red().to_string());
    };

    runner::play_from_profile(app_settings.active_profile_id.unwrap(), false)
}

pub fn play_last_profile() -> Result<String, eyre::Report> {
    let app_settings = db::get_app_settings()?;

    if app_settings.active_profile_id.is_none() {
        return Ok(
            "No last run profile found. Run a profile to make it the last run."
                .red()
                .to_string(),
        );
    };

    runner::play_from_profile(app_settings.last_profile_id.unwrap(), true)
}

pub fn pick_and_play_profile() -> Result<String, eyre::Report> {
    let profile_list = db::get_profile_display_list()?;
    if profile_list.is_empty() {
        return Ok(
            "Cannot set active profile, there are no profiles found. Please create one."
                .red()
                .to_string(),
        );
    }
    let profile = inquire::Select::new("Pick the Profile you want to Play:", profile_list)
        .prompt_skippable()?;

    match profile {
        Some(profile) => runner::play_from_profile(profile.id, true),
        None => Ok("No profile selected.".yellow().to_string()),
    }
}

pub fn pick_and_play_pwad() -> Result<String, eyre::Report> {
    let engines = db::get_engines()?;
    if engines.is_empty() {
        return Ok("There are no Engines to select. Please run 'init'."
            .red()
            .to_string());
    }
    let iwads = db::get_iwads()?;
    if iwads.is_empty() {
        return Ok("There are no IWADs to select. Please run 'init."
            .red()
            .to_string());
    }
    let pwads = db::get_pwads()?;
    if pwads.is_empty() {
        return Ok("There are no PWADs to select. Please run 'init'."
            .red()
            .to_string());
    }

    let engine_selection = inquire::Select::new("Pick the Engine you want to use:", engines)
        .with_page_size(tui::MENU_PAGE_SIZE)
        .prompt()?;

    let iwad_selection = inquire::Select::new("Pick the IWAD you want to use:", iwads)
        .with_page_size(tui::MENU_PAGE_SIZE)
        .prompt()?;

    let pwad_selection =
        inquire::Select::new("Pick the PWAD you want to use (optional):", pwads.clone())
            .with_page_size(tui::MENU_PAGE_SIZE)
            .prompt_skippable()?;
    let pwad_id = pwad_selection.as_ref().map(|pwad| pwad.id);

    // TODO: Enter Additional Arguments
    let additional_arguments =
        inquire::Text::new("Enter any additional arguments (optional):").prompt_skippable()?;

    // TODO: Offer to create profile?
    if inquire::Confirm::new("Autosave this options as a Profile?")
        .with_default(false)
        .prompt()
        .unwrap()
    {
        let wad_name = match pwad_selection {
            None => paths::extract_file_name(&iwad_selection.path),
            Some(pwad) => pwad.title,
        };
        let profile_name = format!("Autosave - '{}' {}", wad_name, Uuid::new_v4());

        // let profile_name = inquire::Text::new("Enter a name for your Profile:")
        //     .with_validator(inquire::min_length!(5))
        //     .prompt()?;

        let profile = data::Profile {
            id: 0,
            name: profile_name,
            engine_id: Some(engine_selection.id),
            iwad_id: Some(iwad_selection.id),
            pwad_id,
            pwad_id2: None,
            pwad_id3: None,
            pwad_id4: None,
            pwad_id5: None,
            additional_arguments,
        };
        let add_result = db::add_profile(profile)?;
        let new_profile_id: i32 = add_result.last_insert_rowid().try_into().unwrap();

        runner::play_from_profile(new_profile_id, true)
    } else {
        runner::play_from_engine_iwad_and_pwad(
            engine_selection.id,
            iwad_selection.id,
            pwad_id,
            additional_arguments,
        )
    }
}
