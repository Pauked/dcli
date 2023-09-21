use chrono::Utc;
use color_eyre::{
    eyre::{self},
    owo_colors::OwoColorize,
    Result,
};
use colored::Colorize;
use eyre::Context;
use log::info;
use uuid::Uuid;

use crate::{data, db, paths, runner, tui};

pub fn get_default_profile_text() -> Result<String, eyre::Report> {
    if !db::database_exists() {
        return Ok("No database found. Please run 'init'".red().to_string());
    }

    if db::is_empty_app_settings_table()? {
        return Ok("No settings configured. Please run 'init'"
            .red()
            .to_string());
    }

    let app_settings = db::get_app_settings()?;

    if app_settings.default_profile_id.is_none() {
        return Ok(format!("Default - {}", "Please set one".yellow()));
    }

    let profile_display = db::get_profile_display_by_id(app_settings.default_profile_id.unwrap())?;
    Ok(format!(
        "Default - {}",
        profile_display.to_string().green().bold()
    ))
}

pub fn get_last_profile_text() -> Result<String, eyre::Report> {
    if db::is_empty_app_settings_table()? {
        return Ok("No settings configured. Please run 'init'"
            .red()
            .to_string());
    }

    let app_settings = db::get_app_settings()?;
    if app_settings.last_profile_id.is_none() {
        return Ok(format!(
            "Last    - {}",
            "Run a profile to set as last run".yellow()
        ));
    }

    let profile_display = db::get_profile_display_by_id(app_settings.last_profile_id.unwrap())?;
    Ok(format!(
        "Last    - {}",
        profile_display.to_string().purple().bold()
    ))
}

pub fn play_default_profile() -> Result<String, eyre::Report> {
    let app_settings = db::get_app_settings()?;

    if app_settings.default_profile_id.is_none() {
        return Ok("No Default Profile found. Please set one."
            .red()
            .to_string());
    };

    runner::play_from_profile(app_settings.default_profile_id.unwrap(), false)
}

pub fn play_last_profile() -> Result<String, eyre::Report> {
    let app_settings = db::get_app_settings()?;

    if app_settings.last_profile_id.is_none() {
        return Ok(
            "No Last Run Profile found. Run a profile to make it the last run."
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
            "Cannot Play Profile, there are no profiles found. Please create one."
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

    let engine_selection = {
        if let Some(engine_id) = app_settings.default_engine_id {
            let engine =
                db::get_engine_by_id(engine_id).wrap_err("Unable to get Default Engine")?;
            info!("Using Default Engine: {}", engine.blue());
            engine
        } else {
            inquire::Select::new("Pick the Engine you want to use:", engines)
                .with_page_size(tui::MENU_PAGE_SIZE)
                .prompt()?
        }
    };

    let iwad_selection = {
        if let Some(iwad_id) = app_settings.default_iwad_id {
            let iwad = db::get_iwad_by_id(iwad_id).wrap_err("Unable to get Default IWAD")?;
            info!("Using Default IWAD: {}", iwad.blue());
            iwad
        } else {
            inquire::Select::new("Pick the IWAD you want to use:", iwads)
                .with_page_size(tui::MENU_PAGE_SIZE)
                .prompt()?
        }
    };

    // Yes this is ONE PWAD only. Profiles can have up to 5 PWADs, but this is just a quick play option.
    let pwad_selection =
        inquire::Select::new("Pick the PWAD you want to use (optional):", pwads.clone())
            .with_page_size(tui::MENU_PAGE_SIZE)
            .prompt_skippable()?;
    let pwad_id = pwad_selection.as_ref().map(|pwad| pwad.id);

    let additional_arguments =
        inquire::Text::new("Enter any additional arguments (optional):").prompt_skippable()?;

    if inquire::Confirm::new("Autosave this options as a Profile?")
        .with_default(false)
        .prompt()
        .unwrap()
    {
        let wad_name = match pwad_selection {
            None => paths::extract_file_name(&iwad_selection.path),
            Some(pwad) => pwad.title,
        };
        let profile_name = format!(
            "Autosave-{} '{}'",
            &Uuid::new_v4().to_string()[..8],
            wad_name
        );

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
            date_created: Utc::now(),
            date_edited: Utc::now(),
            date_last_run: None,
        };
        let add_result = db::add_profile(profile)?;
        let new_profile_id: i32 = add_result.last_insert_rowid().try_into().unwrap();

        runner::play_from_profile(new_profile_id, true)
    } else {
        runner::play_from_engine_iwad_and_pwad(
            engine_selection.id,
            iwad_selection.id,
            data::pwad_ids_from_options(pwad_id, None, None, None, None),
            additional_arguments,
        )
    }
}
