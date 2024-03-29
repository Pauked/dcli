use chrono::Utc;
use color_eyre::{
    eyre::{self},
    Result,
};
use owo_colors::{colors::xterm, OwoColorize};
use uuid::Uuid;

use crate::{
    data::{self, ProfileDisplay},
    db, paths, runner, tui,
};

pub fn get_default_profile_text() -> Result<String, eyre::Report> {
    if !db::database_exists() {
        return Ok("No database found. Please run 'init'".to_string());
    }

    if db::is_empty_app_settings_table()? {
        return Ok("No settings configured. Please run 'init'".to_string());
    }

    let app_settings = db::get_app_settings()?;

    if app_settings.default_profile_id.is_none() {
        return Ok(format!("Default - {}", "Please set one".yellow()));
    }

    let profile_display = db::get_profile_display_by_id(app_settings.default_profile_id.unwrap())?;
    Ok(format!(
        "Default - {}",
        profile_display.simple_display().fg::<xterm::LochmaraBlue>()
    ))
}

pub fn get_last_profile_text() -> Result<String, eyre::Report> {
    if db::is_empty_app_settings_table()? {
        return Ok("No settings configured. Please run 'init'".to_string());
    }

    let app_settings = db::get_app_settings()?;
    if app_settings.last_profile_id.is_none() {
        return Ok(format!(
            "Last    - {}",
            "Run a profile to set as last run".yellow()
        ));
    }

    let profile_display = db::get_profile_display_by_id(app_settings.last_profile_id.unwrap())?;
    Ok(format!(
        "Last    - {}",
        profile_display.simple_display().fg::<xterm::TennOrange>()
    ))
}

pub fn play_default_profile() -> Result<String, eyre::Report> {
    let app_settings = db::get_app_settings()?;

    if app_settings.default_profile_id.is_none() {
        return Ok("No Default Profile found. Please set one".to_string());
    };

    runner::play_from_profile(app_settings.default_profile_id.unwrap(), false)
}

pub fn play_last_profile() -> Result<String, eyre::Report> {
    let app_settings = db::get_app_settings()?;

    if app_settings.last_profile_id.is_none() {
        return Ok("No Last Run Profile found. Run a profile to make it the last run".to_string());
    };

    runner::play_from_profile(app_settings.last_profile_id.unwrap(), true)
}

pub fn pick_and_play_profile_on_name() -> Result<String, eyre::Report> {
    pick_and_play_profile(db::get_profile_display_list(data::ProfileOrder::Name)?)
}

pub fn pick_and_play_profile_on_date_last_run() -> Result<String, eyre::Report> {
    pick_and_play_profile(db::get_profile_display_list(
        data::ProfileOrder::DateLastRun,
    )?)
}

fn pick_and_play_profile(profile_list: Vec<ProfileDisplay>) -> Result<String, eyre::Report> {
    if profile_list.is_empty() {
        return Ok(
            "Cannot Play Profile, there are no profiles found. Please create one".to_string(),
        );
    }
    let profile = inquire::Select::new("Pick the Profile you want to Play:", profile_list)
        .with_page_size(tui::MENU_PAGE_SIZE)
        .with_formatter(&|i| i.value.simple_display())
        .prompt_skippable()?;

    match profile {
        Some(profile) => runner::play_from_profile(profile.id, true),
        None => Ok("No profile selected".to_string()),
    }
}

pub fn cli_play_selected_profile(profile_name: &str) -> Result<String, eyre::Report> {
    let profile = db::get_profile_by_name(profile_name)?;
    runner::play_from_profile(profile.id, true)
}

pub fn pick_and_play_map() -> Result<String, eyre::Report> {
    let engine_list = db::get_engines()?;
    if engine_list.is_empty() {
        return Ok("There are no Engines to select. Please run 'init'".to_string());
    }
    let iwad_list = db::get_iwads()?;
    if iwad_list.is_empty() {
        return Ok("There are no IWADs to select. Please run 'init".to_string());
    }
    let map_list = db::get_maps()?;
    if map_list.is_empty() {
        return Ok("There are no Maps to select. Please run 'init'".to_string());
    }

    let app_settings = db::get_app_settings()?;

    let starting_cursor = match app_settings.default_engine_id {
        Some(ref i) => engine_list.iter().position(|x| x.id == *i).unwrap(),
        None => 0,
    };

    let engine_selection = inquire::Select::new("Pick the Engine you want to use:", engine_list)
        .with_starting_cursor(starting_cursor)
        .with_page_size(tui::MENU_PAGE_SIZE)
        .with_formatter(&|i| i.value.simple_display())
        .prompt()?;

    // let engine_selection = {
    //     if let Some(engine_id) = app_settings.default_engine_id {
    //         let engine =
    //             db::get_engine_by_id(engine_id).wrap_err("Unable to get Default Engine")?;
    //         info!("Using Default Engine: {}", engine.blue());
    //         engine
    //     } else {
    //         inquire::Select::new("Pick the Engine you want to use:", engine_list)
    //             .with_page_size(tui::MENU_PAGE_SIZE)
    //             .prompt()?
    //     }
    // };

    let starting_cursor = match app_settings.default_iwad_id {
        Some(ref i) => iwad_list.iter().position(|x| x.id == *i).unwrap(),
        None => 0,
    };

    let iwad_selection = inquire::Select::new("Pick the IWAD you want to use:", iwad_list)
        .with_starting_cursor(starting_cursor)
        .with_page_size(tui::MENU_PAGE_SIZE)
        .with_formatter(&|i| i.value.simple_display())
        .prompt()?;

    // let iwad_selection = {
    //     if let Some(iwad_id) = app_settings.default_iwad_id {
    //         let iwad = db::get_iwad_by_id(iwad_id).wrap_err("Unable to get Default IWAD")?;
    //         info!("Using Default IWAD: {}", iwad.blue());
    //         iwad
    //     } else {
    //         inquire::Select::new("Pick the IWAD you want to use:", iwad_list)
    //             .with_page_size(tui::MENU_PAGE_SIZE)
    //             .prompt()?
    //     }
    // };

    // Yes this is ONE Map only. Profiles can have up to 5 Maps, but this is just a quick play option.
    let map_selection =
        inquire::Select::new("Pick the Map you want to use (optional):", map_list.clone())
            .with_page_size(tui::MENU_PAGE_SIZE)
            .with_formatter(&|i| i.value.simple_display())
            .prompt_skippable()?;
    let map_id = map_selection.as_ref().map(|map| map.id);

    let save_game =
        inquire::Text::new("Enter save game file name you want to automatically load (optional):")
            .with_help_message("For example with GZDoom 'save26.zds'")
            .prompt_skippable()?;

    let additional_arguments =
        inquire::Text::new("Enter any additional arguments (optional):").prompt_skippable()?;

    if inquire::Confirm::new("Autosave these options as a Profile?")
        .with_default(false)
        .prompt()?
    {
        let wad_name = match map_selection {
            None => paths::extract_file_name(&iwad_selection.path),
            Some(map) => map.title,
        };
        let profile_name = format!(
            "Autosave-{} '{}'",
            &Uuid::new_v4().to_string()[..8],
            wad_name
        );

        let profile = data::Profile {
            id: 0,
            name: profile_name,
            engine_id: Some(engine_selection.id),
            iwad_id: Some(iwad_selection.id),
            map_id,
            map_id2: None,
            map_id3: None,
            map_id4: None,
            map_id5: None,
            save_game,
            additional_arguments,
            date_created: Utc::now(),
            date_edited: Utc::now(),
            date_last_run: None,
            run_count: 0,
        };
        let add_result = db::add_profile(profile)?;
        let add_profile_id: i32 = add_result.last_insert_rowid().try_into().unwrap();

        runner::play_from_profile(add_profile_id, true)
    } else {
        runner::play_from_engine_iwad_and_map(
            engine_selection.id,
            iwad_selection.id,
            data::map_ids_from_options(map_id, None, None, None, None),
            save_game,
            additional_arguments,
        )
    }
}

pub fn play_queue_top() -> Result<String, eyre::Report> {
    let queue_display_list = db::get_queue_display_list()?;
    if queue_display_list.is_empty() {
        return Ok("There are no Queues to select".to_string());
    }

    let queue_selection =
        inquire::Select::new("Pick the Queue you want to Play from:", queue_display_list)
            .with_formatter(&|i| i.value.simple_display())
            .with_page_size(tui::MENU_PAGE_SIZE)
            .prompt()?;

    let queue_items = db::get_queue_items(queue_selection.id)?;
    if queue_items.is_empty() {
        return Ok("There are no Profiles to pick from".to_string());
    }

    let queue_top = queue_items.first().unwrap();
    runner::play_from_profile(queue_top.profile_id, true)
}

pub fn pick_and_play_queue() -> Result<String, eyre::Report> {
    let queue_display_list = db::get_queue_display_list()?;
    if queue_display_list.is_empty() {
        return Ok("There are no Queues to select".to_string());
    }

    let queue_selection =
        inquire::Select::new("Pick the Queue you want to Play from:", queue_display_list)
            .with_formatter(&|i| i.value.simple_display())
            .with_page_size(tui::MENU_PAGE_SIZE)
            .prompt()?;

    let queue_items = db::get_queue_items(queue_selection.id)?;
    if queue_items.is_empty() {
        return Ok("There are no Profiles to pick from".to_string());
    }

    // Build a list of ProfileDisplay from the QueueItems. Keep it in order_index order.
    let mut selected_profiles: Vec<ProfileDisplay> = Vec::new();
    for queue_item in queue_items {
        let profile = db::get_profile_display_by_id(queue_item.profile_id)?;
        selected_profiles.push(profile);
    }

    let queue_selection =
        inquire::Select::new("Pick the Profile you want to Play:", selected_profiles)
            .with_page_size(tui::MENU_PAGE_SIZE)
            .prompt()?;

    runner::play_from_profile(queue_selection.id, true)
}
