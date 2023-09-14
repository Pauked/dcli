use core::fmt;

use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use strum_macros::{EnumString, Display};
use tabled::Tabled;

use crate::doom_data;

#[derive(Clone, Debug)]
pub struct FileVersion {
    pub app_name: String,
    pub path: String,
    pub major: u32,
    pub minor: u32,
    pub build: u32,
    pub revision: u32,
}

impl FileVersion {
    pub fn display_version(&self) -> String {
        format!("{}.{}.{}.{}", self.major, self.minor, self.build, self.revision)
    }
}

#[derive(Clone, Debug, FromRow, Tabled)]
pub struct Engine {
    #[tabled(skip)]
    pub id: i32,
    #[tabled(rename = "App Name")]
    pub app_name: String,
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
            "{} ({} [{}], {})",
            self.app_name, self.path, self.version, self.game_engine_type
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
    pub engine_id: i32,
    pub engine_path: String,
    pub engine_file: String,
    pub engine_version: String,
    pub iwad_id: i32,
    pub iwad_path: String,
    pub iwad_file: String,
    pub pwad_id: i32,
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

#[derive(Clone, Debug, FromRow, Tabled)]
pub struct GameSettings {
    #[tabled(skip)]
    pub id: i32,
    pub comp_level: CompLevel,
    pub fast_monsters: bool,
    pub no_monsters: bool,
    pub respawn_monsters: bool,
    #[tabled(
        rename = "Episode and Level or Map",
        display_with = "display_option_string"
    )]
    pub map: Option<String>,
    pub skill: Skill,
    pub turbo: i8,
    pub timer: i32,
    #[tabled(
        rename = "Screen Resolution",
        display_with = "display_option_string"
    )]
    pub resolution: Option<String>,
    pub full_screen: bool,
}

#[derive(
    Clone, Debug, Serialize, Deserialize, Display, EnumString, PartialEq, sqlx::Type,
)]
pub enum CompLevel {
    Default = 0,
    DoomAndDoom2 = 2,
    UltimateDoom = 3,
    FinalDoom = 4,
    Boom = 9,
    Mbf = 11,
    Mbf21 = 21,
}

#[derive(
    Clone, Debug, Serialize, Deserialize, Display, EnumString, PartialEq, sqlx::Type,
)]
pub enum Skill {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
}