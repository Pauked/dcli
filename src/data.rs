use core::fmt;

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

impl fmt::Display for Engine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({} [{}])",
            self.game_engine_type, self.path, self.version
        )
    }
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

impl fmt::Display for Iwad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({})",
            self.internal_wad_type, self.path
        )
    }
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

impl fmt::Display for Pwad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({})",
            self.name, self.path
        )
    }
}

/*
    TODO: Expand profiles to include additional args.
    - save games
    - complevels
    - episode
    - level
    - difficult
    - fast monsters
    - no monsters
    - respawn monsters
    - demo record,
    - demo playback
    - GzDoom specific options
    - DSDA specific options
*/
#[derive(Clone, Debug, FromRow)]
pub struct Profile {
    pub id: i32,
    pub name: String,
    pub engine_id: Option<i32>,
    pub iwad_id: Option<i32>,
    pub pwad_id: Option<i32>,
    pub additional_arguments: Option<String>,
}

#[derive(Clone, Debug, Tabled)]
pub struct ProfileDisplay {
    #[tabled(skip)]
    pub id: i32,
    pub name: String,
    pub engine_path: String,
    pub engine_file: String,
    pub engine_version: String,
    pub iwad_path: String,
    pub iwad_file: String,
    pub pwad_path: String,
    pub pwad_file: String,
    pub additional_arguments: String,
}

impl fmt::Display for ProfileDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}) / {} [{}] / {}",
            self.name, self.pwad_file, self.engine_file, self.engine_version, self.iwad_file
        )
    }
}

#[derive(Clone, Debug, FromRow, Tabled)]
pub struct Settings {
    #[tabled(skip)]
    pub id: i32,
    #[tabled(skip)]
    pub active_profile_id: Option<i32>,
    #[tabled(rename = "Exe Search Folder", display_with = "display_option_string")]
    pub exe_search_folder: Option<String>,
    #[tabled(
        rename = "Internal WAD Search Folder",
        display_with = "display_option_string"
    )]
    pub iwad_search_folder: Option<String>,
    #[tabled(
        rename = "Patch WAD Search Folder",
        display_with = "display_option_string"
    )]
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
