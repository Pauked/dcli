use core::fmt;

use chrono::{DateTime, Datelike, Local, Timelike, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use strum_macros::{Display, EnumString};
use tabled::Tabled;

use crate::{
    constants, doom_data,
    tui::{self, MenuMode},
};

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
        format!(
            "{}.{}.{}.{}",
            self.major, self.minor, self.build, self.revision
        )
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

impl Default for Engine {
    fn default() -> Self {
        Engine {
            id: 0,
            app_name: constants::DEFAULT_NOT_SET.to_string(),
            path: constants::DEFAULT_NOT_SET.to_string(),
            version: "-".to_string(),
            game_engine_type: doom_data::GameEngineType::Unknown,
        }
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
        write!(f, "{} ({})", self.internal_wad_type, self.path)
    }
}

impl Default for Iwad {
    fn default() -> Self {
        Iwad {
            id: 0,
            path: constants::DEFAULT_NOT_SET.to_string(),
            internal_wad_type: doom_data::InternalWadType::Unknown,
        }
    }
}

#[derive(Clone, Debug, FromRow, Tabled)]
pub struct Pwad {
    #[tabled(skip)]
    pub id: i32,
    #[tabled(rename = "Title")]
    pub title: String,
    #[tabled(rename = "Author")]
    pub author: String,
    #[tabled(rename = "Path")]
    pub path: String,
}

impl fmt::Display for Pwad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} by '{}' ({})", self.title, self.author, self.path)
    }
}

impl Default for Pwad {
    fn default() -> Self {
        Pwad {
            id: 0,
            title: constants::DEFAULT_NOT_SET.to_string(),
            author: constants::DEFAULT_NOT_SET.to_string(),
            path: constants::DEFAULT_NOT_SET.to_string(),
        }
    }
}

#[derive(Clone, Debug, FromRow, Tabled)]
pub struct MapEditor {
    #[tabled(skip)]
    pub id: i32,
    #[tabled(rename = "App Name")]
    pub app_name: String,
    #[tabled(rename = "Path")]
    pub path: String,
    #[tabled(rename = "Load File Argument", display_with = "display_option_string")]
    pub load_file_argument: Option<String>,
    #[tabled(
        rename = "Additional Arguments",
        display_with = "display_option_string"
    )]
    pub additional_arguments: Option<String>,
    #[tabled(rename = "Version")]
    pub version: String,
}

impl fmt::Display for MapEditor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({} [{}])", self.app_name, self.path, self.version)
    }
}

#[derive(Clone, Debug, FromRow)]
pub struct Profile {
    pub id: i32,
    pub name: String,
    pub engine_id: Option<i32>,
    pub iwad_id: Option<i32>,
    pub pwad_id: Option<i32>,
    pub pwad_id2: Option<i32>,
    pub pwad_id3: Option<i32>,
    pub pwad_id4: Option<i32>,
    pub pwad_id5: Option<i32>,
    pub date_created: DateTime<Utc>,
    pub date_edited: DateTime<Utc>,
    pub date_last_run: Option<DateTime<Utc>>,
    pub additional_arguments: Option<String>,
}

pub type PwadIds = (i32, i32, i32, i32, i32);

pub fn pwad_ids_from_options(
    a: Option<i32>,
    b: Option<i32>,
    c: Option<i32>,
    d: Option<i32>,
    e: Option<i32>,
) -> PwadIds {
    (
        a.unwrap_or(0),
        b.unwrap_or(0),
        c.unwrap_or(0),
        d.unwrap_or(0),
        e.unwrap_or(0),
    )
}

pub type PwadStrings = (String, String, String, String, String);

#[derive(Clone, Debug, Tabled)]
pub struct ProfileDisplay {
    #[tabled(skip)]
    pub id: i32,
    #[tabled(rename = "Name")]
    pub name: String,
    #[tabled(skip)]
    pub engine_id: i32,
    #[tabled(rename = "Engine Path")]
    pub engine_path: String,
    #[tabled(rename = "Engine File")]
    pub engine_file: String,
    #[tabled(rename = "Engine Version")]
    pub engine_version: String,
    #[tabled(skip)]
    pub iwad_id: i32,
    #[tabled(rename = "IWAD Path")]
    pub iwad_path: String,
    #[tabled(rename = "IWAD Path")]
    pub iwad_file: String,
    #[tabled(skip)]
    pub pwad_ids: PwadIds,
    #[tabled(
        rename = "PWAD Path",
        display_with = "display_combined_tabled_pwad_strings"
    )]
    pub pwad_paths: PwadStrings,
    #[tabled(
        rename = "PWAD File",
        display_with = "display_combined_tabled_pwad_strings"
    )]
    pub pwad_files: PwadStrings,
    #[tabled(rename = "Additionl Args")]
    pub additional_arguments: String,
    #[tabled(
        rename = "Date Created",
        display_with = "display_utc_datetime_to_local"
    )]
    pub date_created: DateTime<Utc>,
    #[tabled(rename = "Date Edited", display_with = "display_utc_datetime_to_local")]
    pub date_edited: DateTime<Utc>,
    #[tabled(
        rename = "Date Last Run",
        display_with = "display_option_utc_datetime_to_local"
    )]
    pub date_last_run: Option<DateTime<Utc>>,
}

impl fmt::Display for ProfileDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}) / {} [{}] / {}",
            self.name,
            display_combined_pwad_strings(&self.pwad_files),
            self.engine_file,
            self.engine_version,
            self.iwad_file
        )
    }
}

#[derive(Clone, Debug, FromRow)]
pub struct AppSettings {
    pub id: i32,
    pub default_profile_id: Option<i32>,
    pub last_profile_id: Option<i32>,
    pub default_engine_id: Option<i32>,
    pub default_iwad_id: Option<i32>,
    pub default_map_editor_id: Option<i32>,
    pub exe_search_folder: Option<String>,
    pub iwad_search_folder: Option<String>,
    pub pwad_search_folder: Option<String>,
    pub map_editor_search_folder: Option<String>,
    pub menu_mode: tui::MenuMode,
}

impl Default for AppSettings {
    fn default() -> Self {
        AppSettings {
            id: 0,
            default_profile_id: None,
            last_profile_id: None,
            default_engine_id: None,
            default_iwad_id: None,
            default_map_editor_id: None,
            exe_search_folder: None,
            iwad_search_folder: None,
            pwad_search_folder: None,
            map_editor_search_folder: None,
            menu_mode: MenuMode::Simple,
        }
    }
}

#[derive(Clone, Debug, Tabled)]
pub struct AppSettingsDisplay {
    #[tabled(rename = "Menu Mode")]
    pub menu_mode: String,
    #[tabled(rename = "Default Profile")]
    pub default_profile: String,
    #[tabled(rename = "Last Run Profile")]
    pub last_profile: String,
    #[tabled(rename = "Default Engine")]
    pub default_engine: String,
    #[tabled(rename = "Default Internal WAD")]
    pub default_iwad: String,
    #[tabled(rename = "Default Map Editor")]
    pub default_map_editor: String,
    #[tabled(rename = "Executable Search Folder")]
    pub exe_search_folder: String,
    #[tabled(rename = "Internal WAD Search Folder")]
    pub iwad_search_folder: String,
    #[tabled(rename = "Patch WAD Search Folder")]
    pub pwad_search_folder: String,
    #[tabled(rename = "Map Editor Search Folder")]
    pub map_editor_search_folder: String,
}

pub fn display_option_u8(value: &Option<u8>) -> String {
    match value {
        Some(i) => i.to_string(),
        None => constants::DEFAULT_NOT_SET.to_string(),
    }
}

pub fn display_option_u32(value: &Option<u32>) -> String {
    match value {
        Some(i) => i.to_string(),
        None => constants::DEFAULT_NOT_SET.to_string(),
    }
}

pub fn display_option_string(value: &Option<String>) -> String {
    match value {
        Some(s) => s.to_string(),
        None => constants::DEFAULT_NOT_SET.to_string(),
    }
}

pub fn display_option_comp_level(value: &Option<CompLevel>) -> String {
    match value {
        Some(s) => s.to_string(),
        None => constants::DEFAULT_NOT_SET.to_string(),
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Display, EnumString, PartialEq, sqlx::Type)]
pub enum CompLevel {
    Default = 0,
    #[strum(serialize = "Doom and Doom 2")]
    DoomAndDoom2 = 2,
    #[strum(serialize = "Ultimate Doom")]
    UltimateDoom = 3,
    #[strum(serialize = "Final Doom")]
    FinalDoom = 4,
    Boom = 9,
    #[strum(serialize = "MBF")]
    Mbf = 11,
    #[strum(serialize = "MBF 21")]
    Mbf21 = 21,
}

/*
    TODO: Expand play settings to include additional args.
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

#[derive(Clone, Debug, FromRow, Default)]
pub struct PlaySettings {
    pub id: i32,
    pub comp_level: Option<CompLevel>,
    pub config_file: Option<String>,
    pub fast_monsters: bool,
    pub no_monsters: bool,
    pub respawn_monsters: bool,
    pub warp: Option<String>,
    pub skill: Option<u8>,
    pub turbo: Option<u8>,
    pub timer: Option<u32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub full_screen: bool,
    pub windowed: bool,
    pub additional_arguments: Option<String>,
}

// Helper methods for display
pub fn display_combined_tabled_pwad_strings(data: &PwadStrings) -> String {
    let vec = [&data.0, &data.1, &data.2, &data.3, &data.4]
        .iter()
        .filter(|&&s| !s.is_empty() && s != constants::DEFAULT_NOT_SET)
        .map(|&s| s.as_str())
        .collect::<Vec<&str>>();

    vec.join("\n")
}

pub fn display_combined_pwad_strings(data: &PwadStrings) -> String {
    let vec = [&data.0, &data.1, &data.2, &data.3, &data.4]
        .iter()
        .filter(|&&s| !s.is_empty() && s != constants::DEFAULT_NOT_SET)
        .map(|&s| s.as_str())
        .collect::<Vec<&str>>();

    vec.join(", ")
}

pub fn display_utc_datetime_to_local(value: &DateTime<Utc>) -> String {
    let converted: DateTime<Local> = DateTime::from(*value);
    format_local_datetime(&converted)
}

pub fn display_option_utc_datetime_to_local(value: &Option<DateTime<Utc>>) -> String {
    if let Some(d) = value {
        let converted: DateTime<Local> = DateTime::from(*d);
        return format_local_datetime(&converted);
    }

    "N/A".to_string()
}

fn format_local_datetime(local_datetime: &DateTime<Local>) -> String {
    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        local_datetime.year(),
        local_datetime.month(),
        local_datetime.day(),
        local_datetime.hour(),
        local_datetime.minute(),
        local_datetime.second()
    )
}
