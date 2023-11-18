use chrono::Utc;
use eyre::Context;
use inquire::validator::Validation;
use log::info;
use tabled::settings::{object::Rows, Modify, Style, Width};

use crate::{data, db, tui};

pub fn add_queue() -> Result<String, eyre::Report> {
    let profiles = db::get_profile_display_list(data::ProfileOrder::Name)?;
    if profiles.is_empty() {
        return Ok(
            "There are no Profiles to select. Please create one before creating a Queue"
                .to_string(),
        );
    }

    // Name the new queue
    let queue_name = inquire::Text::new("Enter a name for your Queue:")
        .with_validator(|input: &str| {
            let queue_result = db::get_queue_by_name(input);
            if queue_result.is_ok() {
                return Ok(Validation::Invalid("Queue name already exists".into()));
            }

            if input.len() < 5 {
                Ok(Validation::Invalid(
                    "Queue name must be at least 5 characters".into(),
                ))
            } else {
                Ok(Validation::Valid)
            }
        })
        .prompt()?;

    // Pick the profiles and order you would like them in to add to the queue
    let profile_selection = get_profile_selection(profiles, vec![])?;

    // Save queue and get id
    let queue = data::Queue {
        id: 0,
        name: queue_name,
        date_created: Utc::now(),
        date_edited: Utc::now(),
    };
    let add_result = db::add_queue(queue.clone())?;
    let queue_id = add_result.last_insert_rowid().try_into().unwrap();

    // Save queue items and link to the queue
    for (i, profile) in profile_selection.iter().enumerate() {
        let queue_item = data::QueueItem {
            id: 0,
            profile_queue_id: queue_id,
            profile_id: profile.id,
            order_index: i as i32,
        };
        db::add_queue_item(queue_item)?;
    }

    Ok(format!(
        "Successfully created a new Queue - '{}'",
        queue.name
    ))
}

pub fn edit_queue() -> Result<String, eyre::Report> {
    let queues_list = db::get_queues()?;
    if queues_list.is_empty() {
        return Ok("There are no Queues to edit".to_string());
    }

    // Pick the queue to edit
    let queue_selection = inquire::Select::new("Pick the Queue to Edit:", queues_list)
        .with_page_size(tui::MENU_PAGE_SIZE)
        .prompt()?;

    // Name the new queue. Current name is the default
    let queue_name = inquire::Text::new("Enter a name for your Profile:")
        .with_validator(move |input: &str| {
            let queue_result = db::get_queue_by_name(input);
            if let Ok(queue) = queue_result {
                if queue.id != queue_selection.id {
                    return Ok(Validation::Invalid("Profile name already exists".into()));
                }
            }

            if input.len() < 5 {
                Ok(Validation::Invalid(
                    "Queue name must be at least 5 characters".into(),
                ))
            } else {
                Ok(Validation::Valid)
            }
        })
        .with_default(&queue_selection.name)
        .prompt()?;

    // Get the profiles in the queue
    let profiles = db::get_profile_display_list(data::ProfileOrder::Name)?;
    let queue_items = db::get_queue_items(queue_selection.id)?;
    // Need the indexes of the already selected profiles to use as defaults
    let default_profiles: Vec<usize> = queue_items
        .iter()
        .filter_map(|queue_item| {
            profiles
                .iter()
                .position(|profile| profile.id == queue_item.profile_id)
        })
        .collect();

    // Pick the profiles and order you would like them in to add to the queue
    let profile_selection = get_profile_selection(profiles, default_profiles)?;

    // Save the updated queue
    let queue = data::Queue {
        id: queue_selection.id,
        name: queue_name.clone(),
        date_created: queue_selection.date_created,
        date_edited: Utc::now(),
    };
    db::update_queue(queue)?;

    // Delete all existing queue items for this queue
    db::delete_all_queue_items(queue_selection.id)?;

    // Add them back in the new order
    for (i, profile) in profile_selection.iter().enumerate() {
        let queue_item = data::QueueItem {
            id: 0,
            profile_queue_id: queue_selection.id,
            profile_id: profile.id,
            order_index: i as i32,
        };
        db::add_queue_item(queue_item)?;
    }

    Ok(format!("Successfully updated Queue '{}' with {} Profiles", queue_name, profile_selection.len()))
}

pub fn delete_queue() -> Result<String, eyre::Report> {
    let queues = db::get_queues()?;
    if queues.is_empty() {
        return Ok("There are no Queues to delete".to_string());
    }

    let queue_selection = inquire::Select::new("Pick the Queue to Delete:", queues)
        .with_page_size(tui::MENU_PAGE_SIZE)
        .prompt_skippable()?;

    if let Some(queue) = queue_selection {
        if inquire::Confirm::new(&format!(
            "Are you sure you want to delete this Queue - '{}'? This cannot be undone",
            queue.name
        ))
        .with_default(false)
        .prompt()?
        {
            // Delete the queue items first
            db::delete_all_queue_items(queue.id).wrap_err(format!(
                "Failed to delete Queue Items for Queue - '{}",
                queue.name
            ))?;

            // Now delete the queue
            db::delete_queue(queue.id)
                .wrap_err(format!("Failed to delete Queue - '{}", queue.name))?;

            return Ok(format!("Successfully deleted Queue '{}'", queue.name));
        }
    }

    Ok("Canceled Queue deletion".to_string())
}

pub fn get_profile_selection(
    profiles: Vec<data::ProfileDisplay>,
    default_profiles: Vec<usize>,
) -> Result<Vec<data::ProfileDisplay>, eyre::Report> {
    // Multi-select which profiles to add
    let profile_selection =
        inquire::MultiSelect::new("Pick the Profiles you want to add:", profiles)
            .with_default(&default_profiles)
            .with_page_size(tui::MENU_PAGE_SIZE)
            .with_formatter(&|i| {
                i.iter()
                    .map(|e| e.value.simple_display())
                    .collect::<Vec<String>>()
                    .join(", ")
            })
            .prompt()?;

    // No ordering need, no items selected! (prefectly valid, you may want an empty queue)
    if profile_selection.is_empty() {
        return Ok(vec![]);
    }

    // No ordering needed, only one item selected
    if profile_selection.len() == 1 {
        return Ok(profile_selection);
    }

    // Pick the order you would like them in
    let ordered_items: Vec<data::ProfileDisplay> = loop {
        let mut ordered_items = Vec::with_capacity(profile_selection.len());
        let mut temp_items = profile_selection.clone();

        for i in 0..temp_items.len() {
            // FIXME: Could ESC from here and get a crash
            let selected = inquire::Select::new(
                &format!("Pick Profile #{} from your selected Profiles:", i + 1),
                temp_items.clone(),
            )
            .with_page_size(tui::MENU_PAGE_SIZE)
            .with_formatter(&|i| i.value.simple_display())
            .prompt()?;

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
            break ordered_items;
        }
    };

    Ok(ordered_items)
}

pub fn add_profile_to_queue() -> Result<String, eyre::Report> {
    let queues = db::get_queues()?;
    if queues.is_empty() {
        return Ok("There are no Queues to add a Profile to".to_string());
    };

    let profiles_list = db::get_profile_display_list(data::ProfileOrder::Name)?;
    if profiles_list.is_empty() {
        return Ok("There are no Profiles to add to a Queue".to_string());
    }

    // Which queue are we editing
    let queue_selection = inquire::Select::new("Pick the Queue to add a Profile to:", queues)
        .with_page_size(tui::MENU_PAGE_SIZE)
        .prompt()?;

    // Pick a single profile to add
    let profile_selection = inquire::Select::new("Pick the Profile to add:", profiles_list)
        .with_page_size(tui::MENU_PAGE_SIZE)
        .prompt()?;

    // Check this profile isn't already in the queue
    let queue_items = db::get_queue_items(queue_selection.id)?;
    if queue_items
        .iter()
        .any(|queue_item| queue_item.profile_id == profile_selection.id)
    {
        return Ok(format!(
            "Cannot add Profile '{}' to Queue '{}' since it already exists",
            profile_selection.name, queue_selection.name
        ));
    }

    // Determine the order index for this new queue item
    let highest_order = queue_items
        .iter()
        .map(|item| item.order_index)
        .max()
        .unwrap_or(0);

    // Save the new queue item
    let queue_item = data::QueueItem {
        id: 0,
        profile_queue_id: queue_selection.id,
        profile_id: profile_selection.id,
        order_index: highest_order + 1,
    };
    db::add_queue_item(queue_item)?;

    Ok(format!(
        "Successfully added Profile '{}' to Queue '{}'",
        profile_selection.name, queue_selection.name
    ))
}

pub fn delete_profile_from_queue() -> Result<String, eyre::Report> {
    let queues = db::get_queues()?;
    if queues.is_empty() {
        return Ok("There are no Queues to delete a Profile from".to_string());
    };

    // Which queue are we deleting from
    let queue_selection = inquire::Select::new("Pick the Queue to delete a Profile from:", queues)
        .with_page_size(tui::MENU_PAGE_SIZE)
        .prompt()?;

    // Get the queue items for the selected queue
    let queue_items = db::get_queue_items(queue_selection.id)?;
    if queue_items.is_empty() {
        return Ok(format!(
            "There are no Profiles in Queue '{}'",
            queue_selection.name
        ));
    }

    // Get the profiles in the queue
    let profile_display_items: Vec<data::ProfileDisplay> = queue_items
        .iter()
        .map(|queue_item| {
            db::get_profile_display_by_id(queue_item.profile_id).expect("Unable to get Profile")
        })
        .collect();

    // Pick the profile to remove
    let profile_selection =
        inquire::Select::new("Pick the Profile to delete:", profile_display_items)
            .with_page_size(tui::MENU_PAGE_SIZE)
            .with_formatter(&|i| i.value.simple_display())
            .prompt()?;

    if inquire::Confirm::new(&format!(
        "Are you sure you want to delete Profile '{}' from Queue '{}'?",
        profile_selection.simple_display(),
        queue_selection.name
    ))
    .with_default(false)
    .prompt()?
    {
        // Get the queue item to delete
        let selected_queue_item = queue_items
            .iter()
            .find(|queue_item| queue_item.profile_id == profile_selection.id)
            .unwrap();

        // Delete the queue item. Method will also fix the order indexes
        db::delete_queue_item(selected_queue_item)?;

        return Ok(format!(
            "Successfully deleted Profile '{}' from Queue '{}'",
            profile_selection.simple_display(),
            queue_selection.name
        ));
    }

    Ok("Canceled Profile deletion from Queue".to_string())
}

pub fn list_queues() -> Result<String, eyre::Report> {
    let queue_display_list =
        db::get_queue_display_list().wrap_err("Unable to generate Queue listing".to_string())?;

    if queue_display_list.is_empty() {
        return Ok("No Queues found".to_string());
    }

    let table = tabled::Table::new(queue_display_list)
        .with(Modify::new(Rows::new(1..)).with(Width::wrap(100)))
        .with(Style::modern())
        .to_string();
    Ok(table)
}
