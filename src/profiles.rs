// use dialoguer::{theme::ColorfulTheme, Input, Select};
use log::info;

use crate::{constants, tui};

pub fn profiles() -> Result<String, eyre::Report> {
    // Menu:
    loop {
        let menu_command = tui::profiles_menu_prompt();

        match menu_command {
            constants::ProfileCommand::New => {
                // Create a new profile
                // let profile = profiles::create_profile()?;
                // // Save the profile
                // profiles::save_profile(profile)?;
                info!("Created a new profile")
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
        }
    }
}

// async fn new_profile() -> Result<String, eyre::Report> {
//     let profile_name: String = Input::with_theme(&ColorfulTheme::default())
//         .with_prompt("Enter a name for your profile")
//         .interact_text()
//         .unwrap();

//     let engines = db::get_engines().await?;
//     let engine_list = engines
//         .iter()
//         .map(|e| e.path.as_str())
//         .collect::<Vec<&str>>();

//     let engine_selection = Select::with_theme(&ColorfulTheme::default())
//         .with_prompt("Pick the engine you want to use")
//         .items(&engine_list[..])
//         .interact()
//         .unwrap();

//     let iwads = db::get_iwads().await?;
//     let iwad_list = iwads.iter().map(|i| i.path.as_str()).collect::<Vec<&str>>();

//     let iwad_selection = Select::with_theme(&ColorfulTheme::default())
//         .with_prompt("Pick the iwad you want to use")
//         .items(&iwad_list[..])
//         .interact()
//         .unwrap();



//     //let engine =
//     todo!("Finish this")
// }
