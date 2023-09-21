use std::{
    env,
    ffi::OsStr,
    process::{Command, Stdio},
};

use colored::Colorize;
use eyre::Context;

use crate::{constants, data, db, files, paths};

pub fn play_from_profile(
    profile_id: i32,
    update_last_profile: bool,
) -> Result<String, eyre::Report> {
    // Get profile and run it
    let profile = db::get_profile_by_id(profile_id)?;
    let play_result = play_from_engine_iwad_and_pwad(
        profile.engine_id.unwrap(),
        profile.iwad_id.unwrap(),
        data::pwad_ids_from_options(
            profile.pwad_id,
            profile.pwad_id2,
            profile.pwad_id3,
            profile.pwad_id4,
            profile.pwad_id5,
        ),
        profile.additional_arguments,
    )?;

    // Update the profile's last run date
    db::update_profile_date_last_run(profile_id)?;

    // Update last run profile on the app settings
    if update_last_profile {
        let mut app_settings = db::get_app_settings()?;
        app_settings.last_profile_id = Some(profile_id);
        db::save_app_settings(app_settings)?;
    }

    Ok(play_result)
}

pub fn play_from_engine_iwad_and_pwad(
    engine_id: i32,
    iwad_id: i32,
    pwad_ids: data::PwadIds,
    additional_arguments: Option<String>,
) -> Result<String, eyre::Report> {
    let engine = db::get_engine_by_id(engine_id)?;
    let iwad = db::get_iwad_by_id(iwad_id)?;

    // Pre-run checks, do the files we want to use exist?
    if !paths::file_exists(&engine.path) {
        return Err(eyre::eyre!(
            "Play aborted, Engine not found - '{}'",
            engine.path
        ));
    }
    if !paths::file_exists(&iwad.path) {
        return Err(eyre::eyre!(
            "Play aborted, IWAD not found - '{}'",
            iwad.path
        ));
    }

    // TODO: Refactor to be based off selected Doom Engine config (each engine may have different arguments for the same thing)

    // Build up Command based on Profile settings
    let mut cmd = Command::new(&engine.path);
    cmd.arg("-iwad").arg(iwad.path);

    // Multiple PWADs may be selected, so we need to add them all
    if pwad_ids.0 != 0 {
        cmd.arg("-file");
        let pwad_ids_array = [pwad_ids.0, pwad_ids.1, pwad_ids.2, pwad_ids.3, pwad_ids.4];
        for &id in &pwad_ids_array {
            if id != 0 {
                let pwad = db::get_pwad_by_id(id)?;

                if !paths::file_exists(&pwad.path) {
                    return Err(eyre::eyre!(
                        "Play aborted, EPWAD not found - '{}'",
                        pwad.path
                    ));
                }

                cmd.arg(&pwad.path);
            }
        }
    }

    // Add in additional arguments
    add_arguments_to_command(&mut cmd, additional_arguments);

    // Add in shared play settings
    let play_settings = db::get_play_settings()?;
    if play_settings.comp_level.is_some() {
        cmd.arg("-complevel")
            .arg(play_settings.comp_level.unwrap().to_string());
    }
    if play_settings.config_file.is_some() {
        cmd.arg("-config").arg(play_settings.config_file.unwrap());
    }
    if play_settings.fast_monsters {
        cmd.arg("-fast");
    }
    if play_settings.no_monsters {
        cmd.arg("-nomonsters");
    }
    if play_settings.respawn_monsters {
        cmd.arg("-respawn");
    }
    if play_settings.warp.is_some() {
        cmd.arg("-warp").arg(play_settings.warp.unwrap());
    }
    if play_settings.skill.is_some() {
        cmd.arg("-skill")
            .arg(play_settings.skill.unwrap().to_string());
    }
    if play_settings.turbo.is_some() {
        cmd.arg("-turbo")
            .arg(play_settings.turbo.unwrap().to_string());
    }
    if play_settings.timer.is_some() {
        cmd.arg("-timer")
            .arg(play_settings.timer.unwrap().to_string());
    }
    if play_settings.width.is_some() {
        cmd.arg("-width")
            .arg(play_settings.width.unwrap().to_string());
    }
    if play_settings.height.is_some() {
        cmd.arg("-height")
            .arg(play_settings.height.unwrap().to_string());
    }
    if play_settings.full_screen {
        cmd.arg("-fullscreen");
    }
    if play_settings.windowed {
        cmd.arg("-window");
    }
    add_arguments_to_command(&mut cmd, play_settings.additional_arguments);

    let display_args = get_display_args(&cmd);
    let run_message = format!(
        "Engine '{}', Args '{}'",
        engine.path.magenta(),
        display_args.blue()
    );

    // Let's go!
    cmd.stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .wrap_err(format!("Failed to run {}", run_message))?;

    // inquire::Text::new("Press any key to continue...").prompt_skippable()?;
    Ok(format!("Successfully opened {}", run_message))
}

fn add_arguments_to_command(cmd: &mut Command, additional_arguments: Option<String>) {
    if let Some(additional_arguments_unwrapped) = additional_arguments {
        let escaped_arguments = additional_arguments_unwrapped.replace('\\', r"\\");
        let args: Vec<String> = shlex::split(&escaped_arguments).unwrap_or_default();
        for arg in args {
            cmd.arg(arg);
        }
    }
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

pub fn open_map_readme(pwad_path: &str) -> Result<String, eyre::Report> {
    let readme_file_name = files::get_map_readme_file_name(pwad_path)
        .wrap_err(format!("Unable to get Map Readme for PWAD '{}'", pwad_path).to_string())?;

    match readme_file_name {
        Some(readme_file_name) => {
            let program = {
                if env::consts::OS == constants::OS_MACOS {
                    "open"
                } else if env::consts::OS == constants::OS_WINDOWS {
                    "cmd"
                } else {
                    return Err(eyre::eyre!(format!(
                        "open_map_readme is only supported on Windows and MacOS, not on '{}'",
                        env::consts::OS
                    )));
                }
            };

            let mut cmd = Command::new(program);

            #[cfg(target_os = "macos")]
            cmd.arg(&readme_file_name);
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
                readme_file_name.magenta(),
                pwad_path.blue()
            ))
        }
        None => Ok(format!("No Map Readme found for PWAD - '{}'", pwad_path)
            .yellow()
            .to_string()),
    }
}

pub fn map_editor(pwad_path: &str, map_editor: data::MapEditor) -> Result<String, eyre::Report> {
    let mut cmd = Command::new(&map_editor.path);
    if map_editor.load_file_argument.is_some() {
        cmd.arg(map_editor.load_file_argument.unwrap());
    }

    cmd.arg(pwad_path);

    if map_editor.additional_arguments.is_some() {
        let args: Vec<String> =
            shlex::split(&map_editor.additional_arguments.unwrap()).unwrap_or_default();
        for arg in args {
            cmd.arg(arg);
        }
    }

    let display_args = get_display_args(&cmd);
    let run_message = format!(
        "Map Editor '{}', Args '{}'",
        &map_editor.path.magenta(),
        display_args.blue()
    );

    cmd.stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .wrap_err(format!("Failed to run {}", run_message))?;

    Ok(format!("Successfully opened {}", run_message))
}
