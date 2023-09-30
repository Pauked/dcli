use std::{
    env,
    ffi::OsStr,
    path::Path,
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
    let play_result = play_from_engine_iwad_and_map(
        profile.engine_id.unwrap(),
        profile.iwad_id.unwrap(),
        data::map_ids_from_options(
            profile.map_id,
            profile.map_id2,
            profile.map_id3,
            profile.map_id4,
            profile.map_id5,
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

pub fn play_from_engine_iwad_and_map(
    engine_id: i32,
    iwad_id: i32,
    map_ids: data::MapIds,
    additional_arguments: Option<String>,
) -> Result<String, eyre::Report> {
    let engine = db::get_engine_by_id(engine_id)?;
    let iwad = db::get_iwad_by_id(iwad_id)?;

    // For MacOS, the path is split up into the Engine path and the internal path
    // need to merge them together in order to run the app
    let final_engine_path: String = {
        match engine.internal_path {
            Some(internal_path) => Path::new(&engine.path)
                .join(internal_path)
                .to_str()
                .unwrap()
                .to_string(),
            None => engine.path,
        }
    };

    // Pre-run checks, do the files we want to use exist?
    if !paths::file_exists(&final_engine_path) {
        return Err(eyre::eyre!(
            "Play aborted, Engine not found - '{}'",
            final_engine_path
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
    let mut cmd = Command::new(final_engine_path.clone());
    cmd.arg("-iwad").arg(iwad.path);

    // Multiple Maps may be selected, so we need to add them all
    if map_ids.0 != 0 {
        cmd.arg("-file");
        let map_ids_array = [map_ids.0, map_ids.1, map_ids.2, map_ids.3, map_ids.4];
        for &id in &map_ids_array {
            if id != 0 {
                let map = db::get_map_by_id(id)?;

                if !paths::file_exists(&map.path) {
                    return Err(eyre::eyre!("Play aborted, Map not found - '{}'", map.path));
                }

                cmd.arg(&map.path);
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
    if let Some(warp) = &play_settings.warp {
        cmd.arg("-warp");
        warp.split_whitespace().for_each(|arg| {
            cmd.arg(arg);
        });
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
        final_engine_path.magenta(),
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

pub fn open_map_readme(map_path: &str) -> Result<String, eyre::Report> {
    let readme_file_name = files::get_map_readme_file_name(map_path)
        .wrap_err(format!("Unable to get Map Readme for Map '{}'", map_path).to_string())?;

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
                    "Failed to open Map Readme for Map - '{}' / '{}'",
                    readme_file_name, map_path
                ))?;

            Ok(format!(
                "Opened Map Readme for Map - '{}' / '{}'",
                readme_file_name.magenta(),
                map_path.blue()
            ))
        }
        None => Ok(format!("No Map Readme found for Map - '{}'", map_path).to_string()),
    }
}

pub fn editor(map_path: &str, editor: data::Editor) -> Result<String, eyre::Report> {
    let mut cmd = Command::new(&editor.path);
    if editor.load_file_argument.is_some() {
        cmd.arg(editor.load_file_argument.unwrap());
    }

    cmd.arg(map_path);

    if editor.additional_arguments.is_some() {
        let args: Vec<String> =
            shlex::split(&editor.additional_arguments.unwrap()).unwrap_or_default();
        for arg in args {
            cmd.arg(arg);
        }
    }

    let display_args = get_display_args(&cmd);
    let run_message = format!(
        "Editor '{}', Args '{}'",
        &editor.path.magenta(),
        display_args.blue()
    );

    cmd.stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .wrap_err(format!("Failed to run {}", run_message))?;

    Ok(format!("Successfully opened {}", run_message))
}
