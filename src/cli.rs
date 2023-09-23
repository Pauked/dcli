use clap::Parser;
use log::info;

use crate::{
    constants, db, menu_app_settings,
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
    Init,

    /// Resets the database, nuking all settings.
    #[clap(short_flag = 'r')]
    Reset {
        /// Force database reset and skip confirmation prompt.
        #[arg(long, default_value = "false")]
        force: bool,
    },
}

pub fn run_cli_action(args: Args) -> Result<(String, CliRunMode), eyre::Report> {
    if let Some(action) = args.action {
        // If we are not resetting the database, make sure it exists and is ready to use
        match action {
            Action::Reset { force: _ } => {}
            _ => {
                db::create_db()?;
                if db::is_empty_app_settings_table()? {
                    info!("No settings found, running init...");
                    menu_app_settings::init()?;
                }
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
            Action::Init => Ok((tui::run_menu_command(MenuCommand::Init)?, CliRunMode::Tui)),
            Action::Reset { force } => {
                let result = tui::run_menu_command_with_force(MenuCommand::Reset, force)?;
                if result != *"Database reset not confirmed" {
                    Ok((tui::run_menu_command(MenuCommand::Init)?, CliRunMode::Tui))
                } else {
                    Ok((result, CliRunMode::Tui))
                }
            }
        }
    } else {
        Ok(("No arguments specified, continue to UI".to_string(), CliRunMode::Tui))
    }
}
