use color_eyre::{eyre, owo_colors::OwoColorize};
use colored::Colorize;
use eyre::Context;
use inquire::{validator::Validation, InquireError};
use log::{debug, info};
use tabled::settings::{object::Rows, Modify, Rotate, Style, Width};

use crate::{
    data::{self},
    db, doom_data, files, paths, tui,
};

pub fn check_app_can_run(force: bool) -> Result<String, eyre::Report> {
    db::create_db()?;
    if !force && db::is_empty_app_settings_table()? {
        info!("{}", "No app settings found, running 'init'".red());
        init()?;
    }
    Ok("App is ready to run".to_string())
}

pub fn update_menu_mode() -> Result<String, eyre::Report> {
    let mut app_settings = db::get_app_settings()?;
    if app_settings.menu_mode == tui::MenuMode::Full {
        app_settings.menu_mode = tui::MenuMode::Simple;
    } else {
        app_settings.menu_mode = tui::MenuMode::Full;
    }

    db::save_app_settings(app_settings.clone())?;
    Ok(format!("Menu mode set to: {}", app_settings.menu_mode))
}

pub fn cli_update_menu_mode(menu_mode: tui::MenuMode) -> Result<String, eyre::Report> {
    let mut app_settings = db::get_app_settings()?;

    if app_settings.menu_mode == menu_mode {
        return Ok(format!(
            "Menu mode is already set to: {}",
            app_settings.menu_mode
        ));
    }

    app_settings.menu_mode = menu_mode;
    db::save_app_settings(app_settings.clone())?;
    Ok(format!("Menu mode set to: {}", app_settings.menu_mode))
}

pub fn init() -> Result<String, eyre::Report> {
    db::create_db()?;
    let mut app_settings = db::get_app_settings()?;

    info!("We'll ask you some questions, and then you'll be ready to go");

    let engine_search_folder = init_engines(
        &app_settings.engine_search_folder.unwrap_or("".to_string()),
        false,
    )?;

    let iwad_search_folder = match app_settings.iwad_search_folder {
        Some(iwad_search_folder) => iwad_search_folder,
        None => engine_search_folder.clone(),
    };
    let iwad_search_folder = init_iwads(&iwad_search_folder, false)?;

    let map_search_folder = match app_settings.map_search_folder {
        Some(map_search_folder) => map_search_folder,
        None => iwad_search_folder.clone(),
    };
    let map_search_folder = init_maps(&map_search_folder, false)?;

    // Update app_settings
    app_settings.engine_search_folder = Some(engine_search_folder);
    app_settings.iwad_search_folder = Some(iwad_search_folder);
    app_settings.map_search_folder = Some(map_search_folder);
    db::save_app_settings(app_settings)?;

    // Set default engine and iwad for quick play
    set_default_engine()?;
    set_default_iwad()?;

    // Completed init!
    info!("{}", "Successfully run init and app configured!".green());
    inquire::Text::new("Press any key to continue...").prompt_skippable()?;

    Ok("Successfully run init and app configured!".to_string())
}

pub fn cli_init(
    engine_path: &str,
    iwad_path: &str,
    map_path: Option<String>,
    force: bool,
) -> Result<String, eyre::Report> {
    // Check the paths exist
    if !paths::folder_exists(engine_path) {
        return Err(eyre::eyre!(format!(
            "Engine path does not exist: {}",
            engine_path
        )));
    }
    if !paths::folder_exists(iwad_path) {
        return Err(eyre::eyre!(format!(
            "IWAD path does not exist: {}",
            iwad_path
        )));
    }

    if let Some(path) = &map_path {
        if !paths::folder_exists(path) {
            return Err(eyre::eyre!(format!("Map path does not exist: {}", path)));
        }
    }

    // If Map is None then set to Iwad path
    let updated_map_path = match map_path {
        Some(path) => path,
        None => iwad_path.to_string(),
    };

    // Run individual init functions, need to amend to have a force/override option
    //  Force will skip any dialogs and will select all found files
    let engine_search_folder = init_engines(engine_path, force)?;
    let iwad_search_folder = init_iwads(iwad_path, force)?;
    let map_search_folder = init_maps(&updated_map_path, force)?;

    // Update app_settings
    let mut app_settings = db::get_app_settings()?;
    app_settings.engine_search_folder = Some(engine_search_folder);
    app_settings.iwad_search_folder = Some(iwad_search_folder);
    app_settings.map_search_folder = Some(map_search_folder);
    db::save_app_settings(app_settings)?;

    Ok("Successfully run init and app configured!".to_string())
}

pub fn init_engines(default_folder: &str, force: bool) -> Result<String, eyre::Report> {
    let engine_search_folder: String = if force {
        default_folder.to_string()
    } else {
        let path = inquire::Text::new("Folder to search for Engines:")
            .with_validator(|input: &str| {
                if paths::folder_exists(&paths::resolve_path(input)) {
                    Ok(Validation::Valid)
                } else {
                    Ok(Validation::Invalid("Folder does not exist".into()))
                }
            })
            .with_default(default_folder)
            .prompt()?;
        paths::resolve_path(&path)
    };

    // TODO: User filter for exists (what do you want to search for?)
    let doom_engine_list = doom_data::get_engine_list(doom_data::get_operating_system());
    let doom_engine_files = doom_engine_list
        .iter()
        .map(|e| e.exe_name.as_str())
        .collect::<Vec<&str>>();

    let engines_executables = paths::find_file_in_folders(&engine_search_folder, doom_engine_files);
    if engines_executables.is_empty() {
        return Err(eyre::eyre!(format!(
            "No Engine matches found using recursive search in folder - '{}'",
            &engine_search_folder
        )));
    }

    // Work out the indexes of what is already selected
    let db_engines = db::get_engines()?;
    let mut db_defaults = vec![];
    for (index, engine_executable) in engines_executables.iter().enumerate() {
        if db_engines.iter().any(|db| &db.path == engine_executable) {
            db_defaults.push(index);
        }
    }

    // Create a new list with version details
    let mut engines_extended: Vec<data::Engine> = Vec::new();
    for engine_executable in engines_executables {
        info!(
            "Getting version information for Engine: '{}'",
            engine_executable
        );
        let game_engine =
            files::get_game_engine_from_exe_name(doom_engine_list.clone(), &engine_executable)?;
        let file_version = files::get_version_from_exe_name(
            &engine_executable,
            game_engine.game_engine_type.clone(),
        )?;

        engines_extended.push(data::Engine {
            id: 0,
            app_name: file_version.app_name.clone(),
            path: engine_executable,
            internal_path: game_engine.internal_path.clone(),
            version: file_version.display_version(),
            game_engine_type: game_engine.game_engine_type,
        });
        info!("  {}", engines_extended.last().unwrap().blue().to_string());
    }
    //info!("Found engines: {:?}", engines_extended);

    // Multi-select prompt to user, can be skipped and all will be selected
    let selections = if force {
        engines_extended.clone()
    } else {
        inquire::MultiSelect::new("Pick the Engines you want to save:", engines_extended)
            .with_default(&db_defaults)
            .with_page_size(tui::MENU_PAGE_SIZE)
            .prompt()?
    };

    // Remove entries that were not selected but have entries in the database...
    // ...but only do that if the engine isn't linked to a profile
    for db_engine in &db_engines {
        if !selections
            .iter()
            .any(|e| e.path.to_lowercase() == db_engine.path.to_lowercase())
        {
            if db::is_engine_linked_to_profiles(db_engine.id)? {
                info!(
                    "  Cannot delete Engine as it is linked to one or more Profiles: '{}'",
                    db_engine.path
                );
                continue;
            }
            remove_engine_from_app_settings(db_engine.id)?;
            db::delete_engine(&db_engine.path)?;
            debug!("Deleted Engine: {:?}", db_engine);
        }
    }

    let mut count = 0;

    // Save engines to  engines table
    for selection in selections {
        let existing_engine = db_engines.iter().find(|e| e.path == selection.path);
        match existing_engine {
            Some(existing) => {
                info!(
                    "  Engine already exists, no need to add: {}",
                    selection.blue()
                );
                if existing.version != selection.version {
                    info!(
                        "  Updating Engine version from '{}' to '{}'",
                        existing.version, selection.version
                    );
                    db::update_engine_version(existing.id, &selection.version)?;
                }
            }
            None => {
                db::add_engine(&selection)?;
                debug!("Added Engine: {:?}", selection);
                info!("Added Engine: {}", selection.blue());
                count += 1;
            }
        }
    }

    // FIXME: This is getting blanked by menu display...
    // info!("{}", list_engines()?);
    if count > 0 {
        let result_message = format!("Successfully added {} Engines", count);
        info!("{}", result_message.green());
    }

    Ok(engine_search_folder)
}

pub fn init_iwads(default_folder: &str, force: bool) -> Result<String, eyre::Report> {
    // Search for IWADs
    // Use the same folder as the engines, but given option to change
    // Save to IWADs table
    let iwad_search_folder: String = if force {
        default_folder.to_string()
    } else {
        let path = inquire::Text::new("Folder to search for IWADs (Internal WAD files):")
            .with_validator(|input: &str| {
                if paths::folder_exists(&paths::resolve_path(input)) {
                    Ok(Validation::Valid)
                } else {
                    Ok(Validation::Invalid("Folder does not exist".into()))
                }
            })
            .with_default(default_folder)
            .prompt()?;
        paths::resolve_path(&path)
    };

    // TODO: User filter for exists (what do you want to search for?)
    let iwad_list = doom_data::get_internal_wad_list();
    let iwad_files = iwad_list
        .iter()
        .map(|e| e.file_name.as_str())
        .collect::<Vec<&str>>();

    let iwads = paths::find_file_in_folders(&iwad_search_folder, iwad_files);
    if iwads.is_empty() {
        return Err(eyre::eyre!(format!(
            "No IWAD matches found using recursive search in folder - '{}'",
            &iwad_search_folder
        )));
    }

    // Double check it's an IWAD (it should be, it's coming from the hardcoded InternalWad list)
    let mut confirmed_iwads: Vec<String> = vec![];
    for iwad in iwads {
        if !(files::is_iwad(&iwad)?) {
            info!("Skipping non-IWAD file: {}", iwad);
            continue;
        }
        confirmed_iwads.push(iwad);
    }

    // Work out the indexes of what is already selected
    let db_iwads = db::get_iwads()?;
    let mut db_defaults = vec![];
    for (index, iwad) in confirmed_iwads.iter().enumerate() {
        if db_iwads.iter().any(|db| &db.path == iwad) {
            db_defaults.push(index);
        }
    }

    // Multi-select prompt to user, can be skipped and all will be selected
    let selections = if force {
        confirmed_iwads.clone()
    } else {
        inquire::MultiSelect::new("Pick the IWADs you want to save:", confirmed_iwads)
            .with_default(&db_defaults)
            .with_page_size(tui::MENU_PAGE_SIZE)
            .prompt()?
    };

    // Remove entries that were not selected but have entries in the database
    for db_iwad in &db_iwads {
        // Check if the lowercase version of db_iwad.path exists in selections
        if !selections
            .iter()
            .any(|s| s.eq_ignore_ascii_case(&db_iwad.path))
        {
            if db::is_iwad_linked_to_profiles(db_iwad.id)? {
                info!(
                    "  Cannot delete IWAD as it is linked to one or more Profiles: '{}'",
                    db_iwad.path
                );
                continue;
            }
            remove_iwad_from_app_settings(db_iwad.id)?;
            db::delete_iwad(&db_iwad.path)?;
            debug!("Deleted iwad: {:?}", db_iwad);
        }
    }

    let mut count = 0;

    // Save engines to  engines table
    for selection in selections {
        let internal_wad_type =
            files::get_internal_wad_type_from_file_name(iwad_list.clone(), &selection)?;

        let existing_iwad = db_iwads.iter().find(|e| e.path == selection);

        match existing_iwad {
            Some(_) => {
                debug!("IWAD already exists, no need to add: {}", selection);
            }
            None => {
                let iwad = data::Iwad {
                    path: selection,
                    internal_wad_type,
                    id: 0,
                };

                db::add_iwad(&iwad)?;
                debug!("  IWAD: {:?}", iwad);
                info!(
                    "Added IWAD: {} - {}",
                    iwad.internal_wad_type.blue(),
                    iwad.path.blue()
                );
                count += 1;
            }
        }
    }

    //info!("{}", list_iwads()?);
    if count > 0 {
        let result_message = format!("Successfully added {} IWADs", count);
        info!("{}", result_message.green());
    }

    Ok(iwad_search_folder)
}

pub fn init_maps(default_folder: &str, force: bool) -> Result<String, eyre::Report> {
    let map_search_folder: String = if force {
        default_folder.to_string()
    } else {
        let paths = inquire::Text::new("Folder to search for Maps:")
            .with_validator(|input: &str| {
                if paths::folder_exists(&paths::resolve_path(input)) {
                    Ok(Validation::Valid)
                } else {
                    Ok(Validation::Invalid("Folder does not exist".into()))
                }
            })
            .with_default(default_folder)
            .prompt()?;
        paths::resolve_path(&paths)
    };

    let maps = paths::find_files_with_extensions_in_folders(
        &map_search_folder,
        doom_data::GAME_FILES.to_vec(),
    );
    if maps.is_empty() {
        return Err(eyre::eyre!(format!(
            "No Map matches found using recursive search in folder -'{}'",
            &map_search_folder
        )));
    }

    // Get what we have in the datanse
    let db_maps = db::get_maps()?;

    // Remove entries that were not selected but have entries in the database
    for db_map in &db_maps {
        if !maps.iter().any(|s| s.eq_ignore_ascii_case(&db_map.path)) {
            if db::is_map_linked_to_profiles(db_map.id)? {
                info!(
                    "  Cannot delete Map as it is linked to one or more Profiles: '{}'",
                    db_map.path
                );
                continue;
            }
            db::delete_map(&db_map.path)?;
            debug!("Deleted map: {:?}", db_map)
        }
    }

    let mut count = 0;

    for map in maps {
        if files::is_iwad(&map)? {
            info!("Skipping IWAD file: {}", &map.yellow());
            continue;
        }

        if !files::map_file_extension(&map)? {
            info!("Skipping invalid map file: {}", &map.yellow());
            continue;
        }

        let existing_map = db_maps.iter().find(|e| e.path == map);

        match existing_map {
            Some(_) => {
                debug!("Map already exists, no need to add: {}", map);
            }
            None => {
                info!("Getting map title and author for Map: '{}'", map);
                let (title, author) = files::get_details_from_readme(&map)?;
                let map = data::Map {
                    id: 0,
                    title,
                    author,
                    path: map.clone(),
                };

                db::add_map(&map)?;
                debug!("  Map {:?}", map);
                info!("Added Map: {} - {}", map.title.blue(), map.path.blue());

                count += 1;
            }
        }
    }

    // info!("{}", list_maps()?);
    if count > 0 {
        let result_message = format!("Successfully added {} Maps", count);
        info!("{}", result_message.green());
    }

    Ok(map_search_folder)
}

pub fn delete_engines() -> Result<String, eyre::Report> {
    let engine_list = db::get_engines()?;
    if engine_list.is_empty() {
        return Ok("There are no Engines to delete".to_string());
    }

    let engine_selection = inquire::Select::new("Pick the Engine to Delete:", engine_list)
        .with_page_size(tui::MENU_PAGE_SIZE)
        .prompt_skippable()?;

    if let Some(engine) = engine_selection {
        if db::is_engine_linked_to_profiles(engine.id)? {
            return Ok(format!(
                "Cannot delete Engine as it is linked to one or more Profiles: '{}'",
                engine.path
            ));
        }

        if inquire::Confirm::new(&format!(
            "Are you sure you want to delete this Engine - '{}'? This cannot be undone",
            engine.path
        ))
        .with_default(false)
        .prompt()?
        {
            remove_engine_from_app_settings(engine.id)?;

            db::delete_engine(&engine.path)
                .wrap_err(format!("Failed to delete Engine - '{}", engine))?;
            return Ok(format!("Successfully deleted Engine '{}'", engine));
        }
    }

    Ok("Canceled Engine deletion".to_string())
}

pub fn delete_iwads() -> Result<String, eyre::Report> {
    let iwad_list = db::get_iwads()?;
    if iwad_list.is_empty() {
        return Ok("There are no IWADs to delete".to_string());
    }

    let iwad_selection = inquire::Select::new("Pick the IWAD to Delete:", iwad_list)
        .with_page_size(tui::MENU_PAGE_SIZE)
        .prompt_skippable()?;

    if let Some(iwad) = iwad_selection {
        if db::is_iwad_linked_to_profiles(iwad.id)? {
            return Ok(format!(
                "Cannot delete IWAD as it is linked to one or more Profiles: '{}'",
                iwad.path
            ));
        }

        if inquire::Confirm::new(&format!(
            "Are you sure you want to delete this IWAD - '{}'? This cannot be undone",
            iwad.path
        ))
        .with_default(false)
        .prompt()?
        {
            remove_iwad_from_app_settings(iwad.id)?;

            db::delete_iwad(&iwad.path).wrap_err(format!("Failed to delete IWAD - '{}", iwad))?;
            return Ok(format!("Successfully deleted IWAD '{}'", iwad));
        }
    }

    Ok("Canceled IWAD deletion".to_string())
}

pub fn delete_maps() -> Result<String, eyre::Report> {
    let map_list = db::get_maps()?;
    if map_list.is_empty() {
        return Ok("There are no Maps to delete".to_string());
    }

    let map_selection = inquire::Select::new("Pick the Map to Delete:", map_list)
        .with_page_size(tui::MENU_PAGE_SIZE)
        .prompt_skippable()?;

    if let Some(map) = map_selection {
        if db::is_map_linked_to_profiles(map.id)? {
            return Ok(format!(
                "Cannot delete Map as it is linked to one or more Profiles: '{}'",
                map.path
            ));
        }

        if inquire::Confirm::new(&format!(
            "Are you sure you want to delete this Map - '{}'? This cannot be undone",
            map.path
        ))
        .with_default(false)
        .prompt()?
        {
            db::delete_map(&map.path).wrap_err(format!("Failed to delete Map - '{}", map))?;
            return Ok(format!("Successfully deleted Map '{}'", map));
        }
    }

    Ok("Canceled Map deletion".to_string())
}

pub fn list_engines() -> Result<String, eyre::Report> {
    let engines = db::get_engines().wrap_err("Unable to generate Engine listing".to_string())?;

    if engines.is_empty() {
        return Ok("No Engines found".to_string());
    }

    let table = tabled::Table::new(engines)
        .with(Modify::new(Rows::new(1..)).with(Width::wrap(50).keep_words()))
        .with(Style::modern())
        .to_string();
    Ok(table)
}

pub fn list_iwads() -> Result<String, eyre::Report> {
    let iwads = db::get_iwads().wrap_err("Unable to iwad listing".to_string())?;

    if iwads.is_empty() {
        return Ok("No IWADs found".to_string());
    }

    let table = tabled::Table::new(iwads)
        .with(Modify::new(Rows::new(1..)).with(Width::wrap(50).keep_words()))
        .with(Style::modern())
        .to_string();
    Ok(table)
}

pub fn list_maps() -> Result<String, eyre::Report> {
    let maps = db::get_maps().wrap_err("Unable to iwad listing".to_string())?;

    if maps.is_empty() {
        return Ok("No Maps found".to_string());
    }

    let table = tabled::Table::new(maps)
        .with(Modify::new(Rows::new(1..)).with(Width::wrap(50).keep_words()))
        .with(Style::modern())
        .to_string();
    Ok(table)
}

pub fn list_app_settings() -> Result<String, eyre::Report> {
    let app_settings = db::get_app_settings_display()
        .wrap_err("Unable to get App Settings listing".to_string())?;

    // It should just be one record. We use Rotate Left to move the Header
    // to be the first column. We then use Rotate Top to reorder the rows since
    // what was the first header becomes the bottom most row after Rotate Left!
    // (this Tabled crate is crazy!)
    let table = tabled::Table::new(vec![app_settings])
        .with(Modify::new(Rows::new(1..)).with(Width::wrap(50).keep_words()))
        .with(Rotate::Left)
        .with(Rotate::Top)
        .with(Style::modern())
        .to_string();
    Ok(table)
}

pub fn reset(force: bool) -> Result<String, eyre::Report> {
    if !db::database_exists() {
        return Ok("Canceled Reset. Database does not exist".to_string());
    }

    // Prompt the user for confirmation the reset, unless force is set
    if force
        || inquire::Confirm::new("Do you want to reset the database? All data will be deleted")
            .with_default(false)
            .prompt()?
    {
        db::reset_db().wrap_err("Failed to reset database")?;
        Ok("Successfully reset database".to_string())
    } else {
        Err(InquireError::OperationCanceled).wrap_err("Database reset not confirmed".to_string())
    }
}

pub fn set_default_engine() -> Result<String, eyre::Report> {
    let engine_list = db::get_engines()?;
    if engine_list.is_empty() {
        return Ok(
            "Cannot set Default Engine. There are no Engines found. Please add one".to_string(),
        );
    }

    let mut app_settings = db::get_app_settings()?;
    let starting_cursor = match app_settings.default_engine_id {
        Some(ref i) => engine_list.iter().position(|x| x.id == *i).unwrap(),
        None => 0,
    };

    let engine = inquire::Select::new("Pick the Engine to mark as Default:", engine_list)
        .with_starting_cursor(starting_cursor)
        .with_page_size(tui::MENU_PAGE_SIZE)
        .prompt_skippable()?;

    match engine {
        Some(engine) => {
            app_settings.default_engine_id = Some(engine.id);
            db::save_app_settings(app_settings).wrap_err("Failed to set Default Engine")?;
            Ok(format!("Marked Engine '{}' as Default", engine))
        }
        None => Ok("No changes made to setting Engine as Default".to_string()),
    }
}

pub fn cli_set_default_engine(path: &str) -> Result<String, eyre::Report> {
    let engine = db::get_engine_by_path(path).wrap_err("Unable to set Default Engine")?;

    let mut app_settings = db::get_app_settings()?;
    app_settings.default_engine_id = Some(engine.id);
    db::save_app_settings(app_settings).wrap_err("Failed to set Default Engine")?;
    Ok(format!("Successfully set Engine '{}' as Default", engine))
}

pub fn cli_set_default_iwad(path: &str) -> Result<String, eyre::Report> {
    let iwad = db::get_iwad_by_path(path)?;

    let mut app_settings = db::get_app_settings()?;
    app_settings.default_iwad_id = Some(iwad.id);
    db::save_app_settings(app_settings).wrap_err("Failed to set Default IWAD")?;
    Ok(format!("Successfully set IWAD '{}' as Default", iwad))
}

pub fn set_default_iwad() -> Result<String, eyre::Report> {
    let iwad_list = db::get_iwads()?;
    if iwad_list.is_empty() {
        return Ok("Cannot set Default IWAD. There are no IWADs found. Please add one".to_string());
    }

    let mut app_settings = db::get_app_settings()?;
    let starting_cursor = match app_settings.default_iwad_id {
        Some(ref i) => iwad_list.iter().position(|x| x.id == *i).unwrap(),
        None => 0,
    };

    let iwad = inquire::Select::new("Pick the IWAD to mark as Default:", iwad_list)
        .with_starting_cursor(starting_cursor)
        .with_page_size(tui::MENU_PAGE_SIZE)
        .prompt_skippable()?;

    match iwad {
        Some(iwad) => {
            app_settings.default_iwad_id = Some(iwad.id);
            db::save_app_settings(app_settings).wrap_err("Failed to set Default IWAD")?;
            Ok(format!("Marked IWAD '{}' as Default", iwad))
        }
        None => Ok("No changes made to setting IWAD as Default".to_string()),
    }
}

pub fn remove_profile_from_app_settings(profile_id: i32) -> Result<String, eyre::Report> {
    // TODO: Make this "profile in use" check less ugly
    let mut app_settings = db::get_app_settings()?;
    let mut default_profile_tidied = false;
    if let Some(id) = app_settings.default_profile_id {
        if id == profile_id {
            app_settings.default_profile_id = None;
            default_profile_tidied = true;
        }
    }
    let mut last_profile_tidied = false;
    if let Some(id) = app_settings.last_profile_id {
        if id == profile_id {
            app_settings.last_profile_id = None;
            last_profile_tidied = true;
        }
    }
    if default_profile_tidied || last_profile_tidied {
        db::save_app_settings(app_settings).wrap_err("Failed to remove Default Profile")?;
        return Ok("Successfully removed Profile from App Settings".to_string());
    }

    Ok("".to_string())
}

fn remove_engine_from_app_settings(engine_id: i32) -> Result<String, eyre::Report> {
    let mut app_settings = db::get_app_settings()?;
    if let Some(id) = app_settings.default_engine_id {
        if id == engine_id {
            app_settings.default_engine_id = None;
            db::save_app_settings(app_settings).wrap_err("Failed to remove Default Engine")?;
            return Ok("Successfully removed Engine from App Settings".to_string());
        }
    }

    Ok("".to_string())
}

fn remove_iwad_from_app_settings(iwad_id: i32) -> Result<String, eyre::Report> {
    let mut app_settings = db::get_app_settings()?;
    if let Some(id) = app_settings.default_iwad_id {
        if id == iwad_id {
            app_settings.default_iwad_id = None;
            db::save_app_settings(app_settings).wrap_err("Failed to remove Default IWAD")?;
            return Ok("Successfully removed IWAD from App Settings".to_string());
        }
    }

    Ok("".to_string())
}

pub fn remove_editor_from_app_settings(editor_id: i32) -> Result<String, eyre::Report> {
    let mut app_settings = db::get_app_settings()?;
    if let Some(id) = app_settings.default_editor_id {
        if id == editor_id {
            app_settings.default_editor_id = None;
            db::save_app_settings(app_settings).wrap_err("Failed to remove Default Editor")?;
            return Ok("Successfully removed Editor from App Settings".to_string());
        }
    }

    Ok("".to_string())
}
