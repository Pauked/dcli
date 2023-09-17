use colored::Colorize;
use eyre::Context;
use log::info;
use tabled::settings::{object::Rows, Modify, Style, Width};

use crate::{data, db, tui, runner};

pub async fn profiles_menu() -> Result<String, eyre::Report> {
    clearscreen::clear().unwrap();
    loop {
        let menu_command = tui::profiles_menu_prompt();
        if let tui::ProfileCommand::Back = menu_command {
            return Ok("".to_string());
        }
        let result = run_profiles_menu_option(menu_command).await?;
        clearscreen::clear().unwrap();
        info!("{}", result)
    }
}

pub async fn run_profiles_menu_option(
    menu_command: tui::ProfileCommand,
) -> Result<String, eyre::Report> {
    match menu_command {
        tui::ProfileCommand::New => new_profile().await,
        tui::ProfileCommand::Edit => edit_profile().await,
        tui::ProfileCommand::Delete => delete_profile().await,
        tui::ProfileCommand::Active => set_active_profile().await,
        tui::ProfileCommand::List => display_profiles().await,
        tui::ProfileCommand::ViewMapReadme => view_map_readme().await,
        tui::ProfileCommand::Back => Ok("".to_string()),
    }
}

async fn new_profile() -> Result<String, eyre::Report> {
    let engines = db::get_engines().await?;
    if engines.is_empty() {
        return Ok("There are no Engines to select. Please run 'init'."
            .red()
            .to_string());
    }
    let iwads = db::get_iwads().await?;
    if iwads.is_empty() {
        return Ok("There are no IWADs to select. Please run 'init."
            .red()
            .to_string());
    }
    let pwads = db::get_pwads().await?;
    if pwads.is_empty() {
        return Ok("There are no PWADs to select. Please run 'init'."
            .red()
            .to_string());
    }

    // TODO: Validate if profile_name already exists
    let profile_name = inquire::Text::new("Enter a name for your Profile:")
        .with_validator(inquire::min_length!(5))
        .prompt()?;

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

    let pwad_id = match pwad_selection {
        None => None,
        Some(pwad) => Some(pwad.id),
    };

    let additional_arguments =
        inquire::Text::new("Enter any additional arguments (optional):").prompt_skippable()?;

    let profile = data::Profile {
        id: 0,
        name: profile_name,
        engine_id: Some(engine_selection.id),
        iwad_id: Some(iwad_selection.id),
        pwad_id,
        additional_arguments,
    };
    let add_result = db::add_profile(profile).await?;
    let new_profile_id: i32 = add_result.last_insert_rowid().try_into().unwrap();
    set_profile_as_active(new_profile_id).await?;

    Ok("Successfully created a new Profile".to_string())
}

async fn set_profile_as_active(profile_id: i32) -> Result<String, eyre::Report> {
    if inquire::Confirm::new("Would you like to set this as your Active Profile?")
        .with_default(false)
        .prompt()
        .unwrap()
    {
        let mut app_settings = db::get_app_settings().await?;
        app_settings.active_profile_id = Some(profile_id);
        db::save_app_settings(app_settings)
            .await
            .wrap_err("Failed to set Active profile")?;
        return Ok("Successfully set profile as active".to_string());
    }

    Ok("No changes made to setting profile as active".to_string())
}

async fn edit_profile() -> Result<String, eyre::Report> {
    let profile_list = db::get_profile_display_list().await?;
    if profile_list.is_empty() {
        return Ok("There are no profiles to edit.".red().to_string());
    }
    let engines = db::get_engines().await?;
    if engines.is_empty() {
        return Ok("There are no Engines to select. Please run 'init'."
            .red()
            .to_string());
    }
    let iwads = db::get_iwads().await?;
    if iwads.is_empty() {
        return Ok("There are no IWADs to select. Please run 'init."
            .red()
            .to_string());
    }
    let pwads = db::get_pwads().await?;
    if pwads.is_empty() {
        return Ok("There are no PWADs to select. Please run 'init'."
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
        .position(|pwad| profile.pwad_id == pwad.id)
        .unwrap_or(0);

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

    let pwad_selection =
        inquire::Select::new("Pick the PWAD you want to use (optional):", pwads.clone())
            .with_starting_cursor(pwad_starting_cursor)
            .with_page_size(tui::MENU_PAGE_SIZE)
            .prompt_skippable()?;

    let pwad_id = match pwad_selection {
        None => None,
        Some(pwad) => Some(pwad.id),
    };

    let additional_arguments = inquire::Text::new("Enter any additional arguments (optional):")
        .with_default(&profile.additional_arguments)
        .prompt_skippable()?;

    let profile = data::Profile {
        id: profile.id,
        name: profile_name.clone(),
        engine_id: Some(engine_selection.id),
        iwad_id: Some(iwad_selection.id),
        pwad_id,
        additional_arguments,
    };
    db::update_profile(profile).await?;

    Ok(format!("Successfully updated Profile - '{}'", profile_name))
}

async fn delete_profile() -> Result<String, eyre::Report> {
    let profile_list = db::get_profile_display_list().await?;
    if profile_list.is_empty() {
        return Ok("There are no profiles to delete.".red().to_string());
    }

    let profile =
        inquire::Select::new("Pick the Profile to Delete:", profile_list).prompt_skippable()?;

    if profile.is_some() {
        let profile = profile.unwrap();
        if inquire::Confirm::new(&format!(
            "Are you sure you want to delete this profile - '{}'? This cannot be undone.",
            profile.name
        ))
        .with_default(false)
        .prompt()
        .unwrap()
        {
            db::delete_profile(profile.id)
                .await
                .wrap_err(format!("Failed to delete profile - '{}", profile))?;
            return Ok(format!("Successfully Deleted profile '{}'", profile));
        }
    }

    Ok("Cancelled Profile deletion.".yellow().to_string())
}

async fn set_active_profile() -> Result<String, eyre::Report> {
    let profile_list = db::get_profile_display_list().await?;
    if profile_list.is_empty() {
        return Ok(
            "Cannot set active profile, there are no profiles found. Please create one."
                .red()
                .to_string(),
        );
    }

    // Try to get the current active profile
    let mut app_settings = db::get_app_settings().await?;
    let starting_cursor = match app_settings.active_profile_id {
        Some(ref s) => profile_list.iter().position(|x| x.id == *s).unwrap(),
        None => 0,
    };

    let profile = inquire::Select::new("Pick the Profile to mark as Active:", profile_list)
        .with_starting_cursor(starting_cursor)
        .prompt_skippable()?;

    match profile {
        Some(profile) => {
            app_settings.active_profile_id = Some(profile.id);
            db::save_app_settings(app_settings)
                .await
                .wrap_err("Failed to set Active profile")?;
            Ok(format!("Marked Profile '{}' as Active", profile))
        }
        None => Ok("No changes made to setting profile as active".to_string()),
    }
}

pub async fn display_profiles() -> Result<String, eyre::Report> {
    let profiles = db::get_profile_display_list()
        .await
        .wrap_err("Unable to profile listing".to_string())?;

    let table = tabled::Table::new(profiles)
        .with(Modify::new(Rows::new(1..)).with(Width::wrap(30).keep_words()))
        .with(Style::modern())
        .to_string();
    Ok(table)
}

pub async fn view_map_readme() -> Result<String, eyre::Report> {
    let profile_list = db::get_profile_display_list().await?;
    if profile_list.is_empty() {
        return Ok("There are no profiles to select from.".red().to_string());
    }

    let profile = inquire::Select::new("Pick the Profile to view Map Readme from:", profile_list)
        .prompt_skippable()?;

    match profile {
        Some(profile) => {
            if profile.pwad_id > 0 {
                let pwad = db::get_pwad_by_id(profile.pwad_id)
                    .await
                    .wrap_err("Unable to get PWAD".to_string())?;

                runner::open_map_read(&pwad.path)
            } else {
                Ok("No PWAD selected for this Profile.".yellow().to_string())
            }
        }
        None => Ok("Cancelled viewing Map Readme.".yellow().to_string()),
    }
}

