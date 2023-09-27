use clap::Parser;
use log::debug;

use crate::{
    constants, menu_app_settings,
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
    /// Play Doom with the Default Profile PWAD. Takes the first PWAD in Profile.
    #[clap(short_flag = 'p')]
    Play,

    /// Play Doom with the Last Run Profile. Takes the first PWAD in Profile.
    PlayLast,

    /// Open the Map Editor with the Default Profile PWAD. Takes the first PWAD in Profile.
    #[clap(short_flag = 'm')]
    MapEditor,

    /// Open the Map Editor with the Last Run Profile. Takes the first PWAD in Profile.
    MapEditorLast,

    /// Initializes the app for use. Asks a quick set of questions to get you Dooming!
    #[clap(short_flag = 'i')]
    Init {
        /// Engine path
        engine_path: String,
        /// IWAD path
        iwad_path: String,
        /// PWAD paths
        pwad_path: Option<String>,
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
    // #[clap(short_flag = 'a')]
    // AddProfile {
    //     /// Profile name
    //     name: String,
    //     /// Engine path
    //     engine: String,
    //     /// IWAD path
    //     iwad: String,
    //     /// PWAD path
    //     pwad: Option<String>,
    //     /// Additional arguments to pass to the engine
    //     #[arg(long)]
    //     additional_args: Option<Vec<String>>,
    // },
}

pub fn run_cli_action(args: Args) -> Result<(String, CliRunMode), eyre::Report> {
    if let Some(action) = args.action {
        // If we are not resetting the database, make sure it exists and is ready to use
        match action {
            Action::Reset { .. } => {}
            Action::Init { engine_path: _, iwad_path: _, pwad_path: _, force     } => {
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
            Action::MapEditor => Ok((
                tui::run_menu_command(MenuCommand::OpenFromDefaultProfile)?,
                CliRunMode::Quit,
            )),
            Action::MapEditorLast => Ok((
                tui::run_menu_command(MenuCommand::OpenFromDefaultProfile)?,
                CliRunMode::Quit,
            )),
            Action::Init {
                engine_path,
                iwad_path,
                pwad_path,
                force,
            } => {
                debug!("Init engine_path '{}', iwad_path '{}', pwad_path '{:?}', force '{}'", engine_path, iwad_path, pwad_path, force);
                let result = menu_app_settings::cli_init(engine_path, iwad_path, pwad_path, force)?;
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
            // Action::AddProfile {
            //     name,
            //     engine,
            //     iwad,
            //     pwad,
            //     additional_args,
            // } => {
            //     // let result = tui::run_menu_command(MenuCommand::CliAddProfile {
            //     //     name,
            //     //     engine,
            //     //     iwad,
            //     //     pwad,
            //     //     additional_args,
            //     // })?;
            //     Ok(("".to_string(), CliRunMode::Quit))
            // }
        }
    } else {
        Ok((
            "No arguments specified, continue to UI".to_string(),
            CliRunMode::Tui,
        ))
    }
}
