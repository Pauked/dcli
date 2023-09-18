use std::str::FromStr;

use colored::Colorize;
use inquire::validator::Validation;

use crate::{constants, data, db, paths};

pub async fn update_comp_level() -> Result<String, eyre::Report> {
    let selections = vec![
        constants::MENU_NOT_SET.to_string(),
        data::CompLevel::Default.to_string(),
        data::CompLevel::DoomAndDoom2.to_string(),
        data::CompLevel::UltimateDoom.to_string(),
        data::CompLevel::FinalDoom.to_string(),
        data::CompLevel::Boom.to_string(),
        data::CompLevel::Mbf.to_string(),
        data::CompLevel::Mbf21.to_string(),
    ];

    let mut game_settings = db::get_game_settings().await?;
    let starting_cursor = match game_settings.comp_level {
        Some(ref s) => selections.iter().position(|x| x == &s.to_string()).unwrap(),
        None => 0,
    };

    let comp_level = inquire::Select::new("Select a Compatibility Level:", selections)
        .with_starting_cursor(starting_cursor)
        .prompt()?;

    if comp_level == constants::MENU_NOT_SET {
        game_settings.comp_level = None;
    } else {
        game_settings.comp_level = Some(data::CompLevel::from_str(&comp_level).unwrap());
    }
    db::save_game_settings(game_settings).await?;

    Ok("Successfully updated Compatibility Level"
        .green()
        .to_string())
}

pub async fn update_config_file() -> Result<String, eyre::Error> {
    let mut game_settings = db::get_game_settings().await?;
    game_settings.config_file = inquire::Text::new("Enter Config File Path:")
        .with_validator(|input: &str| {
            if paths::file_exists(input) {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid("Config File does not exist.".into()))
            }
        })
        .with_default(&game_settings.config_file.unwrap_or("".to_string()))
        .with_help_message("Include the full path and file name")
        .prompt_skippable()?;
    db::save_game_settings(game_settings).await?;

    Ok("Successfully updated Warp".green().to_string())
}

pub async fn update_fast_monsters() -> Result<String, eyre::Error> {
    let mut game_settings = db::get_game_settings().await?;
    game_settings.fast_monsters = inquire::Confirm::new("Enable Fast Monsters?")
        .with_default(game_settings.fast_monsters)
        .prompt()
        .unwrap();
    db::save_game_settings(game_settings).await?;

    Ok("Successfully updated Fast Monsters".green().to_string())
}

pub async fn update_no_monsters() -> Result<String, eyre::Error> {
    let mut game_settings = db::get_game_settings().await?;
    game_settings.no_monsters = inquire::Confirm::new("Enable No Monsters?")
        .with_default(game_settings.no_monsters)
        .prompt()
        .unwrap();
    db::save_game_settings(game_settings).await?;

    Ok("Successfully updated No Monsters".green().to_string())
}

pub async fn update_respawn_monsters() -> Result<String, eyre::Error> {
    let mut game_settings = db::get_game_settings().await?;
    game_settings.respawn_monsters = inquire::Confirm::new("Enable Respawn Monsters?")
        .with_default(game_settings.respawn_monsters)
        .prompt()
        .unwrap();
    db::save_game_settings(game_settings).await?;

    Ok("Successfully updated Respawn Monsters".green().to_string())
}

pub async fn update_warp_to_level() -> Result<String, eyre::Error> {
    let mut game_settings = db::get_game_settings().await?;
    game_settings.warp = inquire::Text::new("Enter Warp value:")
        .with_default(&game_settings.warp.unwrap_or("".to_string()))
        .with_help_message("Typically in the format of m (1-32) or e m (1-4, 1-9)")
        .prompt_skippable()?;
    db::save_game_settings(game_settings).await?;

    Ok("Successfully updated Warp".green().to_string())
}

pub async fn update_skill() -> Result<String, eyre::Error> {
    let mut game_settings = db::get_game_settings().await?;

    game_settings.skill = inquire::CustomType::<u8>::new("Enter Skill value:")
        .with_validator(|input: &u8| {
            if (&1..=&5).contains(&input) {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid(
                    "Skill is not within the range [1-5].".into(),
                ))
            }
        })
        .with_default(game_settings.skill.unwrap_or(4))
        .with_help_message("Range is 1 to 5")
        .prompt_skippable()
        .unwrap();

    db::save_game_settings(game_settings).await?;
    Ok("Successfully updated Skill".green().to_string())
}

pub async fn update_turbo() -> Result<String, eyre::Error> {
    let mut game_settings = db::get_game_settings().await?;

    game_settings.turbo = inquire::CustomType::<u8>::new("Enter Turbo value:")
        .with_validator(|input: &u8| {
            if (&50..=&255).contains(&input) {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid("Turbo is not within the range.".into()))
            }
        })
        .with_default(game_settings.turbo.unwrap_or(255))
        .with_help_message("Range is 10 to 255")
        .prompt_skippable()
        .unwrap();

    db::save_game_settings(game_settings).await?;
    Ok("Successfully updated Turbo".green().to_string())
}

pub async fn update_timer() -> Result<String, eyre::Error> {
    let mut game_settings = db::get_game_settings().await?;

    game_settings.timer = inquire::CustomType::<u32>::new("Enter Timer value:")
        .with_validator(|input: &u32| {
            if (&1..=&43800).contains(&input) {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid("Timer is not within the range.".into()))
            }
        })
        .with_default(game_settings.timer.unwrap_or(10))
        .with_help_message("Range is 1 to 43800")
        .prompt_skippable()
        .unwrap();

    db::save_game_settings(game_settings).await?;
    Ok("Successfully updated Timer".green().to_string())
}

pub async fn update_height() -> Result<String, eyre::Error> {
    let mut game_settings = db::get_game_settings().await?;

    game_settings.height = inquire::CustomType::<u32>::new("Enter Screen Height:")
        .with_validator(|input: &u32| {
            if (&1..=&10240).contains(&input) {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid(
                    "Height is not within the range.".into(),
                ))
            }
        })
        .with_default(game_settings.height.unwrap_or(768))
        .with_help_message("Range is 1 to 10240")
        .prompt_skippable()
        .unwrap();

    db::save_game_settings(game_settings).await?;
    Ok("Successfully updated Height".green().to_string())
}

pub async fn update_width() -> Result<String, eyre::Error> {
    let mut game_settings = db::get_game_settings().await?;

    game_settings.width = inquire::CustomType::<u32>::new("Enter Screen Width:")
        .with_validator(|input: &u32| {
            if (&1..=&2880).contains(&input) {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid("Width is not within the range.".into()))
            }
        })
        .with_default(game_settings.width.unwrap_or(1024))
        .with_help_message("Range is 1 to 2880")
        .prompt_skippable()
        .unwrap();

    db::save_game_settings(game_settings).await?;
    Ok("Successfully updated Width".green().to_string())
}

pub async fn update_full_screen() -> Result<String, eyre::Error> {
    let mut game_settings = db::get_game_settings().await?;
    game_settings.full_screen = inquire::Confirm::new("Enable Full Screen?")
        .with_default(game_settings.full_screen)
        .prompt()
        .unwrap();
    db::save_game_settings(game_settings).await?;

    Ok("Successfully updated Fast Monsters".green().to_string())
}

pub async fn update_windowed() -> Result<String, eyre::Error> {
    let mut game_settings = db::get_game_settings().await?;
    game_settings.windowed = inquire::Confirm::new("Enable Windowed Mode?")
        .with_default(game_settings.windowed)
        .prompt()
        .unwrap();
    db::save_game_settings(game_settings).await?;

    Ok("Successfully updated Windowed Mode".green().to_string())
}

pub async fn update_additional_arguments() -> Result<String, eyre::Error> {
    let mut game_settings = db::get_game_settings().await?;
    game_settings.additional_arguments = inquire::Text::new("Enter any Additional Arguments:")
        .with_default(&game_settings.additional_arguments.unwrap_or("".to_string()))
        .prompt_skippable()?;
    db::save_game_settings(game_settings).await?;

    Ok("Successfully updated Additional Arguments"
        .green()
        .to_string())
}
