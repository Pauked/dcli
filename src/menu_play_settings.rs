use std::str::FromStr;

use eyre::Context;
use inquire::validator::Validation;
use tabled::settings::{object::Rows, Modify, Rotate, Style, Width};

use crate::{constants, data, db, paths, tui};

pub fn update_comp_level() -> Result<String, eyre::Report> {
    let selections = vec![
        constants::MENU_NOT_SET.to_string(),
        data::CompLevel::DoomV12.to_string(),
        data::CompLevel::DoomV1666.to_string(),
        data::CompLevel::DoomV19.to_string(),
        data::CompLevel::UltimateDoom.to_string(),
        data::CompLevel::FinalDoom.to_string(),
        data::CompLevel::DosDoom.to_string(),
        data::CompLevel::TasDoom.to_string(),
        data::CompLevel::Boom.to_string(),
        data::CompLevel::BoomV201.to_string(),
        data::CompLevel::BoomV202.to_string(),
        data::CompLevel::LxDoom.to_string(),
        data::CompLevel::Mbf.to_string(),
        data::CompLevel::PrBoomPlus.to_string(),
        data::CompLevel::Mbf21.to_string(),
    ];

    let mut play_settings = db::get_play_settings()?;
    let starting_cursor = match play_settings.comp_level {
        Some(ref c) => selections.iter().position(|x| x == &c.to_string()).unwrap(),
        None => 0,
    };

    let comp_level = inquire::Select::new("Select a Compatibility Level:", selections)
        .with_starting_cursor(starting_cursor)
        .with_page_size(tui::MENU_PAGE_SIZE)
        .prompt()?;

    if comp_level == constants::MENU_NOT_SET {
        play_settings.comp_level = None;
    } else {
        play_settings.comp_level = Some(data::CompLevel::from_str(&comp_level).unwrap());
    }
    db::save_play_settings(play_settings.clone())?;

    Ok(format!(
        "Successfully updated Compatibility Level to '{}'",
        data::display_option_comp_level(&play_settings.comp_level)
    ))
}

pub fn cli_set_comp_level(comp_level: data::CompLevel) -> Result<String, eyre::Report> {
    let mut play_settings = db::get_play_settings()?;
    if comp_level == data::CompLevel::NotSet {
        play_settings.comp_level = None;
    } else {
        play_settings.comp_level = Some(comp_level);
    }
    db::save_play_settings(play_settings.clone())?;
    Ok(format!(
        "Successfully updated Compatibility Level to '{}'",
        data::display_option_comp_level(&play_settings.comp_level)
    ))
}

pub fn update_config_file() -> Result<String, eyre::Error> {
    let mut play_settings = db::get_play_settings()?;
    let config_file = inquire::Text::new("Enter Config File Path:")
        .with_validator(|input: &str| {
            if input.to_lowercase() == tui::MENU_CLR || paths::file_exists(input) {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid("Config File does not exist".into()))
            }
        })
        .with_default(&play_settings.config_file.clone().unwrap_or("".to_string()))
        .with_help_message(&format!(
            "Include the full path and file name. {}",
            tui::MENU_CLR_MESSAGE
        ))
        .prompt_skippable()?;

    play_settings.config_file =
        config_file.filter(|config_file| config_file.to_lowercase() != tui::MENU_CLR);
    db::save_play_settings(play_settings.clone())?;

    Ok(format!(
        "Successfully updated Config File to '{}'",
        data::display_option_string(&play_settings.config_file)
    ))
}

pub fn cli_set_config_file(config_file: &str) -> Result<String, eyre::Error> {
    let mut play_settings = db::get_play_settings()?;
    if config_file.to_lowercase() == tui::MENU_CLR {
        play_settings.config_file = None;
    } else {
        if !paths::file_exists(config_file) {
            return Ok(format!(
                "Cannot update Config File because it does not exist - '{}'",
                config_file
            ));
        }
        play_settings.config_file = Some(config_file.to_string());
    }
    db::save_play_settings(play_settings.clone())?;

    Ok(format!(
        "Successfully updated Config File to '{}'",
        data::display_option_string(&play_settings.config_file)
    ))
}

pub fn update_fast_monsters() -> Result<String, eyre::Error> {
    let mut play_settings = db::get_play_settings()?;
    play_settings.fast_monsters = inquire::Confirm::new("Enable Fast Monsters?")
        .with_default(play_settings.fast_monsters)
        .prompt()?;
    db::save_play_settings(play_settings.clone())?;

    Ok(format!(
        "Successfully updated Fast Monsters to '{}'",
        &play_settings.fast_monsters
    ))
}

pub fn cli_set_fast_monsters(fast_monsters: bool) -> Result<String, eyre::Error> {
    let mut play_settings = db::get_play_settings()?;
    play_settings.fast_monsters = fast_monsters;
    db::save_play_settings(play_settings.clone())?;

    Ok(format!(
        "Successfully updated Fast Monsters to '{}'",
        &play_settings.fast_monsters
    ))
}

pub fn update_no_monsters() -> Result<String, eyre::Error> {
    let mut play_settings = db::get_play_settings()?;
    play_settings.no_monsters = inquire::Confirm::new("Enable No Monsters?")
        .with_default(play_settings.no_monsters)
        .prompt()?;
    db::save_play_settings(play_settings.clone())?;

    Ok(format!(
        "Successfully updated No Monsters to '{}'",
        &play_settings.no_monsters
    ))
}

pub fn cli_set_no_monsters(no_monsters: bool) -> Result<String, eyre::Error> {
    let mut play_settings = db::get_play_settings()?;
    play_settings.no_monsters = no_monsters;
    db::save_play_settings(play_settings.clone())?;

    Ok(format!(
        "Successfully updated No Monsters to '{}'",
        &play_settings.no_monsters
    ))
}

pub fn update_respawn_monsters() -> Result<String, eyre::Error> {
    let mut play_settings = db::get_play_settings()?;
    play_settings.respawn_monsters = inquire::Confirm::new("Enable Respawn Monsters?")
        .with_default(play_settings.respawn_monsters)
        .prompt()?;
    db::save_play_settings(play_settings.clone())?;

    Ok(format!(
        "Successfully updated Respawn Monsters to '{}'",
        &play_settings.respawn_monsters
    ))
}

pub fn cli_set_respawn_monsters(respawn_monsters: bool) -> Result<String, eyre::Error> {
    let mut play_settings = db::get_play_settings()?;
    play_settings.respawn_monsters = respawn_monsters;
    db::save_play_settings(play_settings.clone())?;

    Ok(format!(
        "Successfully updated Respawn Monsters to '{}'",
        &play_settings.respawn_monsters
    ))
}

pub fn update_warp_to_level() -> Result<String, eyre::Error> {
    let mut play_settings = db::get_play_settings()?;
    let warp = inquire::Text::new("Enter Warp value:")
        .with_default(&play_settings.warp.unwrap_or("".to_string()))
        .with_help_message(&format!(
            "Typically in the format of m (1-32) or e m (1-4, 1-9). {}",
            tui::MENU_CLR_MESSAGE
        ))
        .prompt_skippable()?;

    play_settings.warp = warp.filter(|warp| warp.to_lowercase() != tui::MENU_CLR);
    db::save_play_settings(play_settings.clone())?;

    Ok(format!(
        "Successfully updated Warp to '{}'",
        data::display_option_string(&play_settings.warp)
    ))
}

pub fn cli_set_warp_to_level(warp: &str) -> Result<String, eyre::Error> {
    let mut play_settings = db::get_play_settings()?;
    if warp.to_lowercase() == tui::MENU_CLR {
        play_settings.warp = None;
    } else {
        play_settings.warp = Some(warp.to_string());
    }
    db::save_play_settings(play_settings.clone())?;

    Ok(format!(
        "Successfully updated Warp to '{}'",
        data::display_option_string(&play_settings.warp)
    ))
}

pub fn update_skill() -> Result<String, eyre::Error> {
    let mut play_settings = db::get_play_settings()?;

    play_settings.skill = inquire::CustomType::<u8>::new("Enter Skill value:")
        .with_validator(|input: &u8| {
            if (&1..=&5).contains(&input) {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid(
                    "Skill is not within the range [1-5]".into(),
                ))
            }
        })
        .with_default(play_settings.skill.unwrap_or(4))
        .with_help_message("Range is 1 to 5")
        .prompt_skippable()?;
    db::save_play_settings(play_settings.clone())?;

    Ok(format!(
        "Successfully updated Skill to '{}'",
        data::display_option_u8(&play_settings.skill)
    ))
}

pub fn cli_set_skill(skill: u8) -> Result<String, eyre::Error> {
    let mut play_settings = db::get_play_settings()?;

    if (1..=5).contains(&skill) {
        play_settings.skill = Some(skill);
        db::save_play_settings(play_settings.clone())?;
    } else {
        return Ok("Cannot update Skill because value is not within the range [1-5]".to_string());
    }

    Ok(format!(
        "Successfully updated Skill to '{}'",
        data::display_option_u8(&play_settings.skill)
    ))
}

pub fn update_turbo() -> Result<String, eyre::Error> {
    let mut play_settings = db::get_play_settings()?;

    play_settings.turbo = inquire::CustomType::<u8>::new("Enter Turbo value:")
        .with_validator(|input: &u8| {
            if (&50..=&255).contains(&input) {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid("Turbo is not within the range".into()))
            }
        })
        .with_default(play_settings.turbo.unwrap_or(255))
        .with_help_message("Range is 10 to 255")
        .prompt_skippable()?;
    db::save_play_settings(play_settings.clone())?;

    Ok(format!(
        "Successfully updated Turbo to '{}'",
        data::display_option_u8(&play_settings.turbo)
    ))
}

pub fn cli_set_turbo(turbo: u8) -> Result<String, eyre::Error> {
    let mut play_settings = db::get_play_settings()?;
    if (50..=255).contains(&turbo) {
        play_settings.turbo = Some(turbo);
        db::save_play_settings(play_settings.clone())?;
    } else {
        return Ok(
            "Cannot update Turbo because value is not within the range [50-255]".to_string(),
        );
    }

    Ok(format!(
        "Successfully updated Turbo to '{}'",
        data::display_option_u8(&play_settings.turbo)
    ))
}

pub fn update_timer() -> Result<String, eyre::Error> {
    let mut play_settings = db::get_play_settings()?;

    play_settings.timer = inquire::CustomType::<u32>::new("Enter Timer value:")
        .with_validator(|input: &u32| {
            if (&1..=&43800).contains(&input) {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid("Timer is not within the range".into()))
            }
        })
        .with_default(play_settings.timer.unwrap_or(10))
        .with_help_message("Range is 1 to 43800")
        .prompt_skippable()?;
    db::save_play_settings(play_settings.clone())?;

    Ok(format!(
        "Successfully updated Timer to '{}'",
        data::display_option_u32(&play_settings.timer)
    ))
}

pub fn cli_set_timer(timer: u32) -> Result<String, eyre::Error> {
    let mut play_settings = db::get_play_settings()?;
    if (1..=43800).contains(&timer) {
        play_settings.timer = Some(timer);
        db::save_play_settings(play_settings.clone())?;
    } else {
        return Ok(
            "Cannot update Timer because value is not within the range [1-43800]".to_string(),
        );
    }

    Ok(format!(
        "Successfully updated Timer to '{}'",
        data::display_option_u32(&play_settings.timer)
    ))
}

pub fn update_screen_height() -> Result<String, eyre::Error> {
    let mut play_settings = db::get_play_settings()?;

    play_settings.height = inquire::CustomType::<u32>::new("Enter Screen Height:")
        .with_validator(|input: &u32| {
            if (&1..=&10240).contains(&input) {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid(
                    "Screen Height is not within the range".into(),
                ))
            }
        })
        .with_default(play_settings.height.unwrap_or(768))
        .with_help_message("Range is 1 to 10240")
        .prompt_skippable()?;
    db::save_play_settings(play_settings.clone())?;

    Ok(format!(
        "Successfully updated Screen Height to '{}'",
        data::display_option_u32(&play_settings.height)
    ))
}

pub fn cli_set_screen_height(height: u32) -> Result<String, eyre::Error> {
    let mut play_settings = db::get_play_settings()?;
    if (1..=10240).contains(&height) {
        play_settings.height = Some(height);
        db::save_play_settings(play_settings.clone())?;
    } else {
        return Ok(
            "Cannot update Screen Height because value is not within the range [1-10240]"
                .to_string(),
        );
    }

    Ok(format!(
        "Successfully updated Screen Height to '{}'",
        data::display_option_u32(&play_settings.height)
    ))
}

pub fn update_screen_width() -> Result<String, eyre::Error> {
    let mut play_settings = db::get_play_settings()?;

    play_settings.width = inquire::CustomType::<u32>::new("Enter Screen Width:")
        .with_validator(|input: &u32| {
            if (&1..=&2880).contains(&input) {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid(
                    "Screen Width is not within the range".into(),
                ))
            }
        })
        .with_default(play_settings.width.unwrap_or(1024))
        .with_help_message("Range is 1 to 2880")
        .prompt_skippable()?;
    db::save_play_settings(play_settings.clone())?;

    Ok(format!(
        "Successfully updated Screen Width to '{}'",
        data::display_option_u32(&play_settings.width)
    ))
}

pub fn cli_set_screen_width(width: u32) -> Result<String, eyre::Error> {
    let mut play_settings = db::get_play_settings()?;
    if (1..=2880).contains(&width) {
        play_settings.width = Some(width);
        db::save_play_settings(play_settings.clone())?;
    } else {
        return Ok(
            "Cannot update Screen Width because value is not within the range [1-2880]".to_string(),
        );
    }

    Ok(format!(
        "Successfully updated Screen Width to '{}'",
        data::display_option_u32(&play_settings.width)
    ))
}

pub fn update_full_screen() -> Result<String, eyre::Error> {
    let mut play_settings = db::get_play_settings()?;
    play_settings.full_screen = inquire::Confirm::new("Enable Full Screen?")
        .with_default(play_settings.full_screen)
        .prompt()?;
    db::save_play_settings(play_settings.clone())?;

    Ok(format!(
        "Successfully updated Full Screen as '{}'",
        &play_settings.full_screen
    ))
}

pub fn cli_set_full_screen(full_screen: bool) -> Result<String, eyre::Error> {
    let mut play_settings = db::get_play_settings()?;
    play_settings.full_screen = full_screen;
    db::save_play_settings(play_settings.clone())?;

    Ok(format!(
        "Successfully updated Full Screen as '{}'",
        &play_settings.full_screen
    ))
}

pub fn update_windowed() -> Result<String, eyre::Error> {
    let mut play_settings = db::get_play_settings()?;
    play_settings.windowed = inquire::Confirm::new("Enable Windowed Mode?")
        .with_default(play_settings.windowed)
        .prompt()?;
    db::save_play_settings(play_settings.clone())?;

    Ok(format!(
        "Successfully updated Windowed Mode as '{}'",
        &play_settings.windowed
    ))
}

pub fn cli_set_windowed(windowed: bool) -> Result<String, eyre::Error> {
    let mut play_settings = db::get_play_settings()?;
    play_settings.windowed = windowed;
    db::save_play_settings(play_settings.clone())?;

    Ok(format!(
        "Successfully updated Windowed Mode as '{}'",
        &play_settings.windowed
    ))
}

pub fn update_additional_arguments() -> Result<String, eyre::Error> {
    let mut play_settings = db::get_play_settings()?;
    play_settings.additional_arguments = inquire::Text::new("Enter any Additional Arguments:")
        .with_default(&play_settings.additional_arguments.unwrap_or("".to_string()))
        .prompt_skippable()?;
    db::save_play_settings(play_settings.clone())?;

    Ok(format!(
        "Successfully updated Additional Arguments to '{}'",
        data::display_option_string(&play_settings.additional_arguments)
    ))
}

pub fn cli_set_additional_args(additional_arguments: Vec<String>) -> Result<String, eyre::Error> {
    let mut play_settings = db::get_play_settings()?;
    play_settings.additional_arguments = Some(additional_arguments.join(" "));
    db::save_play_settings(play_settings.clone())?;

    Ok(format!(
        "Successfully updated Additional Arguments to '{}'",
        data::display_option_string(&play_settings.additional_arguments)
    ))
}

pub fn reset_play_settings(force: bool) -> Result<String, eyre::Error> {
    if force
        || inquire::Confirm::new("Are you sure you want to Reset your Play Settings?")
            .with_default(false)
            .prompt()?
    {
        let play_settings = db::get_play_settings()?;
        let add_play_settings = data::PlaySettings {
            id: play_settings.id,
            ..Default::default()
        };
        db::save_play_settings(add_play_settings)?;
        Ok("Successfully Reset Play Settings".to_string())
    } else {
        Ok("Reset Play Settings not confirmed".to_string())
    }
}

pub fn list_play_settings() -> Result<String, eyre::Report> {
    let play_settings =
        db::get_play_settings().wrap_err("Unable to get Play Settings listing".to_string())?;

    let table = tabled::Table::new(vec![play_settings])
        .with(Modify::new(Rows::new(1..)).with(Width::wrap(50).keep_words(true)))
        .with(Rotate::Left)
        .with(Rotate::Top)
        .with(Style::modern())
        .to_string();
    Ok(table)
}
