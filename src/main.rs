use std::{env, io::Write, process};

use color_eyre::eyre;
use log::debug;

mod actions;
mod config;
mod constants;
mod db;
mod editor;
mod init;
mod log_config;
mod paths;
mod play;
mod settings;

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

    println!("Starting {}...", constants::APP_NAME);

    // Init database
    actions::create_db().await?;

    // Attempt to run from arguments
    let args: Vec<String> = env::args().collect();
    for arg in args {
        run_option(convert_arg_to_cmd(&arg));
    }

    // Wait for user input
    loop {
        let input = prompt(constants::PROMPT);
        run_option(&input);
    }
}

fn convert_arg_to_cmd(arg: &str) -> &str {
    match arg {
        constants::ARG_PLAY => constants::CMD_PLAY,
        constants::ARG_EDITOR => constants::CMD_EDITOR,
        constants::ARG_CONFIG => constants::CMD_CONFIG,
        constants::ARG_INIT => constants::CMD_INIT,
        _ => constants::CMD_USER_INPUT,
    }
}

fn run_option(option: &str) {
    println!("Running Option - {}", option);

    let config_file_path = settings::get_config_filename(constants::CONFIG_FILE);
    let settings = settings::get(config_file_path.clone());
    /*
        println!(
            "Settings:
    Doom Exe - '{}'
    IWAD     - '{}'
    File     - '{}'
    Editor   - '{}'",
            settings.doom_exe, settings.iwad, settings.file, settings.editor_exe
        );
         */

    match option {
        constants::CMD_PLAY => play::play(settings),
        constants::CMD_CONFIG => config::config(config_file_path),
        constants::CMD_EDITOR => editor::editor(settings),
        constants::CMD_INIT => init::init(),
        constants::CMD_EXIT => exit(),
        constants::CMD_QUIT => exit(),
        _ => (),
    }
}

fn prompt(prompt: &str) -> String {
    let mut line = String::new();
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Error: Could not read a line");

    return line.trim().to_string();
}

fn exit() {
    println!("Exiting {}...", constants::APP_NAME);
    process::exit(0);
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

