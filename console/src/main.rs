use std::{io::Write, env, process};

mod config;
mod constants;
mod play;
mod settings;

fn main() {
    println!("Doom CLI...");

    // Attempt to run from arguments
    let args: Vec<String> = env::args().collect();
    for arg in args {
        if arg == constants::ARG_PLAY {
            play::play();
            process::exit(0);
        } else if arg == constants::ARG_CONFIG {
            config::config();
        }
    }

    // Wait for user input
    loop {
        let input = prompt("> ");

        if input == constants::CMD_PLAY {
            play::play();
        } else if input == constants::CMD_CONFIG {
            config::config();
        } else if input == constants::CMD_EXIT {
            break;
        }
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
