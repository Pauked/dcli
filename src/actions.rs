use std::process::{Command, Stdio};

use color_eyre::{
    eyre::{self, Context},
    Report, Result,
};
use colored::Colorize;
use log::info;
use tabled::settings::{object::Rows, Modify, Style, Width};

use crate::{
    constants::{self},
    db, init, profiles, tui,
};

pub async fn run_main_menu_option(command: constants::MainCommand) -> Result<String, eyre::Report> {
    db::create_db().await?;

    match command {
        constants::MainCommand::Play => play().await,
        constants::MainCommand::Profiles => profiles::profiles_menu().await,
        constants::MainCommand::Config => config_menu().await,
        constants::MainCommand::Quit => Ok("Quitting".to_string()),
        constants::MainCommand::Unknown => Ok("Unknown command".to_string()),
    }
}

pub async fn get_active_profile_text() -> Result<String, eyre::Report> {
    if !db::database_exists().await {
        return Ok("No database found, please run 'init'.".red().to_string());
    }
    let settings = db::get_settings().await?;

    if settings.active_profile_id.is_none() {
        return Ok("No active profile found, please set one.".red().to_string());
    }

    let profile_display = db::get_profile_display_by_id(settings.active_profile_id.unwrap()).await?;
    Ok(format!(
        "Active profile: {}",
        profile_display.to_string().bright_green().bold()
    ))
}

pub async fn play() -> Result<String, eyre::Report> {
    // Do we have an active profile?
    // No, pick one.
    // Do we have any profiles configured?
    // No, create one.

    let settings = db::get_settings().await?;

    if settings.active_profile_id.is_none() {
        return Ok("No active profile found, please set one.".red().to_string());
        // FIXME: Call the "set active profile" function
    };

    let single_profile = db::get_profile_by_id(settings.active_profile_id.unwrap()).await?;
    let engine = db::get_engine_by_id(single_profile.engine_id.unwrap()).await?;
    let iwad = db::get_iwad_by_id(single_profile.iwad_id.unwrap()).await?;
    let pwad = db::get_pwad_by_id(single_profile.pwad_id.unwrap()).await?;

    let mut cmd = Command::new(&engine.path);
    cmd.arg("-iwad").arg(iwad.path).arg("-file").arg(&pwad.path);
    // if let Some(save_game) = settings.save_game {
    //     cmd.arg("-loadgame").arg(save_game);
    // }

    // cmd.status().wrap_err(format!("Failed to run Doom! - '{}'", settings.doom_exe))?;
    cmd.stdout(Stdio::null())
        .spawn()
        .wrap_err(format!("Failed to run Doom! - '{}'", engine.path))?;
    Ok(format!(
        "Opened the following file in Doom! - '{}' / '{}''",
        engine.path, pwad.path
    ))
}

pub async fn config_menu() -> Result<String, eyre::Report> {
    // Menu:
    loop {
        let menu_command = tui::config_menu_prompt();
        if let constants::ConfigCommand::Back = menu_command {
            return Ok("Back to main menu".to_string());
        }
        run_config_menu_option(menu_command).await?;
    }
}

pub async fn run_config_menu_option(menu_command: constants::ConfigCommand) -> Result<String, eyre::Report> {
    match menu_command {
        constants::ConfigCommand::List => list_settings().await,
        constants::ConfigCommand::Init => init::init().await,
        constants::ConfigCommand::Reset => reset(false).await,
        constants::ConfigCommand::Back => Ok("Back to main menu".to_string()),
        constants::ConfigCommand::Unknown => Ok("Unknown command".to_string()),
    }
}

pub async fn list_settings() -> Result<String, eyre::Report> {
    info!("{}", display_engines().await?);
    info!("{}", display_iwads().await?);
    info!("{}", display_pwads().await?);
    info!("{}", display_settings().await?);
    //info!("{}", display_profiles().await?);
    Ok("".to_string())
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

async fn reset(force: bool) -> Result<String, Report> {
    if !db::database_exists().await {
        return Ok("Database does not exist, nothing to reset.".to_string());
    }

    // Prompt the user for confirmation to delete the file
    if force
        || inquire::Confirm::new("Do you want to reset the database? All data will be deleted.")
            .with_default(false)
            .prompt()
            .unwrap()
    {
        db::reset_db().wrap_err("Failed to reset database.")?;
        Ok("Successfully reset database.".green().to_string())
    } else {
        Ok("Database reset not confirmed.".to_string())
    }
}

pub async fn display_engines() -> Result<String, Report> {
    let engines = db::get_engines()
        .await
        .wrap_err("Unable to generate engine listing".to_string())?;

    let table = tabled::Table::new(engines)
        .with(Modify::new(Rows::new(1..)).with(Width::wrap(50).keep_words()))
        .with(Style::modern())
        .to_string();
    Ok(table)
}

pub async fn display_iwads() -> Result<String, Report> {
    let iwads = db::get_iwads()
        .await
        .wrap_err("Unable to iwad listing".to_string())?;

    let table = tabled::Table::new(iwads)
        .with(Modify::new(Rows::new(1..)).with(Width::wrap(30).keep_words()))
        .with(Style::modern())
        .to_string();
    Ok(table)
}

pub async fn display_pwads() -> Result<String, Report> {
    let pwads = db::get_pwads()
        .await
        .wrap_err("Unable to iwad listing".to_string())?;

    let table = tabled::Table::new(pwads)
        .with(Modify::new(Rows::new(1..)).with(Width::wrap(30).keep_words()))
        .with(Style::modern())
        .to_string();
    Ok(table)
}

pub async fn display_settings() -> Result<String, Report> {
    let settings = db::get_settings()
        .await
        .wrap_err("Unable to settings listing".to_string())?;

    let table = tabled::Table::new(vec![settings])
        .with(Modify::new(Rows::new(1..)).with(Width::wrap(30).keep_words()))
        .with(Style::modern())
        .to_string();
    Ok(table)
}
