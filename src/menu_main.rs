use color_eyre::{
    eyre::{self},
    Result,
};
use colored::Colorize;
use log::info;

use crate::{db, menu_config, menu_game_settings, menu_map_editor, menu_profiles, tui, runner, menu_view_readme};

pub async fn main_menu() -> Result<String, eyre::Report> {
    clearscreen::clear().unwrap();
    loop {
        info!("{}", get_active_profile_text().await?);
        info!("{}", get_last_profile_text().await?);

        let menu_command = tui::main_menu_prompt();
        if let tui::MainCommand::Quit = menu_command {
            return Ok("Quitting...".to_string());
        }

        let result = run_main_menu_option(menu_command).await?;
        clearscreen::clear().unwrap();
        if !result.is_empty() {
            info!("{}", result)
        }
    }
}

pub async fn run_main_menu_option(command: tui::MainCommand) -> Result<String, eyre::Report> {
    db::create_db().await?;

    match command {
        tui::MainCommand::PlayActiveProfile => play_active_profile().await,
        tui::MainCommand::PlayLastProfile => play_last_profile().await,
        tui::MainCommand::PickAndPlayProfile => pick_and_play_profile().await,
        tui::MainCommand::Profiles => menu_profiles::profiles_menu().await,
        tui::MainCommand::MapEditor => menu_map_editor::map_editor_menu().await,
        tui::MainCommand::GameSettings => menu_game_settings::game_settings_menu().await,
        tui::MainCommand::ViewMapReadme => menu_view_readme::view_readme_menu().await,
        tui::MainCommand::Config => menu_config::config_menu().await,
        tui::MainCommand::Quit => Ok("Quitting".to_string()),
        tui::MainCommand::Unknown => Ok("Unknown command".to_string()),
    }
}

pub async fn get_active_profile_text() -> Result<String, eyre::Report> {
    if !db::database_exists().await {
        return Ok("No database found. Please run 'init'.".red().to_string());
    }

    if db::is_empty_app_settings_table().await? {
        return Ok("No settings configured. Please run 'init'."
            .red()
            .to_string());
    }

    let app_settings = db::get_app_settings().await?;

    if app_settings.active_profile_id.is_none() {
        return Ok("No active profile found. Please set one."
            .yellow()
            .to_string());
    }

    let profile_display =
        db::get_profile_display_by_id(app_settings.active_profile_id.unwrap()).await?;
    Ok(format!(
        "Active profile: {}",
        profile_display.to_string().green().bold()
    ))
}

pub async fn get_last_profile_text() -> Result<String, eyre::Report> {
    if db::is_empty_app_settings_table().await? {
        return Ok("No settings configured. Please run 'init'."
            .red()
            .to_string());
    }

    let app_settings = db::get_app_settings().await?;
    if app_settings.last_profile_id.is_none() {
        return Ok(
            "No last run profile found. Run a profile to make it the last run."
                .yellow()
                .to_string(),
        );
    }

    let profile_display =
        db::get_profile_display_by_id(app_settings.last_profile_id.unwrap()).await?;
    Ok(format!(
        "Last profile: {}",
        profile_display.to_string().purple().bold()
    ))
}

pub async fn play_active_profile() -> Result<String, eyre::Report> {
    let app_settings = db::get_app_settings().await?;

    if app_settings.active_profile_id.is_none() {
        return Ok("No active profile found. Please set one.".red().to_string());
    };

    runner::play(app_settings.active_profile_id.unwrap(), false).await
}

pub async fn play_last_profile() -> Result<String, eyre::Report> {
    let app_settings = db::get_app_settings().await?;

    if app_settings.active_profile_id.is_none() {
        return Ok(
            "No last run profile found. Run a profile to make it the last run."
                .red()
                .to_string(),
        );
    };

    runner::play(app_settings.last_profile_id.unwrap(), true).await
}

pub async fn pick_and_play_profile() -> Result<String, eyre::Report> {
    let profile_list = db::get_profile_display_list().await?;
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
        Some(profile) => runner::play(profile.id, true).await,
        None => Ok("No profile selected.".yellow().to_string()),
    }
}