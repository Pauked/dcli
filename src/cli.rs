use clap::{Parser, ValueEnum};
use log::debug;

use crate::{
    constants, data, menu_app_settings, menu_editor, menu_main, menu_play_settings, menu_profiles,
    menu_queues, paths,
    tui::{self, MenuCommand},
};

pub enum CliRunMode {
    Tui,
    Quit,
}

#[derive(Parser, Debug, PartialEq)]
#[command(name = constants::CRATE_NAME)]
#[command(author = constants::CRATE_AUTHORS)]
#[command(version = constants::CRATE_VERSION)]
#[command(
    help_template = "{about-section}Version : {version}\nAuthor  : {author} \n\n{usage-heading} {usage} \n\n{all-args} {tab}"
)]
#[command(about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub action: Option<Action>,
}

/// Doom Command Line Interface!
#[derive(Parser, Debug, PartialEq)]
pub enum Action {
    /// Play Doom with the Default Profile
    #[clap(short_flag = 'p')]
    Play,

    /// Play Doom with the Last Run Profile
    PlayLast,

    /// Play Doom with the specified Profile
    PlayProfile {
        /// Profile name
        profile_name: String,
    },

    /// Open the Editor with the Default Profile. Takes the first Map in Profile
    Editor,

    /// Open the Editor with the Last Run Profile. Takes the first Map in Profile
    EditorLast,

    /// Initializes the app for use. Asks a quick set of questions to get you Dooming!
    #[clap(short_flag = 'i')]
    Init {
        /// Engine path
        engine_path: String,
        /// IWAD path
        iwad_path: String,
        /// Map paths
        map_path: Option<String>,
        /// Force initialization and skip any entry and selection prompts
        #[arg(long, default_value = "false")]
        force: bool,
    },

    /// Resets the database, nuking all settings
    #[clap(short_flag = 'r')]
    Reset {
        /// Force database reset and skip confirmation prompt
        #[arg(long, default_value = "false")]
        force: bool,
    },

    /// List out selected data from the database
    #[clap(short_flag = 'l')]
    List {
        /// What data to list
        #[clap(value_enum)]
        list_data: ListData,

        /// Show full details. Does not apply to all data types
        #[arg(long, default_value = "false")]
        full: bool,
    },

    /// Add a new Profile to combine Engine, IWAD, and Maps
    AddProfile {
        /// Profile name
        name: String,

        /// Engine path
        engine: String,

        /// IWAD path
        iwad: String,

        /// Map paths. Up to five
        #[clap(long, value_delimiter = ',')]
        maps: Option<Vec<String>>,

        /// Save game file to automatically load
        #[arg(long)]
        save_game: Option<String>,

        /// Additional arguments to pass to the engine
        #[arg(long)]
        args: Option<Vec<String>>,
    },

    /// Delete a Profile. Sad times
    DeleteProfile {
        /// Profile name
        name: String,

        /// Force profile delete and skip confirmation prompt
        #[arg(long, default_value = "false")]
        force: bool,
    },

    /// Add a new Editor to view and edit maps
    AddEditor {
        /// Editor path
        path: String,

        /// Load file argument to pass to the Editor
        #[arg(long)]
        load_file_arg: Option<String>,

        /// Additional arguments to pass to the Editor
        #[arg(long)]
        additional_args: Option<Vec<String>>,
    },

    /// Delete an Editor
    DeleteEditor {
        /// Editor path
        path: String,

        /// Force editor delete and skip confirmation prompt
        #[arg(long, default_value = "false")]
        force: bool,
    },

    /// Set App Settings
    SetAppSettings {
        /// Menu mode
        #[clap(value_enum, long)]
        menu_mode: Option<tui::MenuMode>,
        // Use Doomworld API
        #[clap(value_enum, long)]
        use_doomworld_api: Option<bool>,
    },

    /// Set Defaults for Engine, IWAD, Profile, and Editor
    SetDefault {
        /// Engine path
        #[arg(long)]
        engine: Option<String>,

        /// IWAD path
        #[arg(long)]
        iwad: Option<String>,

        /// Profile name
        #[arg(long)]
        profile: Option<String>,

        /// Editor path
        #[arg(long)]
        editor: Option<String>,
    },

    /// Set Play Settings to control how Doom is played
    SetPlaySettings {
        /// Reset All Play Settings to Defaults
        #[clap(value_enum, long)]
        reset: Option<bool>,

        /// Compatibility Level
        #[clap(value_enum, long)]
        comp_level: Option<data::CompLevel>,

        /// Config File. Enter "clr" to blank the config file
        #[clap(value_enum, long)]
        config_file: Option<String>,

        /// Fast Monsters
        #[clap(value_enum, long)]
        fast_monsters: Option<bool>,

        /// No Monsters
        #[clap(value_enum, long)]
        no_monsters: Option<bool>,

        /// Respawn Monsters
        #[clap(value_enum, long)]
        respawn_monsters: Option<bool>,

        /// Warp to Level. Enter "clr" to blank the config file
        #[clap(value_enum, long)]
        warp_to_level: Option<String>,

        /// Skill
        #[clap(value_enum, long)]
        skill: Option<u8>,

        /// Turbo
        #[clap(value_enum, long)]
        turbo: Option<u8>,

        /// Timer
        #[clap(value_enum, long)]
        timer: Option<u32>,

        /// Screen Width
        #[clap(value_enum, long)]
        screen_width: Option<u32>,

        /// Screen Height
        #[clap(value_enum, long)]
        screen_height: Option<u32>,

        /// Full Screen
        #[clap(value_enum, long)]
        full_screen: Option<bool>,

        /// Windowed
        #[clap(value_enum, long)]
        windowed: Option<bool>,

        /// Additional Arguments
        #[clap(value_enum, long)]
        additional_args: Option<Vec<String>>,
    },
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum ListData {
    Engines,
    Iwads,
    Maps,
    Profiles,
    Queues,
    Editors,
    AppSettings,
    PlaySettings,
}

pub fn run_cli_action(args: Args) -> Result<(String, CliRunMode), eyre::Report> {
    if let Some(action) = args.action {
        // If we are not resetting the database, make sure it exists and is ready to use
        match action {
            Action::Reset { .. } => {}
            Action::Init {
                engine_path: _,
                iwad_path: _,
                map_path: _,
                force,
            } => {
                menu_app_settings::check_app_can_run(force)?;
            }
            _ => {
                menu_app_settings::check_app_can_run(false)?;
            }
        }

        match action {
            Action::Play => Ok((
                tui::run_menu_command(MenuCommand::PlayDefaultProfile)?,
                CliRunMode::Quit,
            )),
            Action::PlayLast => Ok((
                tui::run_menu_command(MenuCommand::PlayLastProfile)?,
                CliRunMode::Quit,
            )),
            Action::PlayProfile { profile_name } => Ok((
                menu_main::cli_play_selected_profile(&profile_name)?,
                CliRunMode::Quit,
            )),
            Action::Editor => Ok((
                tui::run_menu_command(MenuCommand::OpenFromDefaultProfile)?,
                CliRunMode::Quit,
            )),
            Action::EditorLast => Ok((
                tui::run_menu_command(MenuCommand::OpenFromDefaultProfile)?,
                CliRunMode::Quit,
            )),
            Action::Init {
                engine_path,
                iwad_path,
                map_path,
                force,
            } => {
                debug!(
                    "Init engine_path '{}', iwad_path '{}', map_path '{:?}', force '{}'",
                    engine_path, iwad_path, map_path, force
                );
                Ok((
                    menu_app_settings::cli_init(
                        &paths::resolve_path(&engine_path),
                        &paths::resolve_path(&iwad_path),
                        paths::resolve_path_opt(map_path),
                        force,
                    )?,
                    CliRunMode::Quit,
                ))
            }
            Action::Reset { force } => {
                let result = tui::run_menu_command_with_force(MenuCommand::Reset, force)?;
                if force {
                    Ok((result, CliRunMode::Quit))
                } else if result != *"Database reset not confirmed" {
                    Ok((tui::run_menu_command(MenuCommand::Init)?, CliRunMode::Tui))
                } else {
                    Ok((result, CliRunMode::Tui))
                }
            }
            Action::List { list_data, full } => {
                let list_type = match full {
                    true => data::ListType::Full,
                    false => data::ListType::Summary,
                };
                let result = match list_data {
                    ListData::Engines => menu_app_settings::list_engines(),
                    ListData::Iwads => menu_app_settings::list_iwads(),
                    ListData::Maps => menu_app_settings::list_maps(),
                    ListData::Profiles => menu_profiles::list_profiles(list_type),
                    ListData::Queues => menu_queues::list_queues(),
                    ListData::Editors => menu_editor::list_editors(),
                    ListData::AppSettings => menu_app_settings::list_app_settings(),
                    ListData::PlaySettings => menu_play_settings::list_play_settings(),
                }?;
                Ok((result, CliRunMode::Quit))
            }
            Action::AddProfile {
                name,
                engine,
                iwad,
                maps,
                save_game,
                args,
            } => {
                debug!(
                    "AddProfile: name '{}', engine '{}', iwad '{}', maps '{:?}', save_game '{:?}', args '{:?}'",
                    name, engine, iwad, maps, save_game, args
                );
                Ok((
                    menu_profiles::cli_add_profile(
                        &name,
                        &paths::resolve_path(&engine),
                        &paths::resolve_path(&iwad),
                        maps,
                        save_game,
                        args,
                    )?,
                    CliRunMode::Quit,
                ))
            }
            Action::DeleteProfile { name, force } => Ok((
                menu_profiles::cli_delete_profile(&name, force)?,
                CliRunMode::Quit,
            )),
            Action::AddEditor {
                path,
                load_file_arg: load_file_argument,
                additional_args: additional_arguments,
            } => Ok((
                menu_editor::cli_add_editor(
                    &paths::resolve_path(&path),
                    load_file_argument,
                    additional_arguments,
                )?,
                CliRunMode::Quit,
            )),
            Action::DeleteEditor { path, force } => Ok((
                menu_editor::cli_delete_editor(&paths::resolve_path(&path), force)?,
                CliRunMode::Quit,
            )),
            Action::SetAppSettings {
                menu_mode,
                use_doomworld_api,
            } => {
                if let Some(menu_mode) = menu_mode {
                    Ok((
                        menu_app_settings::cli_update_menu_mode(menu_mode)?,
                        CliRunMode::Quit,
                    ))
                } else if let Some(use_doomworld_api) = use_doomworld_api {
                    Ok((
                        menu_app_settings::cli_update_use_doomworld_api(use_doomworld_api)?,
                        CliRunMode::Quit,
                    ))
                } else {
                    Ok(("No arguments specified".to_string(), CliRunMode::Quit))
                }
            }
            Action::SetDefault {
                engine,
                iwad,
                profile,
                editor,
            } => {
                if let Some(engine) = engine {
                    Ok((
                        menu_app_settings::cli_set_default_engine(&paths::resolve_path(&engine))?,
                        CliRunMode::Quit,
                    ))
                } else if let Some(iwad) = iwad {
                    Ok((
                        menu_app_settings::cli_set_default_iwad(&paths::resolve_path(&iwad))?,
                        CliRunMode::Quit,
                    ))
                } else if let Some(profile) = profile {
                    Ok((
                        menu_profiles::cli_set_default_profile(&profile)?,
                        CliRunMode::Quit,
                    ))
                } else if let Some(editor) = editor {
                    Ok((
                        menu_editor::cli_set_default_editor(&paths::resolve_path(&editor))?,
                        CliRunMode::Quit,
                    ))
                } else {
                    Ok(("No arguments specified".to_string(), CliRunMode::Quit))
                }
            }
            Action::SetPlaySettings {
                reset,
                comp_level,
                config_file,
                fast_monsters,
                no_monsters,
                respawn_monsters,
                warp_to_level,
                skill,
                turbo,
                timer,
                screen_width,
                screen_height,
                full_screen,
                windowed,
                additional_args,
            } => {
                if let Some(reset) = reset {
                    Ok((
                        menu_play_settings::reset_play_settings(reset)?,
                        CliRunMode::Quit,
                    ))
                } else if let Some(comp_level) = comp_level {
                    Ok((
                        menu_play_settings::cli_set_comp_level(comp_level)?,
                        CliRunMode::Quit,
                    ))
                } else if let Some(config_file) = config_file {
                    Ok((
                        menu_play_settings::cli_set_config_file(&paths::resolve_path(
                            &config_file,
                        ))?,
                        CliRunMode::Quit,
                    ))
                } else if let Some(fast_monsters) = fast_monsters {
                    Ok((
                        menu_play_settings::cli_set_fast_monsters(fast_monsters)?,
                        CliRunMode::Quit,
                    ))
                } else if let Some(no_monsters) = no_monsters {
                    Ok((
                        menu_play_settings::cli_set_no_monsters(no_monsters)?,
                        CliRunMode::Quit,
                    ))
                } else if let Some(respawn_monsters) = respawn_monsters {
                    Ok((
                        menu_play_settings::cli_set_respawn_monsters(respawn_monsters)?,
                        CliRunMode::Quit,
                    ))
                } else if let Some(warp_to_level) = warp_to_level {
                    Ok((
                        menu_play_settings::cli_set_warp_to_level(&warp_to_level)?,
                        CliRunMode::Quit,
                    ))
                } else if let Some(skill) = skill {
                    Ok((menu_play_settings::cli_set_skill(skill)?, CliRunMode::Quit))
                } else if let Some(turbo) = turbo {
                    Ok((menu_play_settings::cli_set_turbo(turbo)?, CliRunMode::Quit))
                } else if let Some(timer) = timer {
                    Ok((menu_play_settings::cli_set_timer(timer)?, CliRunMode::Quit))
                } else if let Some(screen_width) = screen_width {
                    Ok((
                        menu_play_settings::cli_set_screen_width(screen_width)?,
                        CliRunMode::Quit,
                    ))
                } else if let Some(screen_height) = screen_height {
                    Ok((
                        menu_play_settings::cli_set_screen_height(screen_height)?,
                        CliRunMode::Quit,
                    ))
                } else if let Some(full_screen) = full_screen {
                    Ok((
                        menu_play_settings::cli_set_full_screen(full_screen)?,
                        CliRunMode::Quit,
                    ))
                } else if let Some(windowed) = windowed {
                    Ok((
                        menu_play_settings::cli_set_windowed(windowed)?,
                        CliRunMode::Quit,
                    ))
                } else if let Some(additional_args) = additional_args {
                    Ok((
                        menu_play_settings::cli_set_additional_args(additional_args)?,
                        CliRunMode::Quit,
                    ))
                } else {
                    Ok(("No arguments specified".to_string(), CliRunMode::Quit))
                }
            }
        }
    } else {
        Ok((
            "No arguments specified, continue to UI".to_string(),
            CliRunMode::Tui,
        ))
    }
}
