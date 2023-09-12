use std::str::FromStr;

use strum_macros::Display;
use strum_macros::EnumString;

pub const ARG_PLAY: &str = "--play";
pub const ARG_PROFILES: &str = "--profiles";
pub const ARG_CONFIG: &str = "--config";
// pub const ARG_EDITOR: &str = "--editor";
pub const ARG_INIT: &str = "--init";
pub const ARG_RESET: &str = "--reset";

#[derive(Debug, PartialEq, EnumString, Display)]
pub enum MainCommand {
    #[strum(serialize = "Play Active Profile")]
    PlayActiveProfile,
    #[strum(serialize = "Pick and Play Profile")]
    PickAndPlayProfile,
    Profiles,
    Config,
    Quit,
    Unknown,
}

pub fn convert_arg_to_maincommand(arg: &str) -> MainCommand {
    match arg {
        ARG_PLAY => MainCommand::PlayActiveProfile,
        ARG_PROFILES => MainCommand::Profiles,
        ARG_CONFIG => MainCommand::Config,
        _ => MainCommand::Unknown,
    }
}

#[derive(Debug, PartialEq, EnumString, Display)]
pub enum ProfileCommand {
    New,
    Edit,
    Delete,
    #[strum(serialize = "Set Active Profile")]
    Active,
    List,
    Back,
}

#[derive(Debug, PartialEq, EnumString, Display)]
pub enum ConfigCommand {
    List,
    Init,
    #[strum(serialize = "Update Engines")]
    UpdateEngines,
    #[strum(serialize = "Update Internal WADs")]
    UpdateIwads,
    #[strum(serialize = "Update Patch WADs")]
    UpdatePwads,
    Reset,
    Back,
    Unknown,
}

pub fn convert_arg_to_configcommand(arg: &str) -> ConfigCommand {
    match arg {
        ARG_INIT => ConfigCommand::Init,
        ARG_RESET => ConfigCommand::Reset,
        _ => ConfigCommand::Unknown,
    }
}

pub fn main_menu_prompt() -> MainCommand {
    let selections = vec![
        MainCommand::PlayActiveProfile.to_string(),
        MainCommand::PickAndPlayProfile.to_string(),
        MainCommand::Profiles.to_string(),
        MainCommand::Config.to_string(),
        MainCommand::Quit.to_string(),
    ];

    let choice = inquire::Select::new("Select a Main Menu option", selections)
        .prompt()
        .unwrap();
    MainCommand::from_str(&choice).unwrap()
}

pub fn profiles_menu_prompt() -> ProfileCommand {
    let selections = vec![
        ProfileCommand::Active.to_string(),
        ProfileCommand::List.to_string(),
        ProfileCommand::New.to_string(),
        ProfileCommand::Edit.to_string(),
        ProfileCommand::Delete.to_string(),
        ProfileCommand::Back.to_string(),
    ];

    let choice: String = inquire::Select::new("Select a Profile option", selections)
        .prompt()
        .unwrap();
    ProfileCommand::from_str(&choice).unwrap()
}

pub fn config_menu_prompt() -> ConfigCommand {
    let selections = vec![
        ConfigCommand::List.to_string(),
        ConfigCommand::Init.to_string(),
        ConfigCommand::UpdateEngines.to_string(),
        ConfigCommand::UpdateIwads.to_string(),
        ConfigCommand::UpdatePwads.to_string(),
        ConfigCommand::Reset.to_string(),
        ConfigCommand::Back.to_string(),
    ];

    let choice: String = inquire::Select::new("Select a Config option", selections)
        .prompt()
        .unwrap();
    ConfigCommand::from_str(&choice).unwrap()
}
