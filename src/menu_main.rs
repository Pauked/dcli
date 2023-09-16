use std::{
    ffi::OsStr,
    process::{Command, Stdio},
};

use color_eyre::{
    eyre::{self, Context},
    Result,
};
use colored::Colorize;
use log::info;

use crate::{db, menu_config, menu_game_settings, menu_map_editor, menu_profiles, tui};

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

    play(app_settings.active_profile_id.unwrap(), false).await
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

    play(app_settings.last_profile_id.unwrap(), true).await
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
    let profile = inquire::Select::new("Pick the Profile you want to Play", profile_list)
        .prompt_skippable()?;

    match profile {
        Some(profile) => play(profile.id, true).await,
        None => Ok("No profile selected.".yellow().to_string()),
    }
}

pub async fn play(profile_id: i32, update_last_profile: bool) -> Result<String, eyre::Report> {
    let single_profile = db::get_profile_by_id(profile_id).await?;
    let engine = db::get_engine_by_id(single_profile.engine_id.unwrap()).await?;
    let iwad = db::get_iwad_by_id(single_profile.iwad_id.unwrap()).await?;

    // TODO: Refactor to be based off selected Doom Engine config (each engine may have different arguments for the same thing)

    // Build up Command based on Profile settings
    let mut cmd = Command::new(&engine.path);
    cmd.arg("-iwad").arg(iwad.path);

    if single_profile.pwad_id.is_some() {
        let pwad = db::get_pwad_by_id(single_profile.pwad_id.unwrap()).await?;
        cmd.arg("-file").arg(&pwad.path);
    }

    if single_profile.additional_arguments.is_some() {
        let args: Vec<String> =
            shlex::split(&single_profile.additional_arguments.unwrap()).unwrap_or_default();
        for arg in args {
            cmd.arg(arg);
        }
    }

    // Add in shared Game settings
    let game_settings = db::get_game_settings().await?;
    if game_settings.comp_level.is_some() {
        cmd.arg("-complevel")
            .arg(game_settings.comp_level.unwrap().to_string());
    }
    if game_settings.fast_monsters {
        cmd.arg("-fast");
    }
    if game_settings.no_monsters {
        cmd.arg("-nomonsters");
    }
    if game_settings.respawn_monsters {
        cmd.arg("-respawn");
    }
    if game_settings.warp.is_some() {
        cmd.arg("-warp").arg(game_settings.warp.unwrap());
    }
    if game_settings.skill.is_some() {
        cmd.arg("-skill")
            .arg(game_settings.skill.unwrap().to_string());
    }
    if game_settings.turbo.is_some() {
        cmd.arg("-turbo")
            .arg(game_settings.turbo.unwrap().to_string());
    }
    if game_settings.timer.is_some() {
        cmd.arg("-timer")
            .arg(game_settings.timer.unwrap().to_string());
    }
    if game_settings.width.is_some() {
        cmd.arg("-width")
            .arg(game_settings.width.unwrap().to_string());
    }
    if game_settings.height.is_some() {
        cmd.arg("-height")
            .arg(game_settings.height.unwrap().to_string());
    }
    if game_settings.full_screen {
        cmd.arg("-fullscreen");
    }
    if game_settings.windowed {
        cmd.arg("-window");
    }
    if game_settings.additional_arguments.is_some() {
        let args: Vec<String> =
            shlex::split(&game_settings.additional_arguments.unwrap()).unwrap_or_default();
        for arg in args {
            cmd.arg(arg);
        }
    }

    let display_args = get_display_args(&cmd);
    let run_message = format!(
        "Profile '{}', Engine '{}', Args '{}'",
        single_profile.name.green(),
        engine.path.magenta(),
        display_args.blue()
    );

    // Let's go!
    cmd.stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .wrap_err(format!("Failed to run {}", run_message))?;

    // Update last run profile
    if update_last_profile {
        let mut app_settings = db::get_app_settings().await?;
        app_settings.last_profile_id = Some(profile_id);
        db::save_app_settings(app_settings).await?;
    }

    // Confirm all good
    info!("Successfully opened {}", run_message);
    inquire::Text::new("Press any key to continue...").prompt_skippable()?;
    Ok(format!("Successfully opened {}", run_message))
}

fn get_display_args(cmd: &Command) -> String {
    let cmd_args: Vec<&OsStr> = cmd.get_args().collect();
    let result: String = cmd_args
        .iter()
        .filter_map(|s| s.to_str()) // Convert each &OsStr to Option<&str>
        .collect::<Vec<_>>() // Collect to Vec<&str>
        .join(" ");
    result
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
