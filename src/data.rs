
use sqlx::FromRow;
use tabled::Tabled;

use crate::doom_data;

#[derive(Clone, Debug, FromRow, Tabled)]
pub struct Engine {
    #[tabled(skip)]
    pub id: i32,
    #[tabled(rename = "Path")]
    pub path: String,
    #[tabled(rename = "Version")]
    pub version: String,
    #[tabled(rename = "Engine Type")]
    pub game_engine_type: doom_data::GameEngineType,
}

#[derive(Clone, Debug, FromRow, Tabled)]
pub struct Iwad {
    #[tabled(skip)]
    pub id: i32,
    #[tabled(rename = "Path")]
    pub path: String,
    #[tabled(rename = "Internal WAD Type")]
    pub internal_wad_type: doom_data::InternalWadType,
}

#[derive(Clone, Debug, FromRow, Tabled)]
pub struct Pwad {
    #[tabled(skip)]
    pub id: i32,
    #[tabled(rename = "Name")]
    pub name: String,
    #[tabled(rename = "Path")]
    pub path: String,
}

#[derive(Clone, Debug, FromRow)]
pub struct Profile {
    pub id: i32,
    pub name: String,
    pub engine_id: Option<i32>,
    pub iwad_id: Option<i32>,
    pub pwad_id: Option<i32>,
}

#[derive(Clone, Debug, FromRow, Tabled)]
pub struct Settings {
    #[tabled(skip)]
    pub id: i32,
    #[tabled(skip)]
    pub active_profile_id: Option<i32>,
    #[tabled(rename = "Exe Search Folder", display_with = "display_option_string")]
    pub exe_search_folder: Option<String>,
    #[tabled(rename = "Internal WAD Search Folder", display_with = "display_option_string")]
    pub iwad_search_folder: Option<String>,
    #[tabled(rename = "Patch WAD Search Folder", display_with = "display_option_string")]
    pub pwad_search_folder: Option<String>,
}

// pub fn display_option_i32(value: &Option<i32>) -> String {
//     match value {
//         Some(i) => i.to_string(),
//         None => "N/A".to_string(),
//     }
// }

pub fn display_option_string(value: &Option<String>) -> String {
    match value {
        Some(s) => s.to_string(),
        None => "N/A".to_string(),
    }
}
