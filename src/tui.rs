use std::str::FromStr;

use clap::ValueEnum;
use inquire::InquireError;
use log::debug;
use log::info;
use owo_colors::colors::xterm;
use owo_colors::OwoColorize;
use strum_macros::Display;
use strum_macros::EnumString;

use crate::constants;
use crate::data;
use crate::db;
use crate::menu_app_settings;
use crate::menu_editor;
use crate::menu_main;
use crate::menu_maps;
use crate::menu_play_settings;
use crate::menu_profiles;

pub const MENU_PAGE_SIZE: usize = 15;
pub const MENU_CLR: &str = "clr";
pub const MENU_CLR_MESSAGE: &str = "clr to clear";

pub enum MenuLevel {
    Main,
    Profiles,
    GameSettings,
    MapEditor,
    Maps,
    MapsSearchDoomworld,
    MapsReadme,
    AppSettings,
    AppSettingsDefaults,
    AppSettingsList,
    AppSettingsUpdate,
    AppSettingsDelete,
}

#[derive(Clone, Debug, PartialEq, EnumString, Display, sqlx::Type, ValueEnum)]
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
    #[strum(serialize = "Pick & Play Map")]
    PickAndPlayMap,
    #[strum(serialize = "Editor >>")]
    Editor,
    #[strum(serialize = "Profiles >>")]
    Profiles,
    #[strum(serialize = "Play Settings >>")]
    PlaySettings,
    #[strum(serialize = "Maps >>")]
    Maps,
    #[strum(serialize = "App Settings >>")]
    AppSettings,

    // Profile Menu
    #[strum(serialize = "New Profile")]
    NewProfile,
    #[strum(serialize = "Edit Profile")]
    EditProfile,
    #[strum(serialize = "Change Engine on Profile")]
    ChangeEngineOnProfile,
    #[strum(serialize = "Delete Profile")]
    DeleteProfile,
    #[strum(serialize = "Set Default Profile")]
    SetDefaultProfile,
    #[strum(serialize = "List Profiles")]
    ListProfile,

    // App Settings Menu
    #[strum(serialize = "Menu Mode")]
    MenuMode,
    #[strum(serialize = "Use Doomworld API")]
    UseDoomworldApi,
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
    #[strum(serialize = "List Maps")]
    ListMaps,
    #[strum(serialize = "List App Settings")]
    ListAppSettings,
    #[strum(serialize = "List Play Settings")]
    ListPlaySettings,
    #[strum(serialize = "Update Stored Data >>")]
    UpdateStoredData,
    #[strum(serialize = "Update Engines")]
    UpdateEngines,
    #[strum(serialize = "Update Internal WADs")]
    UpdateIwads,
    #[strum(serialize = "Update Maps")]
    UpdateMaps,
    #[strum(serialize = "Update Map Info")]
    UpdateMapInfo,
    #[strum(serialize = "Delete Stored Data >>")]
    DeleteStoredData,
    #[strum(serialize = "Delete Engines")]
    DeleteEngines,
    #[strum(serialize = "Delete Internal WADs")]
    DeleteIwads,
    #[strum(serialize = "Delete Maps")]
    DeleteMaps,
    #[strum(serialize = "View App Version")]
    AppVersion,
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

    // Editor Menu
    #[strum(serialize = "Open from Default Profile Map")]
    OpenFromDefaultProfile,
    #[strum(serialize = "Open from Last Profile Map")]
    OpenFromLastProfile,
    #[strum(serialize = "Open from Pick Profile")]
    OpenFromPickProfile,
    #[strum(serialize = "Open from Pick Map")]
    OpenFromPickMap,
    #[strum(serialize = "Set Default Editor")]
    SetDefaultEditor,
    #[strum(serialize = "List Editors")]
    ListEditors,
    #[strum(serialize = "Add Editor")]
    AddEditor,
    #[strum(serialize = "Delete Editor")]
    DeleteEditor,

    // Map Menu
    #[strum(serialize = "View on Doomworld")]
    ViewOnDoomworld,
    #[strum(serialize = "Search & Download on Doomworld >>")]
    SearchAndDownloadOnDoomworld,
    #[strum(serialize = "Search Doomworld by Author")]
    SearchDoomworldByAuthor,
    #[strum(serialize = "Search Doomworld by FileName")]
    SearchDoomworldByFileName,
    #[strum(serialize = "Search Doomworld by Map Title")]
    SearchDoomworldByMapTitle,
    #[strum(serialize = "View Readme >>")]
    ViewReadme,
    #[strum(serialize = "Readme from Default Profile")]
    ReadmeFromDefaultProfile,
    #[strum(serialize = "Readme from Last Profile")]
    ReadmeFromLastProfile,
    #[strum(serialize = "Readme from Pick Profile")]
    ReadmeFromPickProfile,
    #[strum(serialize = "Readme from Pick Map")]
    ReadmeFromPickMap,

    // Back and Quit
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
                (MenuCommand::PickAndPlayMap.to_string(), MenuMode::Simple),
                (
                    MenuCommand::PickAndPlayProfile.to_string(),
                    MenuMode::Simple,
                ),
                (MenuCommand::PlaySettings.to_string(), MenuMode::Simple),
                (MenuCommand::Profiles.to_string(), MenuMode::Simple),
                (MenuCommand::Maps.to_string(), MenuMode::Simple),
                (MenuCommand::Editor.to_string(), MenuMode::Full),
                (MenuCommand::AppSettings.to_string(), MenuMode::Simple),
                (MenuCommand::Quit.to_string(), MenuMode::Simple),
            ];
            (
                selections,
                "Main Menu".to_string(),
                "Let's play Doom!"
                    .fg::<xterm::DarkSpringGreen>()
                    .to_string(),
            )
        }
        MenuLevel::Profiles => {
            let selections = vec![
                (MenuCommand::NewProfile.to_string(), MenuMode::Simple),
                (MenuCommand::EditProfile.to_string(), MenuMode::Simple),
                (
                    MenuCommand::ChangeEngineOnProfile.to_string(),
                    MenuMode::Simple,
                ),
                (MenuCommand::SetDefaultProfile.to_string(), MenuMode::Simple),
                (MenuCommand::ListProfile.to_string(), MenuMode::Simple),
                (MenuCommand::DeleteProfile.to_string(), MenuMode::Simple),
                (MenuCommand::Back.to_string(), MenuMode::Simple),
            ];
            (
                selections,
                "Profile".to_string(),
                "Profiles group together Engines, IWADs and Maps for quick play".to_string(),
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
                (MenuCommand::OpenFromPickMap.to_string(), MenuMode::Full),
                (MenuCommand::SetDefaultEditor.to_string(), MenuMode::Full),
                (MenuCommand::AddEditor.to_string(), MenuMode::Full),
                (MenuCommand::DeleteEditor.to_string(), MenuMode::Full),
                (MenuCommand::ListEditors.to_string(), MenuMode::Full),
                (MenuCommand::Back.to_string(), MenuMode::Simple),
            ];
            (
                selections,
                "Editor".to_string(),
                "Quickly view/edit a Map in a Editor".to_string(),
            )
        }
        MenuLevel::Maps => {
            let selections = vec![
                (MenuCommand::ViewOnDoomworld.to_string(), MenuMode::Simple),
                (
                    MenuCommand::SearchAndDownloadOnDoomworld.to_string(),
                    MenuMode::Simple,
                ),
                (MenuCommand::ViewReadme.to_string(), MenuMode::Simple),
                (MenuCommand::UpdateMapInfo.to_string(), MenuMode::Simple),
                (MenuCommand::ListMaps.to_string(), MenuMode::Simple),
                (MenuCommand::UpdateMaps.to_string(), MenuMode::Simple),
                (MenuCommand::Back.to_string(), MenuMode::Simple),
            ];
            (
                selections,
                "Maps".to_string(),
                "Manage Map related data and options".to_string(),
            )
        }
        MenuLevel::MapsSearchDoomworld => {
            let selections = vec![
                (
                    MenuCommand::SearchDoomworldByAuthor.to_string(),
                    MenuMode::Simple,
                ),
                (
                    MenuCommand::SearchDoomworldByFileName.to_string(),
                    MenuMode::Simple,
                ),
                (
                    MenuCommand::SearchDoomworldByMapTitle.to_string(),
                    MenuMode::Simple,
                ),
                (MenuCommand::Back.to_string(), MenuMode::Simple),
            ];
            (
                selections,
                "Maps / Search & Download".to_string(),
                "Search for Maps, Download 'em!".to_string(),
            )
        }
        MenuLevel::MapsReadme => {
            let selections = vec![
                (
                    MenuCommand::ReadmeFromDefaultProfile.to_string(),
                    MenuMode::Simple,
                ),
                (
                    MenuCommand::ReadmeFromLastProfile.to_string(),
                    MenuMode::Full,
                ),
                (
                    MenuCommand::ReadmeFromPickProfile.to_string(),
                    MenuMode::Simple,
                ),
                (MenuCommand::ReadmeFromPickMap.to_string(), MenuMode::Simple),
                (MenuCommand::Back.to_string(), MenuMode::Simple),
            ];
            (
                selections,
                "Maps / View Readme".to_string(),
                "View local Map Readmes".to_string(),
            )
        }
        MenuLevel::AppSettings => {
            let app_settings = db::get_app_settings()?;
            let selections = vec![
                (
                    format!("{} ({})", MenuCommand::MenuMode, app_settings.menu_mode,),
                    MenuMode::Simple,
                ),
                (
                    format!(
                        "{} ({})",
                        MenuCommand::UseDoomworldApi,
                        app_settings.use_doomworld_api,
                    ),
                    MenuMode::Simple,
                ),
                (MenuCommand::SetDefaults.to_string(), MenuMode::Simple),
                (MenuCommand::UpdateStoredData.to_string(), MenuMode::Simple),
                (MenuCommand::ListStoredData.to_string(), MenuMode::Simple),
                (MenuCommand::DeleteStoredData.to_string(), MenuMode::Simple),
                (MenuCommand::Init.to_string(), MenuMode::Simple),
                (MenuCommand::Reset.to_string(), MenuMode::Simple),
                (MenuCommand::AppVersion.to_string(), MenuMode::Simple),
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
                (MenuCommand::SetDefaultEditor.to_string(), MenuMode::Full),
                (MenuCommand::Back.to_string(), MenuMode::Simple),
            ];
            (
                selections,
                "App Settings / Defaults".to_string(),
                "Defaults to use for Profiles, Pick and Play, Editor, etc".to_string(),
            )
        }
        MenuLevel::AppSettingsList => {
            let selections = vec![
                (MenuCommand::ListEngines.to_string(), MenuMode::Simple),
                (MenuCommand::ListIwads.to_string(), MenuMode::Simple),
                (MenuCommand::ListMaps.to_string(), MenuMode::Simple),
                (MenuCommand::ListProfile.to_string(), MenuMode::Simple),
                (MenuCommand::ListEditors.to_string(), MenuMode::Full),
                (MenuCommand::ListAppSettings.to_string(), MenuMode::Simple),
                (MenuCommand::ListPlaySettings.to_string(), MenuMode::Simple),
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
                (MenuCommand::UpdateMaps.to_string(), MenuMode::Simple),
                (MenuCommand::UpdateMapInfo.to_string(), MenuMode::Simple),
                (MenuCommand::AddEditor.to_string(), MenuMode::Full),
                (MenuCommand::Back.to_string(), MenuMode::Simple),
            ];
            (
                selections,
                "App Settings / Update".to_string(),
                "Have fun! Don't break anything".to_string(),
            )
        }
        MenuLevel::AppSettingsDelete => {
            let selections = vec![
                (MenuCommand::DeleteEngines.to_string(), MenuMode::Simple),
                (MenuCommand::DeleteIwads.to_string(), MenuMode::Simple),
                (MenuCommand::DeleteMaps.to_string(), MenuMode::Simple),
                (MenuCommand::DeleteEditor.to_string(), MenuMode::Full),
                (MenuCommand::Back.to_string(), MenuMode::Simple),
            ];
            (
                selections,
                "App Settings / Delete".to_string(),
                "If you really want to delete everything, use Reset".to_string(),
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
        info!(
            "{} {}",
            "Welcome to".bold(),
            constants::APP_NAME.fg::<xterm::DarkSpringGreen>().bold()
        );
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
            Ok(result) => {
                if !result.is_empty() {
                    info!("{}", colour_result(&result))
                }
            }
            Err(e) => match e.downcast_ref::<InquireError>() {
                Some(InquireError::OperationCanceled) => {
                    if e.to_string() == "Operation was canceled by the user" {
                        info!("{}", "Option canceled".yellow())
                    } else {
                        info!("{}", e.yellow())
                    }
                }
                _ => {
                    info!("Error: {}", e.red());
                    debug!("Error: {:?}", e);
                }
            },
        }
    }
}

pub fn run_menu_command(menu_command: MenuCommand) -> Result<String, eyre::Report> {
    run_menu_command_with_force(menu_command, false)
}

pub fn run_menu_command_with_force(
    menu_command: MenuCommand,
    force: bool,
) -> Result<String, eyre::Report> {
    match menu_command {
        // Main Menu
        MenuCommand::PlayDefaultProfile => menu_main::play_default_profile(),
        MenuCommand::PlayLastProfile => menu_main::play_last_profile(),
        MenuCommand::PickAndPlayProfile => menu_main::pick_and_play_profile(),
        MenuCommand::PickAndPlayMap => menu_main::pick_and_play_map(),
        MenuCommand::Editor => menu(MenuLevel::MapEditor),
        MenuCommand::Profiles => menu(MenuLevel::Profiles),
        MenuCommand::PlaySettings => menu(MenuLevel::GameSettings),
        MenuCommand::Maps => menu(MenuLevel::Maps),
        MenuCommand::AppSettings => menu(MenuLevel::AppSettings),

        // Profile Menu
        MenuCommand::NewProfile => menu_profiles::add_profile(),
        MenuCommand::EditProfile => menu_profiles::edit_profile(),
        MenuCommand::ChangeEngineOnProfile => menu_profiles::change_engine_on_profile(),
        MenuCommand::DeleteProfile => menu_profiles::delete_profile(),
        MenuCommand::SetDefaultProfile => menu_profiles::set_default_profile(),
        MenuCommand::ListProfile => menu_profiles::list_profiles(data::ListType::Summary),

        // App Settings Menu
        MenuCommand::MenuMode => menu_app_settings::update_menu_mode(),
        MenuCommand::UseDoomworldApi => menu_app_settings::update_use_doomworld_api(),
        MenuCommand::SetDefaults => menu(MenuLevel::AppSettingsDefaults),
        MenuCommand::SetDefaultEngine => menu_app_settings::set_default_engine(),
        MenuCommand::SetDefaultIwad => menu_app_settings::set_default_iwad(),
        MenuCommand::ListStoredData => menu(MenuLevel::AppSettingsList),
        MenuCommand::ListEngines => menu_app_settings::list_engines(),
        MenuCommand::ListIwads => menu_app_settings::list_iwads(),
        MenuCommand::ListMaps => menu_app_settings::list_maps(),
        MenuCommand::ListAppSettings => menu_app_settings::list_app_settings(),
        MenuCommand::ListPlaySettings => menu_play_settings::list_play_settings(),
        MenuCommand::Init => menu_app_settings::init(),
        MenuCommand::UpdateStoredData => menu(MenuLevel::AppSettingsUpdate),
        MenuCommand::UpdateEngines => menu_app_settings::update_engines(),
        MenuCommand::UpdateIwads => menu_app_settings::update_iwads(),
        MenuCommand::UpdateMaps => menu_app_settings::update_maps(),
        MenuCommand::UpdateMapInfo => menu_app_settings::update_map_info(),
        MenuCommand::DeleteStoredData => menu(MenuLevel::AppSettingsDelete),
        MenuCommand::DeleteEngines => menu_app_settings::delete_engines(),
        MenuCommand::DeleteIwads => menu_app_settings::delete_iwads(),
        MenuCommand::DeleteMaps => menu_app_settings::delete_maps(),
        MenuCommand::AppVersion => menu_app_settings::app_version(),
        MenuCommand::Reset => menu_app_settings::reset(force),

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
        MenuCommand::Width => menu_play_settings::update_screen_width(),
        MenuCommand::Height => menu_play_settings::update_screen_height(),
        MenuCommand::FullScreen => menu_play_settings::update_full_screen(),
        MenuCommand::Windowed => menu_play_settings::update_windowed(),
        MenuCommand::AdditionalArguments => menu_play_settings::update_additional_arguments(),
        MenuCommand::ResetPlaySettings => menu_play_settings::reset_play_settings(force),

        // Editor Menu
        MenuCommand::OpenFromDefaultProfile => menu_editor::open_from_default_profile(),
        MenuCommand::OpenFromLastProfile => menu_editor::open_from_last_profile(),
        MenuCommand::OpenFromPickProfile => menu_editor::open_from_pick_profile(),
        MenuCommand::OpenFromPickMap => menu_editor::open_from_pick_map(),
        MenuCommand::SetDefaultEditor => menu_editor::set_default_editor(),
        MenuCommand::ListEditors => menu_editor::list_editors(),
        MenuCommand::AddEditor => menu_editor::add_editor(),
        MenuCommand::DeleteEditor => menu_editor::delete_editor(),

        // Map Menu
        MenuCommand::ViewOnDoomworld => menu_maps::view_on_doomworld(),
        MenuCommand::SearchAndDownloadOnDoomworld => menu(MenuLevel::MapsSearchDoomworld),
        MenuCommand::ViewReadme => menu(MenuLevel::MapsReadme),

        // Map Search on Doomworld Menu
        MenuCommand::SearchDoomworldByAuthor => menu_maps::search_doomworld_by_author(),
        MenuCommand::SearchDoomworldByFileName => menu_maps::search_doomworld_by_filename(),
        MenuCommand::SearchDoomworldByMapTitle => menu_maps::search_doomworld_by_map_title(),

        // Map View Readme Menu
        MenuCommand::ReadmeFromDefaultProfile => menu_maps::view_from_default_profile(),
        MenuCommand::ReadmeFromLastProfile => menu_maps::view_from_last_profile(),
        MenuCommand::ReadmeFromPickProfile => menu_maps::view_from_pick_profile(),
        MenuCommand::ReadmeFromPickMap => menu_maps::view_from_pick_map(),

        // Back and Quit
        MenuCommand::Ignore => Ok("".to_string()),
        MenuCommand::Back => Ok("".to_string()),
        MenuCommand::Quit => Ok("Quitting".to_string()),
    }
}

pub fn colour_result(result: &str) -> String {
    // Yeah this is horrible
    if result.starts_with("Successfully") {
        result.green().to_string()
    } else if result.starts_with("Canceled")
        || result.starts_with("Cannot")
        || result.starts_with("No ")
    {
        result.yellow().to_string()
    } else if result.starts_with("There are no") {
        result.red().to_string()
    } else {
        result.to_string()
    }
}
