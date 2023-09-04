use std::{env, process};

use color_eyre::{eyre, owo_colors::OwoColorize};
use log::{debug, info};

mod actions;
mod app_settings;
mod constants;
mod data;
mod db;
mod doom_data;
mod finder;
mod init;
mod log_config;
mod paths;
mod profiles;
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

    info!("Welcome to {}", constants::APP_NAME.bright_yellow());

    // Attempt to run from arguments
    // We don't want the full exe path, just the args
    let args: Vec<String> = env::args().skip(1).collect();

    let reset_mode = args.contains(&constants::ARG_RESET.to_string());
    if !reset_mode {
        db::create_db().await?;
        if db::is_empty_settings_table().await? {
            info!("No settings found, running init...");
            init::init().await?;
        }
    }

    for arg in args {
        debug!("Running arg: {}", arg);
        actions::run_option(constants::convert_arg_to_cmd(&arg)).await?;
    }

    // Wait for user input
    loop {
        let menu_command = tui::main_menu_prompt();
        if let constants::Command::Quit = menu_command {
            return Ok("Quitting...".to_string());
        }

        let result = actions::run_option(menu_command).await?;
        info!("{}", result)
    }
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
