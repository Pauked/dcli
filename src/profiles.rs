use eyre::Context;
use log::info;

use crate::{constants, data, db, tui};

pub async fn profiles_menu() -> Result<String, eyre::Report> {
    // Menu:
    loop {
        let menu_command = tui::profiles_menu_prompt();

        match menu_command {
            constants::ProfileCommand::New => {
                new_profile().await?;
            }
            constants::ProfileCommand::Edit => {
                edit_profile().await?;
            }
            constants::ProfileCommand::Delete => {
                info!("Deleted a profile")
            }
            constants::ProfileCommand::Active => {
                active_profile().await?;
            }
            constants::ProfileCommand::Back => {
                return Ok("Back to main menu".to_string());
            }
        }
    }
}

async fn new_profile() -> Result<String, eyre::Report> {
    let profile_name = inquire::Text::new("Enter a name for your profile")
        .with_validator(inquire::min_length!(5))
        .prompt()?;

    let engines = db::get_engines().await?;
    let engine_list = engines
        .iter()
        .map(|e| e.path.as_str())
        .collect::<Vec<&str>>();
    let engine_selection =
        inquire::Select::new("Pick the engine you want to use", engine_list).prompt()?;

    let iwads = db::get_iwads().await?;
    let iwad_list = iwads.iter().map(|i| i.path.as_str()).collect::<Vec<&str>>();
    let iwad_selection =
        inquire::Select::new("Pick the IWAD you want to use", iwad_list).prompt()?;

    let pwads = db::get_pwads().await?;
    let pwad_list = pwads.iter().map(|i| i.path.as_str()).collect::<Vec<&str>>();
    let pwad_selection =
        inquire::Select::new("Pick the PWAD you want to use", pwad_list).prompt()?;

    let engine_id = engines
        .iter()
        .find(|e| e.path == engine_selection)
        .unwrap()
        .id;
    let iwad_id = iwads.iter().find(|i| i.path == iwad_selection).unwrap().id;
    let pwad_id = pwads.iter().find(|p| p.path == pwad_selection).unwrap().id;

    let profile = data::Profile {
        id: 0,
        name: profile_name,
        engine_id: Some(engine_id),
        iwad_id: Some(iwad_id),
        pwad_id: Some(pwad_id),
    };
    db::add_profile(profile)
        .await
        .wrap_err("Failed to add profile")?;

    Ok("Created a new profile".to_string())
}

async fn edit_profile() -> Result<String, eyre::Report> {
    todo!("Edit a profile")
}

async fn active_profile() -> Result<String, eyre::Report> {
    let profile_list = db::get_profile_display_list().await?;
    // Generate a list of profiles showing the full details
    let profile =
        inquire::Select::new("Pick the profile to mark as active", profile_list).prompt()?;

    // TODO: Update Settings table...
    let settings = db::get_settings().await?;
    db::update_settings_active_profile(settings.id, profile.id)
        .await
        .wrap_err("Failed to update active profile")?;

    Ok(format!("Marked profile '{}' as active", profile))
}

