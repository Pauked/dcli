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
mod menu_config;
mod menu_game_settings;
mod menu_main;
mod menu_map_editor;
mod menu_profiles;
mod menu_view_map_readme;
mod paths;
mod runner;
mod tui;

#[tokio::main]
async fn run() -> eyre::Result<String> {
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
        return Ok(format!("{} {}", constants::APP_NAME, constants::CRATE_VERSION));
    }

    let reset_mode =  args.len() == 1 && args.contains(&tui::ARG_RESET.to_string());
    if !reset_mode {
        db::create_db().await?;
        if db::is_empty_app_settings_table().await? {
            info!("No settings found, running init...");
            menu_config::init().await?;
        }
    }

    for arg in args {
        debug!("Running arg: {}", arg);
        // TODO: Refactor to be less bad
        let main_arg: tui::MainCommand = tui::convert_arg_to_maincommand(&arg);
        if main_arg != tui::MainCommand::Unknown {
            menu_main::run_main_menu_option(main_arg).await?;
        } else {
            let config_arg = tui::convert_arg_to_configcommand(&arg);
            if config_arg != tui::ConfigCommand::Unknown {
                let result = menu_config::run_config_menu_option(config_arg.clone()).await?;
                if config_arg == tui::ConfigCommand::Reset && result != *"Database reset not confirmed." {
                    menu_config::init().await?;
                }
            } else {
                info!("Unknown argument: {}", arg);
            }
        }
    }

    info!("Welcome to {}", constants::APP_NAME.bright_yellow());
    menu_main::main_menu().await
}

// async fn manage_args() -> eyre::Result<String> {


//     Ok("".to_string())
// }

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
