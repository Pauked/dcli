use std::str::FromStr;

use strum_macros::Display;
use strum_macros::EnumString;

use crate::data;

pub const ARG_PLAY: &str = "--play";
//pub const ARG_PROFILES: &str = "--profiles";
//pub const ARG_CONFIG: &str = "--config";
pub const ARG_MAP_EDITOR: &str = "--mapeditor";
pub const ARG_INIT: &str = "--init";
pub const ARG_RESET: &str = "--reset";

#[derive(Debug, PartialEq, EnumString, Display)]
pub enum MainCommand {
    #[strum(serialize = "Play Active Profile")]
    PlayActiveProfile,
    #[strum(serialize = "Play Last Profile")]
    PlayLastProfile,
    #[strum(serialize = "Pick & Play Profile")]
    PickAndPlayProfile,
    #[strum(serialize = "Map Editor")]
    MapEditor,
    Profiles,
    #[strum(serialize = "Game Settings")]
    GameSettings,
    #[strum(serialize = "Config Engines and WADs")]
    Config,
    Quit,
    Unknown,
}

pub fn convert_arg_to_maincommand(arg: &str) -> MainCommand {
    match arg {
        ARG_PLAY => MainCommand::PlayActiveProfile,
        // ARG_PROFILES => MainCommand::Profiles,
        // ARG_CONFIG => MainCommand::Config,
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

#[derive(Debug, Clone, PartialEq, EnumString, Display)]
pub enum ConfigCommand {
    #[strum(serialize = "List Engines and WADs")]
    List,
    #[strum(serialize = "List Engines")]
    ListEngines,
    #[strum(serialize = "List Internal WADs")]
    ListIwads,
    #[strum(serialize = "List Patch WADs")]
    ListPwads,
    #[strum(serialize = "List App Settings")]
    ListAppSettings,
    #[strum(serialize = "Update Engines and WADs")]
    Update,
    #[strum(serialize = "Update Engines")]
    UpdateEngines,
    #[strum(serialize = "Update Internal WADs")]
    UpdateIwads,
    #[strum(serialize = "Update Patch WADs")]
    UpdatePwads,
    Init,
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

#[derive(Debug, Clone, PartialEq, EnumString, Display)]
pub enum GameSettingsCommand {
    #[strum(serialize = "Comp Level")]
    CompLevel,
    #[strum(serialize = "Fast Monsters")]
    FastMonsters,
    #[strum(serialize = "No Monsters")]
    NoMonsters,
    #[strum(serialize = "Respawn Monsters")]
    RespawnMonsters,
    #[strum(serialize = "Warp to Level")]
    WarpToLevel,
    Skill,
    Turbo,
    Timer,
    #[strum(serialize = "Screen Width")]
    Width,
    #[strum(serialize = "Screen Height")]
    Height,
    #[strum(serialize = "Full Screen")]
    FullScreen,
    Windowed,
    #[strum(serialize = "Additional Arguments")]
    AdditionalArguments,
    Back,
    Unknown,
}

pub enum MapEditorCommand {
    OpenMapEditor,

    Back,
    Unknown,
}

pub fn main_menu_prompt() -> MainCommand {
    let selections = vec![
        MainCommand::PlayActiveProfile.to_string(),
        MainCommand::PickAndPlayProfile.to_string(),
        MainCommand::Profiles.to_string(),
        MainCommand::GameSettings.to_string(),
        MainCommand::Config.to_string(),
        MainCommand::Quit.to_string(),
    ];

    // clearscreen::clear().unwrap();
    let choice = inquire::Select::new("Select a Main Menu option", selections)
        .prompt()
        .unwrap();
    MainCommand::from_str(&choice).unwrap()
}

pub fn profiles_menu_prompt() -> ProfileCommand {
    let selections = vec![
        ProfileCommand::New.to_string(),
        ProfileCommand::Edit.to_string(),
        ProfileCommand::Active.to_string(),
        ProfileCommand::List.to_string(),
        ProfileCommand::Delete.to_string(),
        ProfileCommand::Back.to_string(),
    ];

    // clearscreen::clear().unwrap();
    let choice: String = inquire::Select::new("Select a Profile option", selections)
        .prompt()
        .unwrap();
    ProfileCommand::from_str(&choice).unwrap()
}

pub fn config_menu_prompt() -> ConfigCommand {
    let selections = vec![
        ConfigCommand::Update.to_string(),
        ConfigCommand::List.to_string(),
        ConfigCommand::Init.to_string(),
        ConfigCommand::Reset.to_string(),
        ConfigCommand::Back.to_string(),
    ];

    // clearscreen::clear().unwrap();
    let choice: String = inquire::Select::new("Select a Config option", selections)
        .prompt()
        .unwrap();
    ConfigCommand::from_str(&choice).unwrap()
}

pub fn config_list_menu_prompt() -> ConfigCommand {
    let selections = vec![
        ConfigCommand::ListEngines.to_string(),
        ConfigCommand::ListIwads.to_string(),
        ConfigCommand::ListPwads.to_string(),
        ConfigCommand::ListAppSettings.to_string(),
        ConfigCommand::Back.to_string(),
    ];

    let choice: String = inquire::Select::new("Select a Config / List option", selections)
        .prompt()
        .unwrap();
    ConfigCommand::from_str(&choice).unwrap()
}

pub fn config_update_menu_prompt() -> ConfigCommand {
    let selections = vec![
        ConfigCommand::UpdateEngines.to_string(),
        ConfigCommand::UpdateIwads.to_string(),
        ConfigCommand::UpdatePwads.to_string(),
        ConfigCommand::Back.to_string(),
    ];

    let choice: String = inquire::Select::new("Select an Config / Update option", selections)
        .prompt()
        .unwrap();
    ConfigCommand::from_str(&choice).unwrap()
}

pub fn game_settings_menu_prompt(game_settings: data::GameSettings) -> GameSettingsCommand {
    let selections = vec![
        format!(
            "{} ({})",
            GameSettingsCommand::CompLevel.to_string(),
            data::display_option_comp_level(&game_settings.comp_level)
        ),
        format!(
            "{} ({})",
            GameSettingsCommand::FastMonsters.to_string(),
            game_settings.fast_monsters
        ),
        format!(
            "{} ({})",
            GameSettingsCommand::NoMonsters.to_string(),
            game_settings.no_monsters
        ),
        format!(
            "{} ({})",
            GameSettingsCommand::RespawnMonsters.to_string(),
            game_settings.respawn_monsters
        ),
        format!(
            "{} ({})",
            GameSettingsCommand::WarpToLevel.to_string(),
            data::display_option_string(&game_settings.warp)
        ),
        format!(
            "{} ({})",
            GameSettingsCommand::Skill.to_string(),
            data::display_option_u8(&game_settings.skill)
        ),
        format!(
            "{} ({})",
            GameSettingsCommand::Turbo.to_string(),
            data::display_option_u8(&game_settings.turbo)
        ),
        format!(
            "{} ({})",
            GameSettingsCommand::Timer.to_string(),
            data::display_option_u32(&game_settings.timer)
        ),
        format!(
            "{} ({})",
            GameSettingsCommand::Width.to_string(),
            data::display_option_u32(&game_settings.width)
        ),
        format!(
            "{} ({})",
            GameSettingsCommand::Height.to_string(),
            data::display_option_u32(&game_settings.height)
        ),
        format!(
            "{} ({})",
            GameSettingsCommand::FullScreen.to_string(),
            game_settings.full_screen
        ),
        format!(
            "{} ({})",
            GameSettingsCommand::Windowed.to_string(),
            game_settings.windowed
        ),
        format!(
            "{} ({})",
            GameSettingsCommand::AdditionalArguments.to_string(),
            data::display_option_string(&game_settings.additional_arguments)
        ),
        GameSettingsCommand::Back.to_string(),
    ];

    // clearscreen::clear().unwrap();
    let choice: String = inquire::Select::new("Select a Game Settings option", selections)
        .with_page_size(15)
        .prompt()
        .unwrap();

    let first_part = choice.split('(').next().unwrap_or("").trim();
    GameSettingsCommand::from_str(first_part).unwrap()
}
