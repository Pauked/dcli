use eyre::Context;

use crate::{db, tui};

pub fn get_pwad_id_from_from_active_profile(error_str: &str) -> Result<i32, eyre::Report> {
    let app_settings = db::get_app_settings()?;

    if app_settings.active_profile_id.is_none() {
        return Err(eyre::eyre!(format!(
            "No Active Profile found. {}",
            error_str
        )));
    };

    let profile = db::get_profile_by_id(app_settings.active_profile_id.unwrap())
        .wrap_err("Unable to get Profile".to_string())?;

    Ok(profile.pwad_id.unwrap_or(0))
}

pub fn get_pwad_id_from_from_last_profile(error_str: &str) -> Result<i32, eyre::Report> {
    let app_settings = db::get_app_settings()?;

    if app_settings.last_profile_id.is_none() {
        return Err(eyre::eyre!(format!(
            "No Active Profile found. {}",
            error_str
        )));
    };

    let profile = db::get_profile_by_id(app_settings.last_profile_id.unwrap())
        .wrap_err("Unable to get Profile".to_string())?;

    Ok(profile.pwad_id.unwrap_or(0))
}

pub fn get_pwad_id_from_pick_profile(
    option_str: &str,
    error_str: &str,
) -> Result<i32, eyre::Report> {
    let profile_list = db::get_profile_display_list()?;
    if profile_list.is_empty() {
        return Err(eyre::eyre!("There are no profiles to select from."));
    }

    let profile_selection = inquire::Select::new(option_str, profile_list)
        .with_page_size(tui::MENU_PAGE_SIZE)
        .prompt_skippable()?;

    if let Some(profile) = profile_selection {
        return Ok(profile.pwad_id);
    }

    Err(eyre::eyre!(error_str.to_string()))
}

pub fn get_pwad_id_from_pick_pwad(
    option_text: &str,
    error_str: &str,
) -> Result<i32, eyre::Report> {
    let pwad_list = db::get_pwads()?;
    if pwad_list.is_empty() {
        return Err(eyre::eyre!("There are no PWADs to select from."));
    }

    let pwad_selection = inquire::Select::new(option_text, pwad_list)
        .with_page_size(tui::MENU_PAGE_SIZE)
        .prompt_skippable()?;

    if let Some(pwad) = pwad_selection {
        return Ok(pwad.id);
    }

    Err(eyre::eyre!(error_str.to_string()))
}
