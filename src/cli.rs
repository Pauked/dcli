use clap::{Parser, ValueEnum};
use log::{debug, info};

use crate::{
    constants, data, menu_app_settings, menu_editor, menu_profiles,
    tui::{self, MenuCommand},
};

pub enum CliRunMode {
    Tui,
    Quit,
}

#[derive(Parser, Debug, PartialEq)]
#[clap(about, author, name = constants::CRATE_NAME, version)]
pub struct Args {
    #[command(subcommand)]
    pub action: Option<Action>,
}

#[derive(Parser, Debug, PartialEq)]
pub enum Action {
    /// Play Doom with the Default Profile.
    #[clap(short_flag = 'p')]
    Play,

    /// Play Doom with the Last Run Profile.
    PlayLast,

    /// Open the Editor with the Default Profile. Takes the first Map in Profile.
    #[clap(short_flag = 'm')]
    Editor,

    /// Open the Editor with the Last Run Profile. Takes the first Map in Profile.
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
        /// Force initialization and skip any entry and selection prompts.
        #[arg(long, default_value = "false")]
        force: bool,
    },

    /// Resets the database, nuking all settings.
    #[clap(short_flag = 'r')]
    Reset {
        /// Force database reset and skip confirmation prompt.
        #[arg(long, default_value = "false")]
        force: bool,
    },

    /// List out selected data from the database.
    #[clap(short_flag = 'l')]
    List {
        /// What data to list.
        #[clap(value_enum)]
        list_data: ListData,

        /// Show full details. Does not apply to all data types.
        #[arg(long, default_value = "false")]
        full: bool,
    },

    #[clap(short_flag = 'a')]
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

        /// Additional arguments to pass to the engine
        #[arg(long)]
        args: Option<Vec<String>>,
    },
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum ListData {
    Engines,
    Iwads,
    Maps,
    Profiles,
    Editors,
    AppSettings,
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
                let result = menu_app_settings::cli_init(engine_path, iwad_path, map_path, force)?;
                Ok((result, CliRunMode::Quit))
                // Ok((tui::run_menu_command(MenuCommand::Init)?, CliRunMode::Tui));
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
                    ListData::Editors => menu_editor::list_editors(),
                    ListData::AppSettings => menu_app_settings::list_app_settings(),
                }?;
                Ok((result, CliRunMode::Quit))
            }
            Action::AddProfile {
                name,
                engine,
                iwad,
                maps,
                args,
            } => {
                info!(
                    "AddProfile: name '{}', engine '{}', iwad '{}', maps '{:?}', args '{:?}'",
                    name, engine, iwad, maps, args
                );
                let result = menu_profiles::cli_new_profile(&name, &engine, &iwad, maps, args)?;
                Ok((result, CliRunMode::Quit))
            }
        }
    } else {
        Ok((
            "No arguments specified, continue to UI".to_string(),
            CliRunMode::Tui,
        ))
    }
}
