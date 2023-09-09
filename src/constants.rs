use strum_macros::Display;
use strum_macros::EnumString;

pub const APP_NAME: &str = "Doom CLI";

// pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

// pub const OS_MACOS: &str = "macos";
pub const OS_WINDOWS: &str = "windows";

// pub const CONFIG_FILE: &str = "App.toml";

// FIXME: Reinstate these args
pub const ARG_PLAY: &str = "--play";
pub const ARG_PROFILES: &str = "--profiles";
pub const ARG_CONFIG: &str = "--config";
// pub const ARG_EDITOR: &str = "--editor";
pub const ARG_INIT: &str = "--init";
pub const ARG_RESET: &str = "--reset";

#[derive(Debug, PartialEq, EnumString, Display)]
pub enum MainCommand {
    Play,
    Profiles,
    Config,
    Quit,
    Unknown,
}

pub fn convert_arg_to_maincommand(arg: &str) -> MainCommand {
    match arg {
        ARG_PLAY => MainCommand::Play,
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
    Active,
    List,
    Back,
}

#[derive(Debug, PartialEq, EnumString, Display)]
pub enum ConfigCommand {
    List,
    Init,
    UpdateEngines,
    UpdateIwads,
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