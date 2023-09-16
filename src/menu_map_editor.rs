use log::info;

use crate::tui;

pub async fn map_editor_menu() -> Result<String, eyre::Report> {
    clearscreen::clear().unwrap();
    loop {
        let menu_command = tui::map_editor_menu_prompt();
        if let tui::MapEditorCommand::Back = menu_command {
            return Ok("".to_string());
        }
        let result = run_map_editor_menu_option(menu_command).await?;
        clearscreen::clear().unwrap();
        info!("{}", result)
    }
}

pub async fn run_map_editor_menu_option(
    menu_command: tui::MapEditorCommand,
) -> Result<String, eyre::Report> {
    match menu_command {

    }
}