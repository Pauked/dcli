use eyre::Context;
use inquire::InquireError;
use log::info;

use crate::{data, db, tui};

fn pick_from_map_from_profile_map_ids(map_ids: data::MapIds) -> Result<i32, eyre::Report> {
    let map_list = db::get_maps_by_ids(map_ids)?;
    if map_list.is_empty() {
        return Err(eyre::eyre!("There are no Maps to select from"));
    }

    // If there's only one, just return it
    if map_list.len() == 1 {
        return Ok(map_list[0].id);
    }

    let map_selection = inquire::Select::new("Pick the Map you want to use:", map_list)
        .with_page_size(tui::MENU_PAGE_SIZE)
        .prompt_skippable()?;

    if let Some(map) = map_selection {
        return Ok(map.id);
    }

    Err(InquireError::OperationCanceled).wrap_err("Cancelled picking Map".to_string())
}

pub fn get_map_id_from_from_default_profile(error_str: &str) -> Result<i32, eyre::Report> {
    let app_settings = db::get_app_settings()?;

    if app_settings.default_profile_id.is_none() {
        return Err(eyre::eyre!(format!(
            "No Default Profile found. {}",
            error_str
        )));
    };

    let profile = db::get_profile_by_id(app_settings.default_profile_id.unwrap())
        .wrap_err("Unable to get Profile".to_string())?;

    let map_id = pick_from_map_from_profile_map_ids((
        profile.map_id.unwrap_or(0),
        profile.map_id2.unwrap_or(0),
        profile.map_id3.unwrap_or(0),
        profile.map_id3.unwrap_or(0),
        profile.map_id5.unwrap_or(0),
    ))?;

    Ok(map_id)
}

pub fn get_map_id_from_from_last_profile(error_str: &str) -> Result<i32, eyre::Report> {
    let app_settings = db::get_app_settings()?;

    if app_settings.last_profile_id.is_none() {
        return Err(eyre::eyre!(format!("No Last Profile found. {}", error_str)));
    };

    let profile = db::get_profile_by_id(app_settings.last_profile_id.unwrap())
        .wrap_err("Unable to get Profile".to_string())?;

    let map_id = pick_from_map_from_profile_map_ids((
        profile.map_id.unwrap_or(0),
        profile.map_id2.unwrap_or(0),
        profile.map_id3.unwrap_or(0),
        profile.map_id3.unwrap_or(0),
        profile.map_id5.unwrap_or(0),
    ))?;

    Ok(map_id)
}

pub fn get_map_id_from_pick_profile(
    option_str: &str,
    error_str: &str,
) -> Result<i32, eyre::Report> {
    let profile_list = db::get_profile_display_list()?;
    if profile_list.is_empty() {
        return Err(eyre::eyre!("There are no profiles to select from"));
    }

    let profile_selection = inquire::Select::new(option_str, profile_list)
        .with_page_size(tui::MENU_PAGE_SIZE)
        .prompt_skippable()?;

    if let Some(profile) = profile_selection {
        let map_id = pick_from_map_from_profile_map_ids(profile.map_ids)?;
        return Ok(map_id);
    }

    Err(InquireError::OperationCanceled).wrap_err(error_str.to_string())
}

pub fn get_map_id_from_pick_map(option_text: &str, error_str: &str) -> Result<i32, eyre::Report> {
    let map_list = db::get_maps()?;
    if map_list.is_empty() {
        return Err(eyre::eyre!("There are no Maps to select from"));
    }

    let map_selection = inquire::Select::new(option_text, map_list)
        .with_page_size(tui::MENU_PAGE_SIZE)
        .prompt_skippable()?;

    if let Some(map) = map_selection {
        return Ok(map.id);
    }

    Err(InquireError::OperationCanceled).wrap_err(error_str.to_string())
}

pub fn get_map_selection(
    maps: Vec<data::Map>,
    default_maps: Vec<usize>,
) -> Result<Vec<data::Map>, eyre::Report> {
    // Multiselect of up to five Maps, can be aborted
    let selected_items =
        inquire::MultiSelect::new("Pick the Map you want to use (optional):", maps.clone())
            .with_default(&default_maps)
            .with_page_size(tui::MENU_PAGE_SIZE)
            .with_help_message("You can select up to five Maps")
            .with_validator(inquire::max_length!(5))
            .prompt_skippable()?;

    if let Some(unwrapped_selected_items) = selected_items {
        // No ordering need if nothing is selected
        if unwrapped_selected_items.is_empty() {
            return Ok(vec![
                data::Map::default(),
                data::Map::default(),
                data::Map::default(),
                data::Map::default(),
                data::Map::default(),
            ]);
        }
        // No ordering need if they just pick one
        if unwrapped_selected_items.len() == 1 {
            return Ok(vec![
                unwrapped_selected_items[0].clone(),
                data::Map::default(),
                data::Map::default(),
                data::Map::default(),
                data::Map::default(),
            ]);
        }

        // Ordering loop
        loop {
            let mut ordered_items = Vec::with_capacity(unwrapped_selected_items.len());
            let mut temp_items = unwrapped_selected_items.clone();

            for i in 0..temp_items.len() {
                let selected = inquire::Select::new(
                    &format!("Pick Map #{} from your selected Maps:", i + 1),
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
                .prompt()?;

            if confirm {
                if ordered_items.len() < 5 {
                    for _ in ordered_items.len()..5 {
                        ordered_items.push(data::Map::default());
                    }
                }
                return Ok(ordered_items);
            }
        }
    } else {
        // They skipped, so default the result
        Ok(vec![
            data::Map::default(),
            data::Map::default(),
            data::Map::default(),
            data::Map::default(),
            data::Map::default(),
        ])
    }
}
