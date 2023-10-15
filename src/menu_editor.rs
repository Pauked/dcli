use eyre::Context;
use inquire::validator::Validation;
use owo_colors::OwoColorize;
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
        .with_formatter(&|i| i.value.simple_display())
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
        .with_formatter(&|i| i.value.simple_display())
        .prompt_skippable()?;

    match editor {
        Some(editor) => {
            app_settings.default_editor_id = Some(editor.id);
            db::save_app_settings(app_settings).wrap_err("Failed to set Default Editor")?;
            Ok(format!(
                "Marked Editor '{}' as Default",
                editor.simple_display()
            ))
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
        .with(Modify::new(Rows::new(1..)).with(Width::wrap(30)))
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

    let editor_executables = paths::find_file_in_folders(
        &editor_search_folder,
        vec![&editor_executable_name],
        "Editors",
    );
    if editor_executables.is_empty() {
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
    for (index, editor_executable) in editor_executables.iter().enumerate() {
        if db_editors
            .iter()
            .any(|db| db.path.to_lowercase() == editor_executable.to_lowercase())
        {
            db_defaults.push(index);
        }
    }

    // Create a new list with version details
    let mut editors_extended: Vec<data::Editor> = Vec::new();
    for editor_executable in editor_executables {
        log::info!(
            "Getting version information for Editor: '{}'",
            editor_executable
        );
        let file_version = finder::get_file_version(&editor_executable);

        match file_version {
            Ok(file_version) => {
                editors_extended.push(data::Editor {
                    id: 0,
                    app_name: file_version.app_name.clone(),
                    path: editor_executable,
                    version: file_version.display_version(),
                    load_file_argument: load_file_argument.clone(),
                    additional_arguments: additional_arguments.clone(),
                });
                log::info!(
                    "  {}",
                    editors_extended
                        .last()
                        .unwrap()
                        .simple_display()
                        .blue()
                        .to_string()
                );
            }
            Err(e) => {
                log::info!(
                    "  Skipping Editor, unable to get version information: {}",
                    e.to_string().red()
                );
                log::debug!("Error: {:?}", e);
            }
        }
    }

    // Multi-select prompt to user
    let selections =
        inquire::MultiSelect::new("Pick the Editors you want to save:", editors_extended)
            .with_default(&db_defaults)
            .with_page_size(tui::MENU_PAGE_SIZE)
            .with_formatter(&|i| {
                i.iter()
                    .map(|e| e.value.simple_display())
                    .collect::<Vec<String>>()
                    .join(", ")
            })
            .prompt()?;

    let mut count = 0;

    // Save engines to  engines table
    for selection in selections {
        let editor = db_editors
            .iter()
            .find(|e| e.path.to_lowercase() == selection.path.to_lowercase());
        match editor {
            Some(existing) => {
                log::info!(
                    "  Editor already exists, no need to add: {}",
                    selection.simple_display().yellow()
                );
                if existing.version != selection.version {
                    log::debug!(
                        "  Updating Editor version from '{}' to '{}'",
                        existing.version.blue(),
                        selection.version.green()
                    );
                    db::update_editor_version(existing.id, &selection.version)?;
                }
            }
            None => {
                db::add_editor(&selection)?;
                log::debug!("Added Editor: {:?}", selection);
                log::info!("Added Editor: {}", selection.simple_display().blue());
                count += 1;
            }
        }
    }

    // Save the updated app setting
    let mut app_settings = db::get_app_settings()?;
    app_settings.editor_search_folder = Some(editor_search_folder);
    db::save_app_settings(app_settings)?;

    // Feedback to user
    if count > 0 {
        let result_message = format!("Successfully added {} Editors", count);
        log::info!("{}", result_message.green());
    }
    inquire::Text::new("Press any key to continue...").prompt_skippable()?;

    Ok("Successfully updated Editors".to_string())
}

pub fn cli_add_editor(
    path: &str,
    load_file_argument: Option<String>,
    args: Option<Vec<String>>,
) -> Result<String, eyre::Report> {
    if !paths::file_exists(path) {
        return Ok(format!("Cannot add Editor '{}'. Does not exist", path));
    };

    // Check it doesn't exist already
    let editor_result = db::get_editor_by_path(path);
    if editor_result.is_ok() {
        return Ok(format!(
            "Cannot add Editor '{}'. Editor already exists",
            path
        ));
    };

    let file_version = finder::get_file_version(path);
    let additional_arguments = args.map(|args_unwrapped| args_unwrapped.join(" "));

    match file_version {
        Ok(file_version) => {
            let editor = data::Editor {
                id: 0,
                app_name: file_version.app_name.clone(),
                path: path.to_string(),
                version: file_version.display_version(),
                load_file_argument: load_file_argument.clone(),
                additional_arguments: additional_arguments.clone(),
            };

            db::add_editor(&editor)?;
            log::debug!("Added Editor: {:?}", editor);

            Ok(format!(
                "Successfully added Editor - '{}'",
                editor.simple_display(),
            ))
        }
        Err(e) => Ok(format!(
            "Cannot add Editor, unable to get version information: '{}'",
            e,
        )),
    }
}

fn delete_editor_core(
    editor_id: i32,
    editor_app_name: &str,
    force: bool,
) -> Result<String, eyre::Report> {
    if force
        || inquire::Confirm::new(&format!(
            "Are you sure you want to delete this Editor - '{}'? This cannot be undone",
            editor_app_name
        ))
        .with_default(false)
        .prompt()?
    {
        // Check if "Default Editor" and remove link if so
        menu_app_settings::remove_editor_from_app_settings(editor_id)?;

        db::delete_editor(editor_id)
            .wrap_err(format!("Failed to delete Editor - '{}", editor_app_name))?;
        return Ok(format!("Successfully deleted Editor '{}'", editor_app_name));
    }

    Ok("Canceled Editor deletion".to_string())
}

pub fn delete_editor() -> Result<String, eyre::Report> {
    let editor_list = db::get_editors()?;
    if editor_list.is_empty() {
        return Ok("There are no Editors to delete".to_string());
    }

    let editor_selection = inquire::Select::new("Pick the Editor to Delete:", editor_list)
        .with_page_size(tui::MENU_PAGE_SIZE)
        .with_formatter(&|i| i.value.simple_display())
        .prompt_skippable()?;

    if let Some(editor) = editor_selection {
        delete_editor_core(editor.id, &editor.app_name, false)
    } else {
        Ok("No changes made to deleting Editor".to_string())
    }
}

pub fn cli_delete_editor(editor_path: &str, force: bool) -> Result<String, eyre::Report> {
    let editor_result = db::get_editor_by_path(editor_path);
    if let Ok(editor) = editor_result {
        delete_editor_core(editor.id, &editor.app_name, force)
    } else {
        Ok(format!("Editor not found - '{}'", editor_path))
    }
}

pub fn cli_set_default_editor(path: &str) -> Result<String, eyre::Report> {
    let editor = db::get_editor_by_path(path)?;

    let mut app_settings = db::get_app_settings()?;
    app_settings.default_editor_id = Some(editor.id);
    db::save_app_settings(app_settings)?;
    Ok(format!(
        "Successfully set Editor '{}' as Default",
        editor.simple_display()
    ))
}
