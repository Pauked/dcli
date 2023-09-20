use eyre::Context;
use log::info;

use crate::{data, db, tui};

fn pick_from_pwad_from_profile_tuple(pwad_ids: data::PwadIds) -> Result<i32, eyre::Report> {
    let pwad_list = db::get_pwads_by_ids(pwad_ids)?;
    if pwad_list.is_empty() {
        return Err(eyre::eyre!("There are no PWADs to select from."));
    }

    // If there's only one, just return it
    if pwad_list.len() == 1 {
        return Ok(pwad_list[0].id);
    }

    let pwad_selection = inquire::Select::new("Pick the PWAD you want to use:", pwad_list)
        .with_page_size(tui::MENU_PAGE_SIZE)
        .prompt_skippable()?;

    if let Some(pwad) = pwad_selection {
        return Ok(pwad.id);
    }

    Err(eyre::eyre!("Cancelled picking PWAD."))
}

pub fn get_pwad_id_from_from_default_profile(error_str: &str) -> Result<i32, eyre::Report> {
    let app_settings = db::get_app_settings()?;

    if app_settings.default_profile_id.is_none() {
        return Err(eyre::eyre!(format!(
            "No Default Profile found. {}",
            error_str
        )));
    };

    let profile = db::get_profile_by_id(app_settings.default_profile_id.unwrap())
        .wrap_err("Unable to get Profile".to_string())?;

    let pwad_id = pick_from_pwad_from_profile_tuple((
        profile.pwad_id.unwrap_or(0),
        profile.pwad_id2.unwrap_or(0),
        profile.pwad_id3.unwrap_or(0),
        profile.pwad_id3.unwrap_or(0),
        profile.pwad_id5.unwrap_or(0),
    ))?;

    Ok(pwad_id)
}

pub fn get_pwad_id_from_from_last_profile(error_str: &str) -> Result<i32, eyre::Report> {
    let app_settings = db::get_app_settings()?;

    if app_settings.last_profile_id.is_none() {
        return Err(eyre::eyre!(format!("No Last Profile found. {}", error_str)));
    };

    let profile = db::get_profile_by_id(app_settings.last_profile_id.unwrap())
        .wrap_err("Unable to get Profile".to_string())?;

    let pwad_id = pick_from_pwad_from_profile_tuple((
        profile.pwad_id.unwrap_or(0),
        profile.pwad_id2.unwrap_or(0),
        profile.pwad_id3.unwrap_or(0),
        profile.pwad_id3.unwrap_or(0),
        profile.pwad_id5.unwrap_or(0),
    ))?;

    Ok(pwad_id)
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
        let pwad_id = pick_from_pwad_from_profile_tuple(profile.pwad_ids)?;

        return Ok(pwad_id);
    }

    Err(eyre::eyre!(error_str.to_string()))
}

pub fn get_pwad_id_from_pick_pwad(option_text: &str, error_str: &str) -> Result<i32, eyre::Report> {
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

pub fn get_pwad_selection(
    pwads: Vec<data::Pwad>,
    default_pwads: Vec<usize>,
) -> Result<Vec<data::Pwad>, eyre::Report> {
    // Multiselect of up to five PWADs, can be aborted
    let selected_items =
        inquire::MultiSelect::new("Pick the PWAD you want to use (optional):", pwads.clone())
            .with_default(&default_pwads)
            .with_page_size(tui::MENU_PAGE_SIZE)
            .with_help_message("You can select up to five PWADs")
            .with_validator(inquire::max_length!(5))
            .prompt_skippable()?;

    if let Some(unwrapped_selected_items) = selected_items {
        // No ordering need if they just pick one
        if unwrapped_selected_items.len() == 1 {
            return Ok(vec![
                unwrapped_selected_items[0].clone(),
                data::Pwad::default(),
                data::Pwad::default(),
                data::Pwad::default(),
                data::Pwad::default(),
            ]);
        }

        // Ordering loop
        loop {
            let mut ordered_items = Vec::with_capacity(unwrapped_selected_items.len());
            let mut temp_items = unwrapped_selected_items.clone();

            for i in 0..temp_items.len() {
                let selected = inquire::Select::new(
                    &format!("Pick PWAD #{} from your selected PWADs:", i + 1),
                    temp_items.clone(),
                )
                .prompt()
                .unwrap();

                ordered_items.push(selected.clone());
                temp_items.remove(temp_items.iter().position(|x| x.id == selected.id).unwrap());
            }

            info!("\nYour ordered selection:");
            for (i, item) in ordered_items.iter().enumerate() {
                info!("{}: {}", i + 1, item);
            }

            let confirm = inquire::Confirm::new("Are you happy with this order?")
                .with_default(true)
                .prompt()
                .unwrap();

            if confirm {
                if ordered_items.len() < 5 {
                    for _ in ordered_items.len()..5 {
                        ordered_items.push(data::Pwad::default());
                    }
                }
                return Ok(ordered_items);
            }
        }
    } else {
        // They skipped, so default the result
        Ok(vec![
            data::Pwad::default(),
            data::Pwad::default(),
            data::Pwad::default(),
            data::Pwad::default(),
            data::Pwad::default(),
        ])
    }
}
