use crate::{data, db, doomworld_api, downloader, menu_common, runner, tui};
use eyre::Context;
use owo_colors::OwoColorize;

pub fn view_on_doomworld() -> Result<String, eyre::Report> {
    let map_list = db::get_maps()?;

    let filtered_maps: Vec<data::Map> = map_list
        .into_iter()
        .filter(|map| map.doomworld_url.is_some())
        .collect();

    if filtered_maps.is_empty() {
        return Err(eyre::eyre!("There are no Maps with Doomworld URLs"));
    }

    let map_selection =
        inquire::Select::new("Pick the Map you want to view on Doomworld:", filtered_maps)
            .with_page_size(tui::MENU_PAGE_SIZE)
            .with_formatter(&|i| i.value.simple_display())
            .prompt_skippable()?;

    let url = if let Some(ref map) = map_selection {
        map.doomworld_url.clone().unwrap()
    } else {
        return Err(eyre::eyre!(
            "Canceled viewing Map on Doomworld because no Map was selected"
        ));
    };

    runner::open_url(
        &url,
        &format!("Map '{}' on Doomworld", map_selection.unwrap().title),
    )
}

pub fn view_map_readme_from_map_id(map_id: i32) -> Result<String, eyre::Report> {
    let map = db::get_map_by_id(map_id)
        .wrap_err(format!("Unable to get Map for id '{}'", map_id).to_string())?;

    runner::open_map_readme(&map.path)
}

pub fn view_from_default_profile() -> Result<String, eyre::Report> {
    let map_id = menu_common::get_map_id_from_from_default_profile("Cannot view Map Readme")?;

    view_map_readme_from_map_id(map_id)
}

pub fn view_from_last_profile() -> Result<String, eyre::Report> {
    let map_id = menu_common::get_map_id_from_from_last_profile("Cannot view Map Readme")?;

    view_map_readme_from_map_id(map_id)
}

pub fn view_from_pick_profile() -> Result<String, eyre::Report> {
    let map_id = menu_common::get_map_id_from_pick_profile(
        "Pick the Profile to view Map Readme from:",
        "Canceled viewing Map Readme",
    )?;

    view_map_readme_from_map_id(map_id)
}

pub fn view_from_pick_map() -> Result<String, eyre::Report> {
    let map_id = menu_common::get_map_id_from_pick_map(
        "Pick the Map to view Map Readme from:",
        "Canceled viewing Map Readme",
    )?;

    view_map_readme_from_map_id(map_id)
}

fn search_dooomworld_and_download(
    nice_name: &str,
    search_type: &str,
) -> Result<String, eyre::Report> {
    let search = inquire::Text::new(&format!("Enter the {} you want to search on:", nice_name))
        //.with_help_message("Please note the search is limited to first 100 results.\nFor more complete results, use https://www.doomworld.com/idgames/index.php?search=")
        .prompt()?;

    let api_result =
        doomworld_api::search_doomworld_api(&search, search_type, doomworld_api::SORT_FILENAME)?;

    if api_result.files.is_empty() {
        return Err(eyre::eyre!(
            "No maps found for '{}' search with '{}'. Message from API '{}'",
            nice_name,
            search,
            api_result.message
        ));
    }

    if !api_result.message.is_empty() {
        log::info!(
            "{} '{}'",
            "Partial search results because".yellow(),
            api_result.message.yellow()
        );
    }

    let selection =
        inquire::MultiSelect::new("Pick the maps you want to download:", api_result.files)
            .with_page_size(tui::MENU_PAGE_SIZE)
            .with_formatter(&|i| {
                i.iter()
                    .map(|e| e.value.short_display())
                    .collect::<Vec<String>>()
                    .join(", ")
            })
            .prompt()?;

    downloader::download_and_extract_map_files(selection)
}

pub fn search_doomworld_by_author() -> Result<String, eyre::Report> {
    search_dooomworld_and_download("author", doomworld_api::SEARCH_AUTHOR)
}

pub fn search_doomworld_by_filename() -> Result<String, eyre::Report> {
    search_dooomworld_and_download("filename", doomworld_api::SEARCH_FILENAME)
}

pub fn search_doomworld_by_map_title() -> Result<String, eyre::Report> {
    search_dooomworld_and_download("map title", doomworld_api::SEARCH_TITLE)
}
