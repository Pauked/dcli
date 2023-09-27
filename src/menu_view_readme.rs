use crate::{db, menu_common, runner};
use eyre::Context;

pub fn view_map_readme_from_map_id(map_id: i32) -> Result<String, eyre::Report> {
    let map = db::get_map_by_id(map_id)
        .wrap_err(format!("Unable to get Map for id '{}'", map_id).to_string())?;

    runner::open_map_readme(&map.path)
}

pub fn view_from_default_profile() -> Result<String, eyre::Report> {
    let map_id = menu_common::get_map_id_from_from_default_profile("Cannot view Map Readme.")?;

    view_map_readme_from_map_id(map_id)
}

pub fn view_from_last_profile() -> Result<String, eyre::Report> {
    let map_id = menu_common::get_map_id_from_from_last_profile("Cannot view Map Readme.")?;

    view_map_readme_from_map_id(map_id)
}

pub fn view_from_pick_profile() -> Result<String, eyre::Report> {
    let map_id = menu_common::get_map_id_from_pick_profile(
        "Pick the Profile to view Map Readme from:",
        "Cancelled viewing Map Readme.",
    )?;

    view_map_readme_from_map_id(map_id)
}

pub fn view_from_pick_map() -> Result<String, eyre::Report> {
    let map_id = menu_common::get_map_id_from_pick_map(
        "Pick the Map to view Map Readme from:",
        "Cancelled viewing Map Readme.",
    )?;

    view_map_readme_from_map_id(map_id)
}
