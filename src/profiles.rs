use log::info;

use crate::{constants, data, db, tui};

pub async fn profiles() -> Result<String, eyre::Report> {
    // Menu:
    loop {
        let menu_command = tui::profiles_menu_prompt();

        match menu_command {
            constants::ProfileCommand::New => {
                // Create a new profile
                // let profile = profiles::create_profile()?;
                // // Save the profile
                // profiles::save_profile(profile)?;
                // info!("Created a new profile")
                new_profile().await?;
            }
            constants::ProfileCommand::Edit => {
                // Edit a profile
                // let profile = profiles::edit_profile()?;
                // // Save the profile
                // profiles::save_profile(profile)?;
                info!("Edited a profile")
            }
            constants::ProfileCommand::Delete => {
                // Delete a profile
                // let profile = profiles::delete_profile()?;
                // // Save the profile
                // profiles::save_profile(profile)?;
                info!("Deleted a profile")
            }
            constants::ProfileCommand::Active => {
                // Set the active profile
                // let profile = profiles::set_active_profile()?;
                // // Save the profile
                // profiles::save_profile(profile)?;
                info!("Set the active profile")
            }
            constants::ProfileCommand::Back => {
                // Back to main menu
                return Ok("Back to main menu".to_string());
            }
            constants::ProfileCommand::UserInput => todo!(),
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
    db::add_profile(&profile).await?;

    Ok("Created a new profile".to_string())
}
