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
        tui::MapEditorCommand::OpenMapEditor => todo!("Open map editor"),
        tui::MapEditorCommand::OpenMapEditorWithMap => todo!("Open map editor with map"),
        tui::MapEditorCommand::List => todo!("List map editors"),
        tui::MapEditorCommand::Update => todo!("Update map editors"),
        tui::MapEditorCommand::Back => Ok("".to_string()),
        tui::MapEditorCommand::Unknown => Ok("Unknown command".to_string()),
    }
}
