use std::str::FromStr;

use crate::constants;

pub fn main_menu_prompt() -> constants::MainCommand {
    let selections = vec![
        constants::MainCommand::Play.to_string(),
        constants::MainCommand::Profiles.to_string(),
        constants::MainCommand::Config.to_string(),
        constants::MainCommand::Quit.to_string(),
    ];

    let choice = inquire::Select::new("Select a Main Menu option", selections)
        .prompt()
        .unwrap();
    constants::MainCommand::from_str(&choice).unwrap()
}

pub fn profiles_menu_prompt() -> constants::ProfileCommand {
    let selections = vec![
        constants::ProfileCommand::Active.to_string(),
        constants::ProfileCommand::List.to_string(),
        constants::ProfileCommand::New.to_string(),
        constants::ProfileCommand::Edit.to_string(),
        constants::ProfileCommand::Delete.to_string(),
        constants::ProfileCommand::Back.to_string(),
    ];

    let choice: String = inquire::Select::new("Select a Profile option", selections)
        .prompt()
        .unwrap();
    constants::ProfileCommand::from_str(&choice).unwrap()
}

pub fn config_menu_prompt() -> constants::ConfigCommand {
    let selections = vec![
        constants::ConfigCommand::List.to_string(),
        constants::ConfigCommand::Init.to_string(),
        constants::ConfigCommand::Reset.to_string(),
        constants::ConfigCommand::Back.to_string(),
    ];

    let choice: String = inquire::Select::new("Select a Config option", selections)
        .prompt()
        .unwrap();
    constants::ConfigCommand::from_str(&choice).unwrap()
}
