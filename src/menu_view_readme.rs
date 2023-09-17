use eyre::Context;
use log::info;

use crate::{db, menu_common, runner, tui};

pub async fn view_readme_menu() -> Result<String, eyre::Report> {
    clearscreen::clear().unwrap();
    loop {
        let menu_command = tui::view_readme_menu_prompt();
        if let tui::ViewReadmeCommand::Back = menu_command {
            return Ok("".to_string());
        }
        let result = run_view_readme_menu_option(menu_command).await?;
        clearscreen::clear().unwrap();
        info!("{}", result)
    }
}

pub async fn run_view_readme_menu_option(
    menu_command: tui::ViewReadmeCommand,
) -> Result<String, eyre::Report> {
    match menu_command {
        tui::ViewReadmeCommand::ViewFromActiveProfile => view_from_active_profile().await,
        tui::ViewReadmeCommand::ViewFromLastProfile => view_from_last_profile().await,
        tui::ViewReadmeCommand::ViewFromPickProfile => view_from_pick_profile().await,
        tui::ViewReadmeCommand::ViewFromPickPwad => view_from_pick_pwad().await,
        tui::ViewReadmeCommand::Back => Ok("".to_string()),
    }
}

async fn view_map_readme_from_pwad_id(pwad_id: i32) -> Result<String, eyre::Report> {
    let pwad = db::get_pwad_by_id(pwad_id)
        .await
        .wrap_err(format!("Unable to get PWAD for id '{}'", pwad_id).to_string())?;

    runner::open_map_readme(&pwad.path)
}

async fn view_from_active_profile() -> Result<String, eyre::Report> {
    let pwad_id =
        menu_common::get_pwad_id_from_from_active_profile("Cannot view Map Readme.").await?;

    view_map_readme_from_pwad_id(pwad_id).await
}

async fn view_from_last_profile() -> Result<String, eyre::Report> {
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

async fn view_from_pick_pwad() -> Result<String, eyre::Report> {
    let pwad_id = menu_common::get_pwad_id_from_pick_pwad(
        "Pick the PWAD to view Map Readme from:",
        "Cancelled viewing Map Readme.",
    )
    .await?;

    view_map_readme_from_pwad_id(pwad_id).await
}
