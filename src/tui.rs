use std::str::FromStr;

use colored::Colorize;
use log::error;
use log::info;
use strum_macros::Display;
use strum_macros::EnumString;

use crate::constants;
use crate::data;
use crate::db;
use crate::menu_app_settings;
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
    AppSettings,
    AppSettingsDefaults,
    AppSettingsList,
    AppSettingsUpdate,
}

#[derive(Clone, Debug, PartialEq, EnumString, Display, sqlx::Type)]
pub enum MenuMode {
    Full,
    Simple,
}

#[derive(Debug, PartialEq, EnumString, Display)]
pub enum MenuCommand {
    // Main Menu
    #[strum(serialize = "Play Default Profile")]
    PlayDefaultProfile,
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
    #[strum(serialize = "App Settings >>")]
    AppSettings,

    // Profile Menu
    #[strum(serialize = "New Profile")]
    NewProfile,
    #[strum(serialize = "Edit Profile")]
    EditProfile,
    #[strum(serialize = "Delete Profile")]
    DeleteProfile,
    #[strum(serialize = "Set Default Profile")]
    SetDefaultProfile,
    #[strum(serialize = "List Profiles")]
    ListProfile,

    // App Settings Menu
    #[strum(serialize = "Menu Mode")]
    MenuMode,
    #[strum(serialize = "Set Defaults >>")]
    SetDefaults,
    #[strum(serialize = "Set Default Engine")]
    SetDefaultEngine,
    #[strum(serialize = "Set Default Internal WAD")]
    SetDefaultIwad,
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
    #[strum(serialize = "Open from Default Profile PWAD")]
    OpenFromDefaultProfile,
    #[strum(serialize = "Open from Last Profile PWAD")]
    OpenFromLastProfile,
    #[strum(serialize = "Open from Pick Profile")]
    OpenFromPickProfile,
    #[strum(serialize = "Open from Pick PWAD")]
    OpenFromPickPwad,
    #[strum(serialize = "Set Default Map Editor")]
    SetDefaultMapEditor,
    #[strum(serialize = "List Map Editors")]
    ListMapEditor,
    #[strum(serialize = "Update Map Editors")]
    UpdateMapEditor,
    #[strum(serialize = "Delete Map Editors")]
    DeleteMapEditor,

    // View Readme Menu
    #[strum(serialize = "View from Default Profile")]
    ViewFromDefaultProfile,
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

pub fn menu_prompt(
    menu_mode: &MenuMode,
    menu_level: &MenuLevel,
) -> Result<MenuCommand, eyre::Report> {
    let (selections, menu_name, help_message) = match menu_level {
        MenuLevel::Main => {
            let selections = vec![
                (
                    MenuCommand::PlayDefaultProfile.to_string(),
                    MenuMode::Simple,
                ),
                (MenuCommand::PlayLastProfile.to_string(), MenuMode::Full),
                (MenuCommand::PickAndPlayProfile.to_string(), MenuMode::Full),
                (MenuCommand::PickAndPlayPwad.to_string(), MenuMode::Simple),
                (MenuCommand::PlaySettings.to_string(), MenuMode::Simple),
                (MenuCommand::Profiles.to_string(), MenuMode::Simple),
                (MenuCommand::MapEditor.to_string(), MenuMode::Full),
                (MenuCommand::ViewMapReadme.to_string(), MenuMode::Full),
                (MenuCommand::AppSettings.to_string(), MenuMode::Simple),
                (MenuCommand::Quit.to_string(), MenuMode::Simple),
            ];
            (
                selections,
                "Main Menu".to_string(),
                "Lets play Doom!".green().to_string(),
            )
        }
        MenuLevel::Profiles => {
            let selections = vec![
                (MenuCommand::NewProfile.to_string(), MenuMode::Simple),
                (MenuCommand::EditProfile.to_string(), MenuMode::Simple),
                (MenuCommand::SetDefaultProfile.to_string(), MenuMode::Simple),
                (MenuCommand::ListProfile.to_string(), MenuMode::Simple),
                (MenuCommand::DeleteProfile.to_string(), MenuMode::Simple),
                (MenuCommand::Back.to_string(), MenuMode::Simple),
            ];
            (
                selections,
                "Profile".to_string(),
                "Profiles group together Engines, IWADs and PWADs".to_string(),
            )
        }
        MenuLevel::GameSettings => {
            let play_settings = db::get_play_settings()?;
            let selections = vec![
                (
                    format!(
                        "{} ({})",
                        MenuCommand::CompLevel,
                        data::display_option_comp_level(&play_settings.comp_level)
                    ),
                    MenuMode::Simple,
                ),
                (
                    format!(
                        "{} ({})",
                        MenuCommand::ConfigFile,
                        data::display_option_string(&play_settings.config_file)
                    ),
                    MenuMode::Simple,
                ),
                (
                    format!(
                        "{} ({})",
                        MenuCommand::FastMonsters,
                        play_settings.fast_monsters
                    ),
                    MenuMode::Simple,
                ),
                (
                    format!(
                        "{} ({})",
                        MenuCommand::NoMonsters,
                        play_settings.no_monsters
                    ),
                    MenuMode::Simple,
                ),
                (
                    format!(
                        "{} ({})",
                        MenuCommand::RespawnMonsters,
                        play_settings.respawn_monsters
                    ),
                    MenuMode::Simple,
                ),
                (
                    format!(
                        "{} ({})",
                        MenuCommand::WarpToLevel,
                        data::display_option_string(&play_settings.warp)
                    ),
                    MenuMode::Simple,
                ),
                (
                    format!(
                        "{} ({})",
                        MenuCommand::Skill,
                        data::display_option_u8(&play_settings.skill)
                    ),
                    MenuMode::Simple,
                ),
                (
                    format!(
                        "{} ({})",
                        MenuCommand::Turbo,
                        data::display_option_u8(&play_settings.turbo)
                    ),
                    MenuMode::Simple,
                ),
                (
                    format!(
                        "{} ({})",
                        MenuCommand::Timer,
                        data::display_option_u32(&play_settings.timer)
                    ),
                    MenuMode::Simple,
                ),
                (
                    format!(
                        "{} ({})",
                        MenuCommand::Width,
                        data::display_option_u32(&play_settings.width)
                    ),
                    MenuMode::Simple,
                ),
                (
                    format!(
                        "{} ({})",
                        MenuCommand::Height,
                        data::display_option_u32(&play_settings.height)
                    ),
                    MenuMode::Simple,
                ),
                (
                    format!(
                        "{} ({})",
                        MenuCommand::FullScreen,
                        play_settings.full_screen
                    ),
                    MenuMode::Simple,
                ),
                (
                    format!("{} ({})", MenuCommand::Windowed, play_settings.windowed),
                    MenuMode::Simple,
                ),
                (
                    format!(
                        "{} ({})",
                        MenuCommand::AdditionalArguments,
                        data::display_option_string(&play_settings.additional_arguments)
                    ),
                    MenuMode::Simple,
                ),
                (MenuCommand::ResetPlaySettings.to_string(), MenuMode::Simple),
                (MenuCommand::Back.to_string(), MenuMode::Simple),
            ];
            (
                selections,
                "Play Settings".to_string(),
                "These Settings apply when you Play".to_string(),
            )
        }
        MenuLevel::MapEditor => {
            let selections = vec![
                (
                    MenuCommand::OpenFromDefaultProfile.to_string(),
                    MenuMode::Full,
                ),
                (MenuCommand::OpenFromLastProfile.to_string(), MenuMode::Full),
                (MenuCommand::OpenFromPickProfile.to_string(), MenuMode::Full),
                (MenuCommand::OpenFromPickPwad.to_string(), MenuMode::Full),
                (MenuCommand::SetDefaultMapEditor.to_string(), MenuMode::Full),
                (MenuCommand::UpdateMapEditor.to_string(), MenuMode::Full),
                (MenuCommand::DeleteMapEditor.to_string(), MenuMode::Full),
                (MenuCommand::ListMapEditor.to_string(), MenuMode::Full),
                (MenuCommand::Back.to_string(), MenuMode::Simple),
            ];
            (
                selections,
                "Map Editor".to_string(),
                "Quickly view/edit a PWAD in a Map Editor".to_string(),
            )
        }
        MenuLevel::ViewReadme => {
            let selections = vec![
                (
                    MenuCommand::ViewFromDefaultProfile.to_string(),
                    MenuMode::Full,
                ),
                (MenuCommand::ViewFromLastProfile.to_string(), MenuMode::Full),
                (MenuCommand::ViewFromPickProfile.to_string(), MenuMode::Full),
                (MenuCommand::ViewFromPickPwad.to_string(), MenuMode::Full),
                (MenuCommand::Back.to_string(), MenuMode::Simple),
            ];
            (
                selections,
                "Readme".to_string(),
                "Quickly view the Readme for a PWAD".to_string(),
            )
        }
        MenuLevel::AppSettings => {
            let app_settings = db::get_app_settings()?;
            let selections = vec![
                (
                    format!("{} ({})", MenuCommand::MenuMode, app_settings.menu_mode,),
                    MenuMode::Simple,
                ),
                (MenuCommand::SetDefaults.to_string(), MenuMode::Simple),
                (MenuCommand::UpdateStoredData.to_string(), MenuMode::Simple),
                (MenuCommand::ListStoredData.to_string(), MenuMode::Simple),
                (MenuCommand::Init.to_string(), MenuMode::Simple),
                (MenuCommand::Reset.to_string(), MenuMode::Simple),
                (MenuCommand::Back.to_string(), MenuMode::Simple),
            ];
            (
                selections,
                "App Settings".to_string(),
                "Tinker with the settings behind the app".to_string(),
            )
        }
        MenuLevel::AppSettingsDefaults => {
            let selections = vec![
                (MenuCommand::SetDefaultProfile.to_string(), MenuMode::Simple),
                (MenuCommand::SetDefaultEngine.to_string(), MenuMode::Simple),
                (MenuCommand::SetDefaultIwad.to_string(), MenuMode::Simple),
                (MenuCommand::SetDefaultMapEditor.to_string(), MenuMode::Full),
                (MenuCommand::Back.to_string(), MenuMode::Simple),
            ];
            (
                selections,
                "App Settings / Defaults".to_string(),
                "Defaults to use for Profiles, Pick and Play, Map Editor, etc".to_string(),
            )
        }
        MenuLevel::AppSettingsList => {
            let selections = vec![
                (MenuCommand::ListEngines.to_string(), MenuMode::Simple),
                (MenuCommand::ListIwads.to_string(), MenuMode::Simple),
                (MenuCommand::ListPwads.to_string(), MenuMode::Simple),
                (MenuCommand::ListMapEditor.to_string(), MenuMode::Full),
                (MenuCommand::ListAppSettings.to_string(), MenuMode::Simple),
                (MenuCommand::Back.to_string(), MenuMode::Simple),
            ];
            (
                selections,
                "App Settings / List".to_string(),
                "List the data stored in the local Sqlite database".to_string(),
            )
        }
        MenuLevel::AppSettingsUpdate => {
            let selections = vec![
                (MenuCommand::UpdateEngines.to_string(), MenuMode::Simple),
                (MenuCommand::UpdateIwads.to_string(), MenuMode::Simple),
                (MenuCommand::UpdatePwads.to_string(), MenuMode::Simple),
                (MenuCommand::UpdateMapEditor.to_string(), MenuMode::Full),
                (MenuCommand::Back.to_string(), MenuMode::Simple),
            ];
            (
                selections,
                "App Settings / Update".to_string(),
                "Have fun! Don't break anything".to_string(),
            )
        }
    };

    let filtered_selections = selections
        .into_iter()
        .filter_map(|(cmd, cmd_mode)| {
            if cmd_mode == *menu_mode || menu_mode == &MenuMode::Full {
                Some(cmd)
            } else {
                None
            }
        })
        .collect();

    let final_help_message = format!(
        "↑↓ to move, enter to select, type to filter]\n[{}",
        help_message
    );
    let choice = inquire::Select::new(
        &format!("Select a {} option:", menu_name),
        filtered_selections,
    )
    .with_page_size(MENU_PAGE_SIZE)
    .with_help_message(&final_help_message)
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
        let app_settings = db::get_app_settings()?;
        info!("Welcome to {}", constants::APP_NAME.bright_green());
        //info!("({} menu mode)", app_settings.menu_mode);
        if let (MenuLevel::Main, MenuMode::Full) = (&menu_level, &app_settings.menu_mode) {
            info!("{}", "Profiles".bright_white());
            info!("  {}", menu_main::get_default_profile_text()?);
            info!("  {}", menu_main::get_last_profile_text()?);
        }

        let menu_command = menu_prompt(&app_settings.menu_mode, &menu_level)?;

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
        MenuCommand::PlayDefaultProfile => menu_main::play_default_profile(),
        MenuCommand::PlayLastProfile => menu_main::play_last_profile(),
        MenuCommand::PickAndPlayProfile => menu_main::pick_and_play_profile(),
        MenuCommand::PickAndPlayPwad => menu_main::pick_and_play_pwad(),
        MenuCommand::MapEditor => menu(MenuLevel::MapEditor),
        MenuCommand::Profiles => menu(MenuLevel::Profiles),
        MenuCommand::PlaySettings => menu(MenuLevel::GameSettings),
        MenuCommand::ViewMapReadme => menu(MenuLevel::ViewReadme),
        MenuCommand::AppSettings => menu(MenuLevel::AppSettings),

        // Profile Menu
        MenuCommand::NewProfile => menu_profiles::new_profile(),
        MenuCommand::EditProfile => menu_profiles::edit_profile(),
        MenuCommand::DeleteProfile => menu_profiles::delete_profile(),
        MenuCommand::SetDefaultProfile => menu_profiles::set_default_profile(),
        MenuCommand::ListProfile => menu_profiles::list_profiles(),

        // App Settings Menu
        MenuCommand::MenuMode => menu_app_settings::update_menu_mode(),
        MenuCommand::SetDefaults => menu(MenuLevel::AppSettingsDefaults),
        MenuCommand::SetDefaultEngine => menu_app_settings::set_default_engine(),
        MenuCommand::SetDefaultIwad => menu_app_settings::set_default_iwad(),
        MenuCommand::ListStoredData => menu(MenuLevel::AppSettingsList),
        MenuCommand::ListEngines => menu_app_settings::list_engines(),
        MenuCommand::ListIwads => menu_app_settings::list_iwads(),
        MenuCommand::ListPwads => menu_app_settings::list_pwads(),
        MenuCommand::ListAppSettings => menu_app_settings::list_app_settings(),
        MenuCommand::Init => menu_app_settings::init(),
        MenuCommand::UpdateStoredData => menu(MenuLevel::AppSettingsUpdate),
        MenuCommand::UpdateEngines => {
            let mut app_settings = db::get_app_settings()?;
            let folder = menu_app_settings::init_engines(
                &app_settings.exe_search_folder.unwrap_or("".to_string()),
            )?;
            app_settings.exe_search_folder = Some(folder);
            db::save_app_settings(app_settings)?;
            inquire::Text::new("Press any key to continue...").prompt_skippable()?;
            Ok("Successfully updated Engines".to_string())
        }
        MenuCommand::UpdateIwads => {
            let mut app_settings = db::get_app_settings()?;
            let folder = menu_app_settings::init_iwads(
                &app_settings.iwad_search_folder.unwrap_or("".to_string()),
            )?;
            app_settings.iwad_search_folder = Some(folder);
            db::save_app_settings(app_settings)?;
            inquire::Text::new("Press any key to continue...").prompt_skippable()?;
            Ok("Successfully updated IWADs".to_string())
        }
        MenuCommand::UpdatePwads => {
            let mut app_settings = db::get_app_settings()?;
            let folder = menu_app_settings::init_pwads(
                &app_settings.pwad_search_folder.unwrap_or("".to_string()),
            )?;
            app_settings.pwad_search_folder = Some(folder);
            db::save_app_settings(app_settings)?;
            inquire::Text::new("Press any key to continue...").prompt_skippable()?;
            Ok("Successfully updated PWADs".to_string())
        }
        MenuCommand::Reset => menu_app_settings::reset(false),

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
        MenuCommand::OpenFromDefaultProfile => menu_map_editor::open_from_default_profile(),
        MenuCommand::OpenFromLastProfile => menu_map_editor::open_from_last_profile(),
        MenuCommand::OpenFromPickProfile => menu_map_editor::open_from_pick_profile(),
        MenuCommand::OpenFromPickPwad => menu_map_editor::open_from_pick_pwad(),
        MenuCommand::SetDefaultMapEditor => menu_map_editor::set_default_map_editor(),
        MenuCommand::ListMapEditor => menu_map_editor::list_map_editors(),
        MenuCommand::UpdateMapEditor => menu_map_editor::update_map_editors(),
        MenuCommand::DeleteMapEditor => menu_map_editor::delete_map_editor(),

        // View Readme Menu
        MenuCommand::ViewFromDefaultProfile => menu_view_readme::view_from_default_profile(),
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
        ARG_PLAY => MenuCommand::PlayDefaultProfile,
        ARG_PLAY_LAST => MenuCommand::PlayLastProfile,
        ARG_MAP_EDITOR => MenuCommand::OpenFromDefaultProfile,
        ARG_MAP_EDITOR_LAST => MenuCommand::OpenFromLastProfile,
        _ => MenuCommand::Ignore,
    }
}
