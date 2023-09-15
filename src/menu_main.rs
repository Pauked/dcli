use std::process::{Command, Stdio};

use color_eyre::{
    eyre::{self, Context},
    Result,
};
use colored::Colorize;
use log::info;

use crate::{db, menu_config, menu_profiles, tui};

pub async fn main_menu() -> Result<String, eyre::Report> {
    clearscreen::clear().unwrap();
    loop {
        info!("{}", get_active_profile_text().await?);

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
        tui::MainCommand::PickAndPlayProfile => pick_and_play_profile().await,
        tui::MainCommand::Profiles => menu_profiles::profiles_menu().await,
        tui::MainCommand::Config => menu_config::config_menu().await,
        tui::MainCommand::Quit => Ok("Quitting".to_string()),
        tui::MainCommand::Unknown => Ok("Unknown command".to_string()),
    }
}

pub async fn get_active_profile_text() -> Result<String, eyre::Report> {
    if !db::database_exists().await {
        return Ok("No database found. Please run 'init'.".red().to_string());
    }

    if db::is_empty_settings_table().await? {
        return Ok("No settings configured. Please run 'init'."
            .red()
            .to_string());
    }

    let settings = db::get_settings().await?;

    if settings.active_profile_id.is_none() {
        return Ok("No active profile found. Please set one."
            .yellow()
            .to_string());
    }

    let profile_display =
        db::get_profile_display_by_id(settings.active_profile_id.unwrap()).await?;
    Ok(format!(
        "Active profile: {}",
        profile_display.to_string().bright_green().bold()
    ))
}

pub async fn play_active_profile() -> Result<String, eyre::Report> {
    // Do we have an active profile?
    // No, pick one.
    // Do we have any profiles configured?
    // No, create one.

    let settings = db::get_settings().await?;

    if settings.active_profile_id.is_none() {
        return Ok("No active profile found, please set one.".red().to_string());
        // FIXME: Call the "set active profile" function
    };

    play(settings.active_profile_id.unwrap()).await
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
    // Generate a list of profiles showing the full details
    let profile =
        inquire::Select::new("Pick the Profile you want to Play", profile_list).prompt()?;

    play(profile.id).await
}

pub async fn play(profile_id: i32) -> Result<String, eyre::Report> {
    let single_profile = db::get_profile_by_id(profile_id).await?;
    let engine = db::get_engine_by_id(single_profile.engine_id.unwrap()).await?;
    let iwad = db::get_iwad_by_id(single_profile.iwad_id.unwrap()).await?;
    let pwad = db::get_pwad_by_id(single_profile.pwad_id.unwrap()).await?;

    // TODO: Refactor to be based off selected Doom Engine config
    let mut cmd = Command::new(&engine.path);
    cmd.arg("-iwad").arg(iwad.path).arg("-file").arg(&pwad.path);

    if single_profile.additional_arguments.is_some() {
        let args: Vec<String> = shlex::split(&single_profile.additional_arguments.unwrap()).unwrap_or_default();
        for arg in args {
            cmd.arg(arg);
        }
    }
    // if let Some(save_game) = settings.save_game {
    //     cmd.arg("-loadgame").arg(save_game);
    // }

    cmd.stdout(Stdio::null())
        .spawn()
        .wrap_err(format!("Failed to run Engine - '{}'", engine.path))?;

    info!(
        "Successfully opened Profile - '{}'",
        single_profile.name.green()
    );
    inquire::Text::new("Press any key to continue...").prompt_skippable()?;
    Ok(format!("Successfully open Profile - '{}'", single_profile.name))
}

// pub fn notepad_config(config_file_path: PathBuf) -> Result<String, eyre::Report> {
//     // Open the App.toml file in notepad
//     Command::new("notepad.exe")
//         .arg(config_file_path.clone())
//         .spawn()
//         .wrap_err(format!(
//             "Failed to open file in Notepad! - '{}'",
//             config_file_path.display()
//         ))?;

//     Ok(format!(
//         "Opened the following file in Notepad! - '{}'",
//         config_file_path.display()
//     ))
// }

// pub fn editor(settings: app_settings::AppSettings) -> Result<String, eyre::Report> {
//     // Open Editor
//     Command::new(&settings.editor_exe)
//         .arg(&settings.file)
//         //.arg(format!("'{}'", &file))
//         .spawn()
//         .wrap_err(format!(
//             "Failed to open file in Editor! - '{}' / '{}'",
//             settings.editor_exe, settings.file
//         ))?;

//     Ok(format!(
//         "Opened the following file in Editor! - '{}' / '{}'",
//         settings.editor_exe, settings.file
//     ))
// }

// fn play(settings: app_settings::AppSettings) -> Result<String, eyre::Report> {
//     let mut cmd = Command::new(&settings.doom_exe);
//     cmd.arg("-iwad")
//         .arg(settings.iwad)
//         .arg("-file")
//         .arg(&settings.file);
//     if let Some(save_game) = settings.save_game {
//         cmd.arg("-loadgame").arg(save_game);
//     }

//     // cmd.status().wrap_err(format!("Failed to run Doom! - '{}'", settings.doom_exe))?;
//     cmd.spawn()
//         .wrap_err(format!("Failed to run Doom! - '{}'", settings.doom_exe))?;
//     Ok(format!(
//         "Opened the following file in Doom! - '{}' / '{}''",
//         settings.doom_exe, settings.file
//     ))
// }
