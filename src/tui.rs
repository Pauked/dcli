use std::str::FromStr;

use dialoguer::{theme::ColorfulTheme, Select};

use crate::constants;

pub fn prompt() -> constants::Command {
    let selections = &[
        constants::Command::Play.to_string(),
        constants::Command::Config.to_string(),
        constants::Command::Editor.to_string(),
        constants::Command::Init.to_string(),
        constants::Command::Quit.to_string(),
    ];

    let selection_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an option")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    constants::Command::from_str(&selections[selection_index]).unwrap()
}
