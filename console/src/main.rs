use std::{env, io::Write, process};

mod config;
mod constants;
mod editor;
mod play;
mod settings;

fn main() {
    println!("Starting {}...", constants::UI_DOOM_CLI);

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
        _ => constants::CMD_USER_INPUT,
    }
}

fn run_option(option: &str) {
    println!("Running Option - {}", option);

    let config_file_path = settings::get_config_filename(constants::CONFIG_FILE);
    let settings = settings::get(config_file_path.clone());
    println!(
        "Settings:
Doom Exe - '{}'
IWAD     - '{}'
File     - '{}'
Editor   - '{}'",
        settings.doom_exe, settings.iwad, settings.file, settings.editor_exe
    );

    match option {
        constants::CMD_PLAY => play::play(settings),
        constants::CMD_CONFIG => config::config(config_file_path),
        constants::CMD_EDITOR => editor::editor(settings),
        constants::CMD_EXIT => exit(),
        _ => (),
    }
}

fn prompt(name: &str) -> String {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Error: Could not read a line");

    return line.trim().to_string();
}

fn exit() {
    println!("Exiting {}...", constants::UI_DOOM_CLI);
    process::exit(0);
}
