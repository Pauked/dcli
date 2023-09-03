use std::{path::PathBuf, process::Command};

use color_eyre::{
    eyre::{self, Context},
    Report, Result,
};
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm};

use crate::{constants, db, init, settings};

pub async fn run_option(command: constants::Command) -> Result<String, eyre::Report> {
    let config_file_path = settings::get_config_filename(constants::CONFIG_FILE);
    let settings = settings::get(config_file_path.clone());

    match command {
        constants::Command::Reset => {}
        _ => {
            db::create_db().await?;
        }
    }

    match command {
        constants::Command::Play => play(settings),
        constants::Command::Config => config(config_file_path),
        constants::Command::Editor => editor(settings),
        constants::Command::Init => init::init().await,
        constants::Command::Reset => reset(false).await,
        _ => Ok("".to_string()),
    }
}

pub fn config(config_file_path: PathBuf) -> Result<String, eyre::Report> {
    // Open the App.toml file in notepad
    Command::new("notepad.exe")
        .arg(config_file_path.clone())
        .spawn()
        .wrap_err(format!(
            "Failed to open file in Notepad! - '{}'",
            config_file_path.display()
        ))?;

    Ok(format!(
        "Opened the following file in Notepad! - '{}'",
        config_file_path.display()
    ))
}

pub fn editor(settings: settings::Settings) -> Result<String, eyre::Report> {
    // Open Editor
    Command::new(&settings.editor_exe)
        .arg(&settings.file)
        //.arg(format!("'{}'", &file))
        .spawn()
        .wrap_err(format!(
            "Failed to open file in Editor! - '{}' / '{}'",
            settings.editor_exe, settings.file
        ))?;

    Ok(format!(
        "Opened the following file in Editor! - '{}' / '{}'",
        settings.editor_exe, settings.file
    ))
}

fn play(settings: settings::Settings) -> Result<String, eyre::Report> {
    let mut cmd = Command::new(&settings.doom_exe);
    cmd.arg("-iwad")
        .arg(settings.iwad)
        .arg("-file")
        .arg(&settings.file);
    if let Some(save_game) = settings.save_game {
        cmd.arg("-loadgame").arg(save_game);
    }

    // cmd.status().wrap_err(format!("Failed to run Doom! - '{}'", settings.doom_exe))?;
    cmd.spawn()
        .wrap_err(format!("Failed to run Doom! - '{}'", settings.doom_exe))?;
    Ok(format!(
        "Opened the following file in Doom! - '{}' / '{}''",
        settings.doom_exe, settings.file
    ))
}

async fn reset(force: bool) -> Result<String, Report> {
    if !db::database_exists().await {
        return Ok("Database does not exist, nothing to reset.".to_string());
    }

    // Prompt the user for confirmation to delete the file
    if force
        || Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to reset the database? All data will be deleted.")
            .interact()
            .unwrap()
    {
        db::reset_db().wrap_err("Failed to reset database.")?;
        Ok("Successfully reset database.".green().to_string())
    } else {
        Ok("Database reset not confirmed.".to_string())
    }
}
