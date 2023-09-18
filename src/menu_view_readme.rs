use crate::{db, menu_common, runner};
use eyre::Context;

pub async fn view_map_readme_from_pwad_id(pwad_id: i32) -> Result<String, eyre::Report> {
    let pwad = db::get_pwad_by_id(pwad_id)
        .await
        .wrap_err(format!("Unable to get PWAD for id '{}'", pwad_id).to_string())?;

    runner::open_map_readme(&pwad.path)
}

pub async fn view_from_active_profile() -> Result<String, eyre::Report> {
    let pwad_id =
        menu_common::get_pwad_id_from_from_active_profile("Cannot view Map Readme.").await?;

    view_map_readme_from_pwad_id(pwad_id).await
}

pub async fn view_from_last_profile() -> Result<String, eyre::Report> {
    let pwad_id =
        menu_common::get_pwad_id_from_from_last_profile("Cannot view Map Readme.").await?;

    view_map_readme_from_pwad_id(pwad_id).await
}

pub async fn view_from_pick_profile() -> Result<String, eyre::Report> {
    let pwad_id = menu_common::get_pwad_id_from_pick_profile(
        "Pick the Profile to view Map Readme from:",
        "Cancelled viewing Map Readme.",
    )
    .await?;

    view_map_readme_from_pwad_id(pwad_id).await
}

pub async fn view_from_pick_pwad() -> Result<String, eyre::Report> {
    let pwad_id = menu_common::get_pwad_id_from_pick_pwad(
        "Pick the PWAD to view Map Readme from:",
        "Cancelled viewing Map Readme.",
    )
    .await?;

    view_map_readme_from_pwad_id(pwad_id).await
}
