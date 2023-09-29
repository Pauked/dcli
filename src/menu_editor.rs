use eyre::Context;
use inquire::validator::Validation;
use log::{debug, info};
use tabled::settings::{object::Rows, Modify, Style, Width};

use crate::{data, db, finder, menu_app_settings, menu_common, paths, runner, tui};

fn open_editor_from_map_id(map_id: i32) -> Result<String, eyre::Report> {
    let map = db::get_map_by_id(map_id)
        .wrap_err(format!("Unable to get Map for id '{}'", map_id).to_string())?;

    // Use the Default Editor if set
    let app_settings = db::get_app_settings()?;
    if let Some(editor_id) = app_settings.default_editor_id {
        let editor = db::get_editor_by_id(editor_id).wrap_err("Unable to get Default Editor")?;
        return runner::editor(&map.path, editor);
    }

    // Otherwise, try select editor...
    let editor_list = db::get_editors()?;
    if editor_list.is_empty() {
        return Ok("There are no Editors to select from".to_string());
    }

    let editor = inquire::Select::new("Pick the Editor to use:", editor_list)
        .with_page_size(tui::MENU_PAGE_SIZE)
        .prompt_skippable()?;

    match editor {
        Some(editor) => runner::editor(&map.path, editor),
        None => Ok("Canceled opening Editor".to_string()),
    }
}

pub fn open_from_default_profile() -> Result<String, eyre::Report> {
    if db::get_editor_count()? == 0 {
        return Ok("There are no Editors to select from".to_string());
    }
    let map_id = menu_common::get_map_id_from_from_default_profile("Cannot open Editor")?;

    open_editor_from_map_id(map_id)
}

pub fn open_from_last_profile() -> Result<String, eyre::Report> {
    if db::get_editor_count()? == 0 {
        return Ok("There are no Editors to select from".to_string());
    }
    let map_id = menu_common::get_map_id_from_from_last_profile("Cannot open Editor")?;

    open_editor_from_map_id(map_id)
}

pub fn open_from_pick_profile() -> Result<String, eyre::Report> {
    if db::get_editor_count()? == 0 {
        return Ok("There are no Editors to select from".to_string());
    }
    let map_id = menu_common::get_map_id_from_pick_profile(
        "Pick the Profile to open in Editor:",
        "Canceled opening Editor",
    )?;

    open_editor_from_map_id(map_id)
}

pub fn open_from_pick_map() -> Result<String, eyre::Report> {
    if db::get_editor_count()? == 0 {
        return Ok("There are no Editors to select from".to_string());
    }
    let map_id = menu_common::get_map_id_from_pick_map(
        "Pick the Map to open in Editor:",
        "Canceled opening Editor",
    )?;

    open_editor_from_map_id(map_id)
}

pub fn set_default_editor() -> Result<String, eyre::Report> {
    let editor_list = db::get_editors()?;
    if editor_list.is_empty() {
        return Ok(
            "Cannot set Default Editor. There are no Editors found. Please add one".to_string(),
        );
    }

    let mut app_settings = db::get_app_settings()?;
    let starting_cursor = match app_settings.default_editor_id {
        Some(ref i) => editor_list.iter().position(|x| x.id == *i).unwrap(),
        None => 0,
    };

    let editor = inquire::Select::new("Pick the Editor to mark as Default:", editor_list)
        .with_starting_cursor(starting_cursor)
        .with_page_size(tui::MENU_PAGE_SIZE)
        .prompt_skippable()?;

    match editor {
        Some(editor) => {
            app_settings.default_editor_id = Some(editor.id);
            db::save_app_settings(app_settings).wrap_err("Failed to set Default Editor")?;
            Ok(format!("Marked Editor '{}' as Default", editor))
        }
        None => Ok("No changes made to setting Editor as Default".to_string()),
    }
}

pub fn list_editors() -> Result<String, eyre::Report> {
    let editors = db::get_editors().wrap_err("Unable to generate Editor listing".to_string())?;

    if editors.is_empty() {
        return Ok("There are no Editors to list".to_string());
    }

    let table = tabled::Table::new(editors)
        .with(Modify::new(Rows::new(1..)).with(Width::wrap(50).keep_words()))
        .with(Style::modern())
        .to_string();
    Ok(table)
}

pub fn add_editor() -> Result<String, eyre::Report> {
    let app_settings = db::get_app_settings()?;

    let default_folder = app_settings.editor_search_folder.unwrap_or("".to_string());

    let editor_executable_name: String = inquire::Text::new("Executable name of Editor:")
        .with_help_message("Just the file name, not the full path. E.g. 'builder.exe'")
        .prompt()?;

    let editor_search_folder: String = {
        let path = inquire::Text::new("Folder to search for Editor:")
            .with_validator(|input: &str| {
                if paths::folder_exists(&paths::resolve_path(input)) {
                    Ok(Validation::Valid)
                } else {
                    Ok(Validation::Invalid("Folder does not exist".into()))
                }
            })
            .with_default(&default_folder)
            .prompt()?;
        paths::resolve_path(&path)
    };

    let editors = paths::find_file_in_folders(&editor_search_folder, vec![&editor_executable_name]);
    if editors.is_empty() {
        return Err(eyre::eyre!(format!(
            "No Editor matches found using recursive search in folder - '{}'",
            &editor_search_folder
        )));
    }

    let load_file_argument = inquire::Text::new("Load file argument:")
        .with_help_message(
            "Your selected Editor may require an argument to load a file. E.g. '-file'",
        )
        .prompt_skippable()?;

    let additional_arguments = inquire::Text::new("Additional arguments:")
        .with_help_message("Any additional arguments you want to pass to the Editor")
        .prompt_skippable()?;

    // Work out the indexes of what is already selected
    let db_editors = db::get_editors()?;
    let mut db_defaults = vec![];
    for (index, editor) in editors.iter().enumerate() {
        if db_editors.iter().any(|db| &db.path == editor) {
            db_defaults.push(index);
        }
    }

    // Create a new list with version details
    let mut editors_extended: Vec<data::Editor> = Vec::new();
    for editor in editors {
        info!("Getting version information for Editor: '{}'", editor);
        let file_version = finder::get_file_version(&editor)?;

        editors_extended.push(data::Editor {
            id: 0,
            app_name: file_version.app_name.clone(),
            path: editor,
            version: file_version.display_version(),
            load_file_argument: load_file_argument.clone(),
            additional_arguments: additional_arguments.clone(),
        });
        info!("Done - {}", editors_extended.last().unwrap().to_string());
    }

    // Multi-select prompt to user
    let selections =
        inquire::MultiSelect::new("Pick the Editors you want to save:", editors_extended)
            .with_default(&db_defaults)
            .with_page_size(tui::MENU_PAGE_SIZE)
            .prompt()?;

    // Remove entries that were not selected but have entries in the database
    for db_editor in &db_editors {
        if !selections
            .iter()
            .any(|e| e.path.to_lowercase() == db_editor.path.to_lowercase())
        {
            menu_app_settings::remove_editor_from_app_settings(db_editor.id)?;
            db::delete_editor(db_editor.id)?;
            debug!("Deleted editor: {:?}", db_editor);
        }
    }

    // Save engines to  engines table
    for selection in selections {
        let editor = db_editors.iter().find(|e| e.path == selection.path);
        match editor {
            Some(existing) => {
                debug!("Editor already exists, no need to add: {}", selection);
                if existing.version != selection.version {
                    debug!(
                        "Updating Editor version from '{}' to '{}'",
                        existing.version, selection.version
                    );
                    db::update_editor_version(existing.id, &selection.version)?;
                }
            }
            None => {
                db::add_editor(&selection)?;
                debug!("Added Editor: {:?}", selection);
            }
        }
    }

    // FIXME: This is getting blanked by menu display...
    info!("{}", list_editors()?);

    // Save the updated app setting
    let mut app_settings = db::get_app_settings()?;
    app_settings.editor_search_folder = Some(editor_search_folder);
    db::save_app_settings(app_settings)?;
    inquire::Text::new("Press any key to continue...").prompt_skippable()?;

    Ok("Successfully updated Editors".to_string())
}

pub fn delete_editor() -> Result<String, eyre::Report> {
    let editor_list = db::get_editors()?;
    if editor_list.is_empty() {
        return Ok("There are no Editors to delete".to_string());
    }

    let editor_selection =
        inquire::Select::new("Pick the Editor to Delete:", editor_list).prompt_skippable()?;

    if let Some(editor) = editor_selection {
        if inquire::Confirm::new(&format!(
            "Are you sure you want to delete this Editor - '{}'? This cannot be undone",
            editor.app_name
        ))
        .with_default(false)
        .prompt()?
        {
            // Check if "Default Editor" and remove link if so
            menu_app_settings::remove_editor_from_app_settings(editor.id)?;

            db::delete_editor(editor.id)
                .wrap_err(format!("Failed to delete Editor - '{}", editor))?;
            return Ok(format!("Successfully deleted Editor '{}'", editor));
        }
    }

    Ok("Canceled Editor deletion".to_string())
}

pub fn cli_set_default_editor(path: &str) -> Result<String, eyre::Report> {
    let editor = db::get_editor_by_path(path)?;

    let mut app_settings = db::get_app_settings()?;
    app_settings.default_editor_id = Some(editor.id);
    db::save_app_settings(app_settings)?;
    Ok(format!("Successfully set Editor '{}' as Default", editor))
}
