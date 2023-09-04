use std::str::FromStr;

use crate::constants;

pub fn main_menu_prompt() -> constants::Command {
    let selections = vec![
        constants::Command::Play.to_string(),
        constants::Command::Profiles.to_string(),
        constants::Command::ShowSettings.to_string(),
        // constants::Command::NotepadConfig.to_string(),
        // constants::Command::Editor.to_string(),
        constants::Command::Init.to_string(),
        constants::Command::Reset.to_string(),
        constants::Command::Quit.to_string(),
    ];

    let choice = inquire::Select::new("Select a Main Menu option", selections)
        .prompt()
        .unwrap();
    constants::Command::from_str(&choice).unwrap()
}

pub fn profiles_menu_prompt() -> constants::ProfileCommand {
    let selections = vec![
        constants::ProfileCommand::New.to_string(),
        constants::ProfileCommand::Edit.to_string(),
        constants::ProfileCommand::Delete.to_string(),
        constants::ProfileCommand::Active.to_string(),
        constants::ProfileCommand::Back.to_string(),
    ];

    let choice: String = inquire::Select::new("Select a Profile option", selections)
        .prompt()
        .unwrap();
    constants::ProfileCommand::from_str(&choice).unwrap()
}
