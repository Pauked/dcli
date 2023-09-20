use crate::{db, menu_common, runner};
use eyre::Context;

pub fn view_map_readme_from_pwad_id(pwad_id: i32) -> Result<String, eyre::Report> {
    let pwad = db::get_pwad_by_id(pwad_id)
        .wrap_err(format!("Unable to get PWAD for id '{}'", pwad_id).to_string())?;

    runner::open_map_readme(&pwad.path)
}

pub fn view_from_default_profile() -> Result<String, eyre::Report> {
    let pwad_id = menu_common::get_pwad_id_from_from_default_profile("Cannot view Map Readme.")?;

    view_map_readme_from_pwad_id(pwad_id)
}

pub fn view_from_last_profile() -> Result<String, eyre::Report> {
    let pwad_id = menu_common::get_pwad_id_from_from_last_profile("Cannot view Map Readme.")?;

    view_map_readme_from_pwad_id(pwad_id)
}

pub fn view_from_pick_profile() -> Result<String, eyre::Report> {
    let pwad_id = menu_common::get_pwad_id_from_pick_profile(
        "Pick the Profile to view Map Readme from:",
        "Cancelled viewing Map Readme.",
    )?;

    view_map_readme_from_pwad_id(pwad_id)
}

pub fn view_from_pick_pwad() -> Result<String, eyre::Report> {
    let pwad_id = menu_common::get_pwad_id_from_pick_pwad(
        "Pick the PWAD to view Map Readme from:",
        "Cancelled viewing Map Readme.",
    )?;

    view_map_readme_from_pwad_id(pwad_id)
}
