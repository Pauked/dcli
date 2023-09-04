use strum_macros::Display;
use strum_macros::EnumString;

pub const APP_NAME: &str = "Doom CLI";

// pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

// pub const OS_MACOS: &str = "macos";
pub const OS_WINDOWS: &str = "windows";

// pub const CONFIG_FILE: &str = "App.toml";

pub const ARG_PLAY: &str = "--play";
pub const ARG_PROFILES: &str = "--profiles";
pub const ARG_CONFIG: &str = "--config";
pub const ARG_EDITOR: &str = "--editor";
pub const ARG_INIT: &str = "--init";
pub const ARG_RESET: &str = "--reset";

#[derive(Debug, PartialEq, EnumString, Display)]
pub enum Command {
    Play,
    Profiles,
    ShowSettings,
    NotepadConfig,
    Editor,
    Init,
    Reset,
    Quit,
    UserInput,
}

pub fn convert_arg_to_cmd(arg: &str) -> Command {
    match arg {
        ARG_PLAY => Command::Play,
        ARG_PROFILES => Command::Profiles,
        ARG_EDITOR => Command::Editor,
        ARG_CONFIG => Command::NotepadConfig,
        ARG_INIT => Command::Init,
        ARG_RESET => Command::Reset,
        _ => Command::UserInput,
    }
}

#[derive(Debug, PartialEq, EnumString, Display)]
pub enum ProfileCommand {
    New,
    Edit,
    Delete,
    Active,
    Back,
    UserInput,
}