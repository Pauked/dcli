use strum_macros::Display;
use strum_macros::EnumString;

pub const APP_NAME: &str = "Doom CLI";

// pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

// pub const OS_MACOS: &str = "macos";
// pub const OS_WINDOWS: &str = "windows";

pub const CONFIG_FILE: &str = "App.toml";

pub const ARG_PLAY: &str = "--play";
pub const ARG_CONFIG: &str = "--config";
pub const ARG_EDITOR: &str = "--editor";
pub const ARG_INIT: &str = "--init";
pub const ARG_RESET: &str = "--reset";

#[derive(Debug, PartialEq, EnumString, Display)]
pub enum Command {
    Play,
    Config,
    Editor,
    Init,
    Reset,
    Quit,
    UserInput,
}

pub fn convert_arg_to_cmd(arg: &str) -> Command {
    match arg {
        ARG_PLAY => Command::Play,
        ARG_EDITOR => Command::Editor,
        ARG_CONFIG => Command::Config,
        ARG_INIT => Command::Init,
        ARG_RESET => Command::Reset,
        _ => Command::UserInput,
    }
}