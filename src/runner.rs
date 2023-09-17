use std::{process::{Command, Stdio}, ffi::OsStr};

use colored::Colorize;
use eyre::Context;
use log::info;

use crate::{db, files};

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

pub fn open_map_read(pwad_path: &str) -> Result<String, eyre::Report> {
    let readme_file_name = files::get_map_readme_file_name(pwad_path)
        .wrap_err(format!("Unable to get Map Readme for PWAD '{}'", pwad_path).to_string())?;

    match readme_file_name {
        Some(readme_file_name) => {
            #[cfg(target_os = "macos")]
            let mut cmd = Command::new("open");
            #[cfg(target_os = "macos")]
            cmd.arg(readme_file_name);

            #[cfg(target_os = "windows")]
            let mut cmd = Command::new("cmd");
            #[cfg(target_os = "windows")]
            cmd.args(["/C", "start", "", &readme_file_name]);

            cmd.stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .wrap_err(format!(
                    "Failed to open Map Readme for PWAD - '{}' / '{}'",
                    readme_file_name, pwad_path
                ))?;

            Ok(format!(
                "Opened Map Readme for PWAD - '{}' / '{}'",
                readme_file_name, pwad_path
            ))
        }
        None => Ok(format!("No Map Readme found for PWAD - '{}'", pwad_path)
            .yellow()
            .to_string()),
    }
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
