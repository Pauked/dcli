use std::str::FromStr;

use dialoguer::{theme::ColorfulTheme, Select};

use crate::constants;

pub fn main_menu_prompt() -> constants::Command {
    let selections = &[
        constants::Command::Play.to_string(),
        constants::Command::Profiles.to_string(),
        constants::Command::ShowSettings.to_string(),
        // constants::Command::NotepadConfig.to_string(),
        // constants::Command::Editor.to_string(),
        constants::Command::Init.to_string(),
        constants::Command::Reset.to_string(),
        constants::Command::Quit.to_string(),
    ];

    let selection_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an option")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    constants::Command::from_str(&selections[selection_index]).unwrap()
}

pub fn profiles_menu_prompt() -> constants::ProfileCommand {
    let selections = &[
        constants::ProfileCommand::New.to_string(),
        constants::ProfileCommand::Edit.to_string(),
        constants::ProfileCommand::Delete.to_string(),
        constants::ProfileCommand::Active.to_string(),
        constants::ProfileCommand::Back.to_string(),
    ];

    let selection_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a Profile option")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    constants::ProfileCommand::from_str(&selections[selection_index]).unwrap()
}