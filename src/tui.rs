use std::str::FromStr;

use colored::Colorize;
use log::error;
use log::info;
use strum_macros::Display;
use strum_macros::EnumString;

use crate::data;
use crate::db;
use crate::menu_config;
use crate::menu_main;
use crate::menu_map_editor;
use crate::menu_play_settings;
use crate::menu_profiles;
use crate::menu_view_readme;

const ARG_PLAY: &str = "--play";
const ARG_PLAY_LAST: &str = "--playlast";
const ARG_MAP_EDITOR: &str = "--mapeditor";
const ARG_MAP_EDITOR_LAST: &str = "--mapeditorlast";
const ARG_INIT: &str = "--init";
pub const ARG_RESET: &str = "--reset";
pub const ARG_VERSION: &str = "--version";

pub const MENU_PAGE_SIZE: usize = 15;

pub enum MenuLevel {
    Main,
    Profiles,
    GameSettings,
    MapEditor,
    ViewReadme,
    Config,
    ConfigList,
    ConfigUpdate,
}

#[derive(Debug, PartialEq, EnumString, Display)]
pub enum MenuCommand {
    // Main Menu
    #[strum(serialize = "Play Active Profile")]
    PlayActiveProfile,
    #[strum(serialize = "Play Last Profile")]
    PlayLastProfile,
    #[strum(serialize = "Pick & Play Profile")]
    PickAndPlayProfile,
    #[strum(serialize = "Pick & Play PWAD")]
    PickAndPlayPwad,
    #[strum(serialize = "Map Editor >>")]
    MapEditor,
    #[strum(serialize = "Profiles >>")]
    Profiles,
    #[strum(serialize = "Play Settings >>")]
    PlaySettings,
    #[strum(serialize = "View Map Readme >>")]
    ViewMapReadme,
    #[strum(serialize = "Config App >>")]
    Config,

    // Profile Menu
    #[strum(serialize = "New Profile")]
    NewProfile,
    #[strum(serialize = "Edit Profile")]
    EditProfile,
    #[strum(serialize = "Delete Profile")]
    DeleteProfile,
    #[strum(serialize = "Set Active Profile")]
    ActiveProfile,
    #[strum(serialize = "List Profiles")]
    ListProfile,

    // Config Menu
    #[strum(serialize = "List Stored Data >>")]
    ListStoredData,
    #[strum(serialize = "List Engines")]
    ListEngines,
    #[strum(serialize = "List Internal WADs")]
    ListIwads,
    #[strum(serialize = "List Patch WADs")]
    ListPwads,
    #[strum(serialize = "List App Settings")]
    ListAppSettings,
    #[strum(serialize = "Update Stored Data >>")]
    UpdateStoredData,
    #[strum(serialize = "Update Engines")]
    UpdateEngines,
    #[strum(serialize = "Update Internal WADs")]
    UpdateIwads,
    #[strum(serialize = "Update Patch WADs")]
    UpdatePwads,
    Init,
    Reset,

    // Play Settings Menu
    #[strum(serialize = "Compatibility Level")]
    CompLevel,
    #[strum(serialize = "Config File")]
    ConfigFile,
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
    #[strum(serialize = "Reset Play Settings")]
    ResetPlaySettings,

    // Map Editor Menu
    #[strum(serialize = "Open from Active Profile PWAD")]
    OpenFromActiveProfile,
    #[strum(serialize = "Open from Last Profile PWAD")]
    OpenFromLastProfile,
    #[strum(serialize = "Open from Pick Profile")]
    OpenFromPickProfile,
    #[strum(serialize = "Open from Pick PWAD")]
    OpenFromPickPwad,
    #[strum(serialize = "Set Active Map Editor")]
    ActiveMapEditor,
    #[strum(serialize = "List Map Editors")]
    ListMapEditor,
    #[strum(serialize = "Update Map Editors")]
    UpdateMapEditor,
    #[strum(serialize = "Delete Map Editors")]
    DeleteMapEditor,

    // View Readme Menu
    #[strum(serialize = "View from Active Profile")]
    ViewFromActiveProfile,
    #[strum(serialize = "View from Last Profile")]
    ViewFromLastProfile,
    #[strum(serialize = "View from Pick Profile")]
    ViewFromPickProfile,
    #[strum(serialize = "View from Pick PWAD")]
    ViewFromPickPwad,

    #[strum(serialize = "Back <ESC>")]
    Back,
    #[strum(serialize = "Quit <ESC>")]
    Quit,

    Ignore,
}

pub fn menu_prompt(menu_level: &MenuLevel) -> Result<MenuCommand, eyre::Report> {
    let (selections, menu_name) = match menu_level {
        MenuLevel::Main => {
            let selections = vec![
                MenuCommand::PlayActiveProfile.to_string(),
                MenuCommand::PlayLastProfile.to_string(),
                MenuCommand::PickAndPlayProfile.to_string(),
                MenuCommand::PickAndPlayPwad.to_string(),
                MenuCommand::PlaySettings.to_string(),
                MenuCommand::Profiles.to_string(),
                MenuCommand::MapEditor.to_string(),
                MenuCommand::ViewMapReadme.to_string(),
                MenuCommand::Config.to_string(),
                MenuCommand::Quit.to_string(),
            ];
            (selections, "Main Menu".to_string())
        }
        MenuLevel::Profiles => {
            let selections = vec![
                MenuCommand::NewProfile.to_string(),
                MenuCommand::EditProfile.to_string(),
                MenuCommand::ActiveProfile.to_string(),
                MenuCommand::ListProfile.to_string(),
                MenuCommand::DeleteProfile.to_string(),
                MenuCommand::Back.to_string(),
            ];
            (selections, "Profile".to_string())
        }
        MenuLevel::GameSettings => {
            let play_settings = db::get_play_settings()?;
            let selections = vec![
                format!(
                    "{} ({})",
                    MenuCommand::CompLevel.to_string(),
                    data::display_option_comp_level(&play_settings.comp_level)
                ),
                format!(
                    "{} ({})",
                    MenuCommand::ConfigFile.to_string(),
                    data::display_option_string(&play_settings.config_file)
                ),
                format!(
                    "{} ({})",
                    MenuCommand::FastMonsters.to_string(),
                    play_settings.fast_monsters
                ),
                format!(
                    "{} ({})",
                    MenuCommand::NoMonsters.to_string(),
                    play_settings.no_monsters
                ),
                format!(
                    "{} ({})",
                    MenuCommand::RespawnMonsters.to_string(),
                    play_settings.respawn_monsters
                ),
                format!(
                    "{} ({})",
                    MenuCommand::WarpToLevel.to_string(),
                    data::display_option_string(&play_settings.warp)
                ),
                format!(
                    "{} ({})",
                    MenuCommand::Skill.to_string(),
                    data::display_option_u8(&play_settings.skill)
                ),
                format!(
                    "{} ({})",
                    MenuCommand::Turbo.to_string(),
                    data::display_option_u8(&play_settings.turbo)
                ),
                format!(
                    "{} ({})",
                    MenuCommand::Timer.to_string(),
                    data::display_option_u32(&play_settings.timer)
                ),
                format!(
                    "{} ({})",
                    MenuCommand::Width.to_string(),
                    data::display_option_u32(&play_settings.width)
                ),
                format!(
                    "{} ({})",
                    MenuCommand::Height.to_string(),
                    data::display_option_u32(&play_settings.height)
                ),
                format!(
                    "{} ({})",
                    MenuCommand::FullScreen.to_string(),
                    play_settings.full_screen
                ),
                format!(
                    "{} ({})",
                    MenuCommand::Windowed.to_string(),
                    play_settings.windowed
                ),
                format!(
                    "{} ({})",
                    MenuCommand::AdditionalArguments.to_string(),
                    data::display_option_string(&play_settings.additional_arguments)
                ),
                MenuCommand::ResetPlaySettings.to_string(),
                MenuCommand::Back.to_string(),
            ];
            (selections, "Play Settings".to_string())
        }
        MenuLevel::MapEditor => {
            let selections = vec![
                MenuCommand::OpenFromActiveProfile.to_string(),
                MenuCommand::OpenFromLastProfile.to_string(),
                MenuCommand::OpenFromPickProfile.to_string(),
                MenuCommand::OpenFromPickPwad.to_string(),
                MenuCommand::ActiveMapEditor.to_string(),
                MenuCommand::UpdateMapEditor.to_string(),
                MenuCommand::DeleteMapEditor.to_string(),
                MenuCommand::ListMapEditor.to_string(),
                MenuCommand::Back.to_string(),
            ];
            (selections, "Map Editor".to_string())
        }
        MenuLevel::ViewReadme => {
            let selections = vec![
                MenuCommand::ViewFromActiveProfile.to_string(),
                MenuCommand::ViewFromLastProfile.to_string(),
                MenuCommand::ViewFromPickProfile.to_string(),
                MenuCommand::ViewFromPickPwad.to_string(),
                MenuCommand::Back.to_string(),
            ];
            (selections, "Readme".to_string())
        }
        MenuLevel::Config => {
            let selections = vec![
                MenuCommand::UpdateStoredData.to_string(),
                MenuCommand::ListStoredData.to_string(),
                MenuCommand::Init.to_string(),
                MenuCommand::Reset.to_string(),
                MenuCommand::Back.to_string(),
            ];
            (selections, "Config".to_string())
        }
        MenuLevel::ConfigList => {
            let selections = vec![
                MenuCommand::ListEngines.to_string(),
                MenuCommand::ListIwads.to_string(),
                MenuCommand::ListPwads.to_string(),
                MenuCommand::ListMapEditor.to_string(),
                MenuCommand::ListAppSettings.to_string(),
                MenuCommand::Back.to_string(),
            ];
            (selections, "Config / List".to_string())
        }
        MenuLevel::ConfigUpdate => {
            let selections = vec![
                MenuCommand::UpdateEngines.to_string(),
                MenuCommand::UpdateIwads.to_string(),
                MenuCommand::UpdatePwads.to_string(),
                MenuCommand::UpdateMapEditor.to_string(),
                MenuCommand::Back.to_string(),
            ];
            (selections, "Config / Update".to_string())
        }
    };

    let choice = inquire::Select::new(&format!("Select a {} option:", menu_name), selections)
        .with_page_size(MENU_PAGE_SIZE)
        .prompt_skippable()
        .unwrap();

    match choice {
        Some(choice) => {
            let first_part = choice.split('(').next().unwrap_or("").trim();
            Ok(MenuCommand::from_str(first_part).unwrap())
        }
        None => Ok(MenuCommand::Back),
    }
}

pub fn menu(menu_level: MenuLevel) -> Result<String, eyre::Report> {
    clearscreen::clear().unwrap();
    loop {
        if let MenuLevel::Main = menu_level {
            info!("{}", menu_main::get_active_profile_text()?);
            info!("{}", menu_main::get_last_profile_text()?);
        }

        let menu_command = menu_prompt(&menu_level)?;

        if let MenuCommand::Back = menu_command {
            return Ok("".to_string());
        }
        if let MenuCommand::Quit = menu_command {
            return Ok("Quitting...".to_string());
        }

        let result = run_menu_command(menu_command);
        clearscreen::clear().unwrap();
        match result {
            Ok(result) => info!("{}", result.green()),
            Err(err) => {
                error!("Error: {:?}", err);
            }
        }
    }
}

pub fn run_menu_command(menu_command: MenuCommand) -> Result<String, eyre::Report> {
    match menu_command {
        // Main Menu
        MenuCommand::PlayActiveProfile => menu_main::play_active_profile(),
        MenuCommand::PlayLastProfile => menu_main::play_last_profile(),
        MenuCommand::PickAndPlayProfile => menu_main::pick_and_play_profile(),
        MenuCommand::PickAndPlayPwad => menu_main::pick_and_play_pwad(),
        MenuCommand::MapEditor => menu(MenuLevel::MapEditor),
        MenuCommand::Profiles => menu(MenuLevel::Profiles),
        MenuCommand::PlaySettings => menu(MenuLevel::GameSettings),
        MenuCommand::ViewMapReadme => menu(MenuLevel::ViewReadme),
        MenuCommand::Config => menu(MenuLevel::Config),

        // Profile Menu
        MenuCommand::NewProfile => menu_profiles::new_profile(),
        MenuCommand::EditProfile => menu_profiles::edit_profile(),
        MenuCommand::DeleteProfile => menu_profiles::delete_profile(),
        MenuCommand::ActiveProfile => menu_profiles::set_active_profile(),
        MenuCommand::ListProfile => menu_profiles::list_profiles(),

        // Config Menu
        MenuCommand::ListStoredData => menu(MenuLevel::ConfigList),
        MenuCommand::ListEngines => menu_config::list_engines(),
        MenuCommand::ListIwads => menu_config::list_iwads(),
        MenuCommand::ListPwads => menu_config::list_pwads(),
        MenuCommand::ListAppSettings => menu_config::list_app_settings(),
        MenuCommand::Init => menu_config::init(),
        MenuCommand::UpdateStoredData => menu(MenuLevel::ConfigUpdate),
        MenuCommand::UpdateEngines => {
            let mut app_settings = db::get_app_settings()?;
            let folder = menu_config::init_engines(
                &app_settings.exe_search_folder.unwrap_or("".to_string()),
            )?;
            app_settings.exe_search_folder = Some(folder);
            db::save_app_settings(app_settings)?;
            inquire::Text::new("Press any key to continue...").prompt_skippable()?;
            Ok("Successfully updated Engines".to_string())
        }
        MenuCommand::UpdateIwads => {
            let mut app_settings = db::get_app_settings()?;
            let folder = menu_config::init_iwads(
                &app_settings.iwad_search_folder.unwrap_or("".to_string()),
            )?;
            app_settings.iwad_search_folder = Some(folder);
            db::save_app_settings(app_settings)?;
            inquire::Text::new("Press any key to continue...").prompt_skippable()?;
            Ok("Successfully updated IWADs".to_string())
        }
        MenuCommand::UpdatePwads => {
            let mut app_settings = db::get_app_settings()?;
            let folder = menu_config::init_pwads(
                &app_settings.pwad_search_folder.unwrap_or("".to_string()),
            )?;
            app_settings.pwad_search_folder = Some(folder);
            db::save_app_settings(app_settings)?;
            inquire::Text::new("Press any key to continue...").prompt_skippable()?;
            Ok("Successfully updated PWADs".to_string())
        }
        MenuCommand::Reset => menu_config::reset(false),

        // Play Settings Menu
        MenuCommand::CompLevel => menu_play_settings::update_comp_level(),
        MenuCommand::ConfigFile => menu_play_settings::update_config_file(),
        MenuCommand::FastMonsters => menu_play_settings::update_fast_monsters(),
        MenuCommand::NoMonsters => menu_play_settings::update_no_monsters(),
        MenuCommand::RespawnMonsters => menu_play_settings::update_respawn_monsters(),
        MenuCommand::WarpToLevel => menu_play_settings::update_warp_to_level(),
        MenuCommand::Skill => menu_play_settings::update_skill(),
        MenuCommand::Turbo => menu_play_settings::update_turbo(),
        MenuCommand::Timer => menu_play_settings::update_timer(),
        MenuCommand::Width => menu_play_settings::update_width(),
        MenuCommand::Height => menu_play_settings::update_height(),
        MenuCommand::FullScreen => menu_play_settings::update_full_screen(),
        MenuCommand::Windowed => menu_play_settings::update_windowed(),
        MenuCommand::AdditionalArguments => menu_play_settings::update_additional_arguments(),
        MenuCommand::ResetPlaySettings => menu_play_settings::reset(),

        // Map Editor Menu
        MenuCommand::OpenFromActiveProfile => menu_map_editor::open_from_active_profile(),
        MenuCommand::OpenFromLastProfile => menu_map_editor::open_from_last_profile(),
        MenuCommand::OpenFromPickProfile => menu_map_editor::open_from_pick_profile(),
        MenuCommand::OpenFromPickPwad => menu_map_editor::open_from_pick_pwad(),
        MenuCommand::ActiveMapEditor => menu_map_editor::set_active_map_editor(),
        MenuCommand::ListMapEditor => menu_map_editor::list_map_editors(),
        MenuCommand::UpdateMapEditor => menu_map_editor::update_map_editors(),
        MenuCommand::DeleteMapEditor => menu_map_editor::delete_map_editor(),

        // View Readme Menu
        MenuCommand::ViewFromActiveProfile => menu_view_readme::view_from_active_profile(),
        MenuCommand::ViewFromLastProfile => menu_view_readme::view_from_last_profile(),
        MenuCommand::ViewFromPickProfile => menu_view_readme::view_from_pick_profile(),
        MenuCommand::ViewFromPickPwad => menu_view_readme::view_from_pick_pwad(),

        // Back and Quit
        MenuCommand::Ignore => Ok("".to_string()),
        MenuCommand::Back => Ok("".to_string()),
        MenuCommand::Quit => Ok("Quitting".to_string()),
    }
}

pub fn convert_arg_to_menu_command(arg: &str) -> MenuCommand {
    match arg {
        ARG_INIT => MenuCommand::Init,
        ARG_RESET => MenuCommand::Reset,
        ARG_PLAY => MenuCommand::PlayActiveProfile,
        ARG_PLAY_LAST => MenuCommand::PlayLastProfile,
        ARG_MAP_EDITOR => MenuCommand::OpenFromActiveProfile,
        ARG_MAP_EDITOR_LAST => MenuCommand::OpenFromLastProfile,
        _ => MenuCommand::Ignore,
    }
}
