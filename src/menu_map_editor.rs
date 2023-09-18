use colored::Colorize;
use eyre::Context;
use inquire::validator::Validation;
use log::{debug, info};
use tabled::settings::{object::Rows, Modify, Style, Width};

use crate::{data, db, finder, menu_common, paths, runner, tui};

async fn open_map_editor_from_pwad_id(pwad_id: i32) -> Result<String, eyre::Report> {
    let pwad = db::get_pwad_by_id(pwad_id)
        .await
        .wrap_err(format!("Unable to get PWAD for id '{}'", pwad_id).to_string())?;

    // Use the Active Map Editor if set
    let app_settings = db::get_app_settings().await?;
    if app_settings.active_map_editor_id.is_some() {
        let map_editor = db::get_map_editor_by_id(app_settings.active_map_editor_id.unwrap())
            .await
            .wrap_err("Unable to get Active Map Editor")?;
        return runner::map_editor(&pwad.path, map_editor);
    }

    // Otherwise, try select map editor...
    let map_editor_list = db::get_map_editors().await?;
    if map_editor_list.is_empty() {
        return Ok("There are no Map Editors to select from.".red().to_string());
    }

    let map_editor = inquire::Select::new("Pick the Map Editor to use:", map_editor_list)
        .with_page_size(tui::MENU_PAGE_SIZE)
        .prompt_skippable()?;

    match map_editor {
        Some(map_editor) => runner::map_editor(&pwad.path, map_editor),
        None => Ok("Cancelled opening Map Editor.".yellow().to_string()),
    }
}

pub async fn open_from_active_profile() -> Result<String, eyre::Report> {
    let pwad_id =
        menu_common::get_pwad_id_from_from_active_profile("Cannot open Map Editor.").await?;

    open_map_editor_from_pwad_id(pwad_id).await
}

pub async fn open_from_last_profile() -> Result<String, eyre::Report> {
    let pwad_id =
        menu_common::get_pwad_id_from_from_last_profile("Cannot open Map Editor.").await?;

    open_map_editor_from_pwad_id(pwad_id).await
}

pub async fn open_from_pick_profile() -> Result<String, eyre::Report> {
    let pwad_id = menu_common::get_pwad_id_from_pick_profile(
        "Pick the Profile to open in Map Editor:",
        "Cancelled opening Map Editor.",
    )
    .await?;

    open_map_editor_from_pwad_id(pwad_id).await
}

pub async fn open_from_pick_pwad() -> Result<String, eyre::Report> {
    let pwad_id = menu_common::get_pwad_id_from_pick_pwad(
        "Pick the PWAD to open in Map Editor:",
        "Cancelled opening Map Editor.",
    )
    .await?;

    open_map_editor_from_pwad_id(pwad_id).await
}

pub async fn set_active_map_editor() -> Result<String, eyre::Report> {
    let map_editor_list = db::get_map_editors().await?;
    if map_editor_list.is_empty() {
        return Ok(
            "Cannot set Active Map Editor. There are no Map Editors found. Please add one."
                .red()
                .to_string(),
        );
    }

    // Try to get the current active map eidtor
    let mut app_settings = db::get_app_settings().await?;
    let starting_cursor = match app_settings.active_map_editor_id {
        Some(ref s) => map_editor_list.iter().position(|x| x.id == *s).unwrap(),
        None => 0,
    };

    let map_editor =
        inquire::Select::new("Pick the Map Editor to mark as Active:", map_editor_list)
            .with_starting_cursor(starting_cursor)
            .prompt_skippable()?;

    match map_editor {
        Some(map_editor) => {
            app_settings.active_map_editor_id = Some(map_editor.id);
            db::save_app_settings(app_settings)
                .await
                .wrap_err("Failed to set Active Map Editor")?;
            Ok(format!("Marked Map Editor '{}' as Active", map_editor))
        }
        None => Ok("No changes made to setting Map Editor as Active".to_string()),
    }
}

pub async fn list_map_editors() -> Result<String, eyre::Report> {
    let map_editors = db::get_map_editors()
        .await
        .wrap_err("Unable to generate Map Editor listing".to_string())?;

    let table = tabled::Table::new(map_editors)
        .with(Modify::new(Rows::new(1..)).with(Width::wrap(50).keep_words()))
        .with(Style::modern())
        .to_string();
    Ok(table)
}

pub async fn update_map_editors() -> Result<String, eyre::Report> {
    let app_settings = db::get_app_settings().await?;

    let default_folder = app_settings
        .map_editor_search_folder
        .unwrap_or("".to_string());

    let map_editor_executable_name: String = inquire::Text::new("Executable name of Map Editor:")
        .with_help_message("Just the file name, not the full path. E.g. 'builder.exe'")
        .prompt()?;

    let map_editor_search_folder: String = inquire::Text::new("Folder to search for Map Editor:")
        .with_validator(|input: &str| {
            if paths::folder_exists(input) {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid("Folder does not exist.".into()))
            }
        })
        .with_default(&default_folder)
        .prompt()?;

    let map_editors =
        paths::find_file_in_folders(&map_editor_search_folder, vec![&map_editor_executable_name]);
    if map_editors.is_empty() {
        return Err(eyre::eyre!(format!(
            "No matches found using recursive search in folder '{}'",
            &map_editor_search_folder
        )));
    }

    let load_file_argument = inquire::Text::new("Load file argument:")
        .with_help_message(
            "Your selected Map Editor may require an argument to load a file. E.g. '-file'",
        )
        .prompt_skippable()?;

    let additional_arguments = inquire::Text::new("Additional arguments:")
        .with_help_message("Any additional arguments you want to pass to the Map Editor")
        .prompt_skippable()?;

    // Work out the indexes of what is already selected
    let db_map_editors = db::get_map_editors().await?;
    let mut db_defaults = vec![];
    for (index, map_editor) in map_editors.iter().enumerate() {
        if db_map_editors.iter().any(|db| &db.path == map_editor) {
            db_defaults.push(index);
        }
    }

    // Create a new list with version details
    let mut map_editors_extended: Vec<data::MapEditor> = Vec::new();
    for map_editor in map_editors {
        info!(
            "Getting version information for Map Editor: '{}'",
            map_editor
        );
        let file_version = finder::get_file_version(&map_editor)?;

        map_editors_extended.push(data::MapEditor {
            id: 0,
            app_name: file_version.app_name.clone(),
            path: map_editor,
            version: file_version.display_version(),
            load_file_argument: load_file_argument.clone(),
            additional_arguments: additional_arguments.clone(),
        });
        info!(
            "Done - {}",
            map_editors_extended.last().unwrap().to_string()
        );
    }

    // Multi-select prompt to user
    let selections = inquire::MultiSelect::new(
        "Pick the Map Editors you want to save:",
        map_editors_extended,
    )
    .with_default(&db_defaults)
    .with_page_size(tui::MENU_PAGE_SIZE)
    .prompt()?;

    // Remove entries that were not selected but have entries in the database
    // for db_engine in &db_engines {
    //     if !selections.iter().any(|e| e.path == db_engine.path) {
    //         db::delete_engine(&db_engine.path).await?;
    //         debug!("Deleted engine: {:?}", db_engine);
    //     }
    // }

    // Save engines to  engines table
    for selection in selections {
        let existing_map_editor = db_map_editors.iter().find(|e| e.path == selection.path);
        match existing_map_editor {
            Some(existing) => {
                debug!("Map Editor already exists, no need to add: {}", selection);
                if existing.version != selection.version {
                    debug!(
                        "Updating Map Editor version from '{}' to '{}'",
                        existing.version, selection.version
                    );
                    db::update_map_editor_version(existing.id, &selection.version).await?;
                }
            }
            None => {
                db::add_map_editor(&selection).await?;
                debug!("Added Map Editor: {:?}", selection);
            }
        }
    }

    // FIXME: This is getting blanked by menu display...
    info!("{}", list_map_editors().await?);

    // Save the updated app setting
    let mut app_settings = db::get_app_settings().await?;
    app_settings.map_editor_search_folder = Some(map_editor_search_folder);
    db::save_app_settings(app_settings).await?;
    inquire::Text::new("Press any key to continue...").prompt_skippable()?;

    Ok("Successfully updated Map Editors".to_string())
}

pub async fn delete_map_editor() -> Result<String, eyre::Report> {
    let map_editor_list = db::get_map_editors().await?;
    if map_editor_list.is_empty() {
        return Ok("There are no Map Editors to delete.".red().to_string());
    }

    let map_editor = inquire::Select::new("Pick the Map Editor to Delete:", map_editor_list)
        .prompt_skippable()?;

    if map_editor.is_some() {
        let map_editor = map_editor.unwrap();
        if inquire::Confirm::new(&format!(
            "Are you sure you want to delete this Map Editor - '{}'? This cannot be undone.",
            map_editor.app_name
        ))
        .with_default(false)
        .prompt()
        .unwrap()
        {
            // TODO: Check if "active map editor" and remove link if so
            db::delete_map_editor(map_editor.id)
                .await
                .wrap_err(format!("Failed to delete Map Editor - '{}", map_editor))?;
            return Ok(format!("Successfully deleted Map Editor '{}'", map_editor));
        }
    }

    Ok("Cancelled Map Editor deletion.".yellow().to_string())
}
