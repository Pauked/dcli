use std::process;

use clap::Parser;
use color_eyre::eyre;
use log::{debug, info};
use owo_colors::{colors::xterm, OwoColorize};

mod cli;
mod constants;
mod data;
mod db;
mod doom_data;
mod files;
mod finder;
mod log_config;
mod menu_app_settings;
mod menu_common;
mod menu_editor;
mod menu_main;
mod menu_play_settings;
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

    let args = cli::Args::parse();
    log::debug!("Args {:?}", args);

    let (cli_result, cli_run_mode) = cli::run_cli_action(args)?;
    match cli_run_mode {
        cli::CliRunMode::Tui => {
            info!(
                "{} {}",
                "Welcome to".bold(),
                constants::APP_NAME.fg::<xterm::DarkSpringGreen>().bold()
            );
            menu_app_settings::check_app_can_run(false)?;
            tui::menu(tui::MenuLevel::Main)
        }
        cli::CliRunMode::Quit => Ok(tui::colour_result(&cli_result)),
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
