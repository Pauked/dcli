use std::{env, process};

use color_eyre::eyre;
use colored::Colorize;
use log::{debug, info};

mod constants;
mod data;
mod db;
mod doom_data;
mod files;
mod finder;
mod log_config;
mod menu_common;
mod menu_config;
mod menu_game_settings;
mod menu_main;
mod menu_map_editor;
mod menu_profiles;
mod menu_view_readme;
mod paths;
mod runner;
mod tui;

fn run() -> eyre::Result<String> {
    color_eyre::install()?;
    log_config::init_log(constants::APP_NAME);
    // This line is intentionally blank... so I can see new runs in the log file
    debug!("");
    debug!(
        "Starting '{}' from '{}', version {}",
        constants::APP_NAME,
        paths::get_current_exe(),
        constants::CRATE_VERSION,
    );

    // Attempt to run from arguments
    // We don't want the full exe path, just the args
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 1 && args.contains(&tui::ARG_VERSION.to_string()) {
        return Ok(format!(
            "{} {}",
            constants::APP_NAME,
            constants::CRATE_VERSION
        ));
    }

    let reset_mode = args.len() == 1 && args.contains(&tui::ARG_RESET.to_string());
    if !reset_mode {
        db::create_db()?;
        if db::is_empty_app_settings_table()? {
            info!("No settings found, running init...");
            //menu_config::init()?;
        }
    }

    for arg in args {
        debug!("Running arg: {}", arg);
        // TODO: Refactor to be less bad
        let main_arg = tui::convert_arg_to_menu_command(&arg);
        if main_arg != tui::MenuCommand::Ignore {
            let result = tui::run_menu_command(main_arg)?;
            if reset_mode && result != *"Database reset not confirmed." {
                menu_config::init()?;
            }
        }
    }

    info!("Welcome to {}", constants::APP_NAME.bright_yellow());
    tui::menu(tui::MenuLevel::Main)
}

fn main() {
    match run() {
        Err(error) => {
            log::error!("Error: {:?}", error);
            process::exit(1);
        }
        Ok(success) => {
            log::info!("{}", success);
            process::exit(0);
        }
    }
}
