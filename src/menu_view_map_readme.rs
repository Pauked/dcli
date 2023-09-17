use colored::Colorize;
use eyre::Context;
use log::info;

use crate::{tui, db, runner};

pub async fn view_map_readme_menu() -> Result<String, eyre::Report> {
        clearscreen::clear().unwrap();
        loop {
            let menu_command = tui::view_map_readme_menu_prompt();
            if let tui::ViewMapReadmeCommand::Back = menu_command {
                return Ok("".to_string());
            }
            let result = run_view_map_readme_menu_option(menu_command).await?;
            clearscreen::clear().unwrap();
            info!("{}", result)
        }
    }

pub async fn run_view_map_readme_menu_option(
    menu_command: tui::ViewMapReadmeCommand,
) -> Result<String, eyre::Report> {
    match menu_command {
        tui::ViewMapReadmeCommand::ViewFromActiveProfile => view_map_readme_from_active_profile().await,
        tui::ViewMapReadmeCommand::ViewFromLastProfile => view_map_readme_from_last_profile().await,
        tui::ViewMapReadmeCommand::ViewFromPickProfile => view_map_readme_from_pick_profile().await,
        tui::ViewMapReadmeCommand::ViewFromPickPwad => view_map_readme_from_pick_pwad().await,
        tui::ViewMapReadmeCommand::Back => Ok("".to_string()),
    }
}

async fn view_map_readme_from_pwad_id(pwad_id: i32) -> Result<String, eyre::Report> {
    let pwad = db::get_pwad_by_id(pwad_id)
        .await
        .wrap_err("Unable to get PWAD".to_string())?;

    runner::open_map_read(&pwad.path)
}

async fn view_map_readme_from_active_profile() -> Result<String, eyre::Report> {
    let app_settings = db::get_app_settings().await?;

    if app_settings.active_profile_id.is_none() {
        return Ok("No Active Profile found. Cannot View Map Readme.".yellow().to_string());
    };

    let profile = db::get_profile_by_id(app_settings.active_profile_id.unwrap())
        .await
        .wrap_err("Unable to get Profile".to_string())?;

    match profile.pwad_id {
        Some(pwad_id) => view_map_readme_from_pwad_id(pwad_id).await,
        None => Ok("No PWAD selected for this Profile.".yellow().to_string()),
    }
}

async fn view_map_readme_from_last_profile() -> Result<String, eyre::Report> {
    let app_settings = db::get_app_settings().await?;

    if app_settings.last_profile_id.is_none() {
        return Ok("No Last Profile found. Cannot View Map Readme.".yellow().to_string());
    };

    let profile = db::get_profile_by_id(app_settings.last_profile_id.unwrap())
        .await
        .wrap_err("Unable to get Profile".to_string())?;

    match profile.pwad_id {
        Some(pwad_id) => view_map_readme_from_pwad_id(pwad_id).await,
        None => Ok("No PWAD selected for this Profile.".yellow().to_string()),
    }
}

pub async fn view_map_readme_from_pick_profile() -> Result<String, eyre::Report> {
    let profile_list = db::get_profile_display_list().await?;
    if profile_list.is_empty() {
        return Ok("There are no profiles to select from.".red().to_string());
    }

    let profile = inquire::Select::new("Pick the Profile to view Map Readme from:", profile_list)
        .prompt_skippable()?;

    match profile {
        Some(profile) => {
            if profile.pwad_id > 0 {
                view_map_readme_from_pwad_id(profile.pwad_id).await
            } else {
                Ok("No PWAD selected for this Profile.".yellow().to_string())
            }
        }
        None => Ok("Cancelled viewing Map Readme.".yellow().to_string()),
    }
}

async fn view_map_readme_from_pick_pwad() -> Result<String, eyre::Report> {
    let pwad_list = db::get_pwads().await?;
    if pwad_list.is_empty() {
        return Ok("There are no PWADs to select from.".red().to_string());
    }

    let pwad = inquire::Select::new("Pick the PWAD to view Map Readme from:", pwad_list)
        .prompt_skippable()?;

    match pwad {
        Some(pwad) => view_map_readme_from_pwad_id(pwad.id).await,
        None => Ok("Cancelled viewing Map Readme.".yellow().to_string()),
    }
}
