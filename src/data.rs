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
    #[tabled(rename = "Internal Path", display_with = "display_option_string")]
    pub internal_path: Option<String>,
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
            internal_path: None,
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
pub struct Map {
    #[tabled(skip)]
    pub id: i32,
    #[tabled(rename = "Title")]
    pub title: String,
    #[tabled(rename = "Author")]
    pub author: String,
    #[tabled(rename = "Path")]
    pub path: String,
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} by '{}' ({})", self.title, self.author, self.path)
    }
}

impl Default for Map {
    fn default() -> Self {
        Map {
            id: 0,
            title: constants::DEFAULT_NOT_SET.to_string(),
            author: constants::DEFAULT_NOT_SET.to_string(),
            path: constants::DEFAULT_NOT_SET.to_string(),
        }
    }
}

#[derive(Clone, Debug, FromRow, Tabled)]
pub struct Editor {
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

impl fmt::Display for Editor {
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
    pub map_id: Option<i32>,
    pub map_id2: Option<i32>,
    pub map_id3: Option<i32>,
    pub map_id4: Option<i32>,
    pub map_id5: Option<i32>,
    pub date_created: DateTime<Utc>,
    pub date_edited: DateTime<Utc>,
    pub date_last_run: Option<DateTime<Utc>>,
    pub run_count: i32,
    pub additional_arguments: Option<String>,
}

pub type MapIds = (i32, i32, i32, i32, i32);

pub fn map_ids_from_options(
    a: Option<i32>,
    b: Option<i32>,
    c: Option<i32>,
    d: Option<i32>,
    e: Option<i32>,
) -> MapIds {
    (
        a.unwrap_or(0),
        b.unwrap_or(0),
        c.unwrap_or(0),
        d.unwrap_or(0),
        e.unwrap_or(0),
    )
}

pub type MapStrings = (String, String, String, String, String);

#[derive(Clone, Debug, Tabled)]
pub struct ProfileDisplay {
    #[tabled(skip)]
    pub id: i32,
    #[tabled(rename = "Profile Name")]
    pub name: String,
    #[tabled(skip)]
    pub engine_id: i32,
    #[tabled(rename = "Engine App Name")]
    pub engine_app_name: String,
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
    #[tabled(rename = "IWAD File")]
    pub iwad_file: String,
    #[tabled(skip)]
    pub map_ids: MapIds,
    #[tabled(
        rename = "Map Paths",
        display_with = "display_combined_tabled_map_strings"
    )]
    pub map_paths: MapStrings,
    #[tabled(
        rename = "Map Files",
        display_with = "display_combined_tabled_map_strings"
    )]
    pub map_files: MapStrings,
    #[tabled(rename = "Additional Args")]
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
    #[tabled(rename = "Run Count")]
    pub run_count: i32,
}

impl fmt::Display for ProfileDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let maps = display_combined_map_strings(&self.map_files);
        if maps.is_empty() {
            write!(
                f,
                "{} / {} [{}] / {}",
                self.name, self.engine_file, self.engine_version, self.iwad_file
            )
        } else {
            write!(
                f,
                "{} ({}) / {} [{}] / {}",
                self.name, maps, self.engine_file, self.engine_version, self.iwad_file
            )
        }
    }
}

#[derive(Clone, Debug, FromRow)]
pub struct AppSettings {
    pub id: i32,
    pub default_profile_id: Option<i32>,
    pub last_profile_id: Option<i32>,
    pub default_engine_id: Option<i32>,
    pub default_iwad_id: Option<i32>,
    pub default_editor_id: Option<i32>,
    pub engine_search_folder: Option<String>,
    pub iwad_search_folder: Option<String>,
    pub map_search_folder: Option<String>,
    pub editor_search_folder: Option<String>,
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
            default_editor_id: None,
            engine_search_folder: None,
            iwad_search_folder: None,
            map_search_folder: None,
            editor_search_folder: None,
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
    #[tabled(rename = "Default Editor")]
    pub default_editor: String,
    #[tabled(rename = "Engine Search Folder")]
    pub engine_search_folder: String,
    #[tabled(rename = "Internal WAD Search Folder")]
    pub iwad_search_folder: String,
    #[tabled(rename = "Map Search Folder")]
    pub map_search_folder: String,
    #[tabled(rename = "Editor Search Folder")]
    pub editor_search_folder: String,
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
    #[strum(serialize = "Doom v1.2 (0)")]
    DoomV12 = 0,
    #[strum(serialize = "Doom v1.666 (1)")]
    DoomV1666 = 1,
    #[strum(serialize = "Doom v1.9 (2)")]
    DoomV19 = 2,
    #[strum(serialize = "Ultimate Doom (3)")]
    UltimateDoom = 3,
    #[strum(serialize = "Final Doom & Doom95 (4)")]
    FinalDoom = 4,
    #[strum(serialize = "DOSDoom (5)")]
    DosDoom = 5,
    #[strum(serialize = "TASDoom (6)")]
    TasDoom = 6,
    #[strum(serialize = "Boom (6)")]
    Boom = 7,
    #[strum(serialize = "Boom v2.01 (8)")]
    BoomV201 = 8,
    #[strum(serialize = "Boom v2.02 (9)")]
    BoomV202 = 9,
    #[strum(serialize = "LxDoom (10)")]
    LxDoom = 10,
    #[strum(serialize = "MBF (11)")]
    Mbf = 11,
    #[strum(serialize = "PrBoom+ (17)")]
    PrBoomPlus = 17,
    #[strum(serialize = "MBF 21 (21)")]
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

#[derive(Clone, Debug, FromRow, Default, Tabled)]
pub struct PlaySettings {
    #[tabled(skip)]
    pub id: i32,
    #[tabled(
        rename = "Compatibility Level",
        display_with = "display_option_comp_level"
    )]
    pub comp_level: Option<CompLevel>,
    #[tabled(rename = "Config File", display_with = "display_option_string")]
    pub config_file: Option<String>,
    #[tabled(rename = "Fast Monsters")]
    pub fast_monsters: bool,
    #[tabled(rename = "No Monsters")]
    pub no_monsters: bool,
    #[tabled(rename = "Respawn Monsters")]
    pub respawn_monsters: bool,
    #[tabled(rename = "Warp to Level", display_with = "display_option_string")]
    pub warp: Option<String>,
    #[tabled(rename = "Skill", display_with = "display_option_u8")]
    pub skill: Option<u8>,
    #[tabled(rename = "Turbo", display_with = "display_option_u8")]
    pub turbo: Option<u8>,
    #[tabled(rename = "Timer", display_with = "display_option_u32")]
    pub timer: Option<u32>,
    #[tabled(rename = "Screen Width", display_with = "display_option_u32")]
    pub width: Option<u32>,
    #[tabled(rename = "Screen Height", display_with = "display_option_u32")]
    pub height: Option<u32>,
    #[tabled(rename = "Full Screen")]
    pub full_screen: bool,
    #[tabled(rename = "Windowed")]
    pub windowed: bool,
    #[tabled(
        rename = "Additional Arguments",
        display_with = "display_option_string"
    )]
    pub additional_arguments: Option<String>,
}

// Helper methods for display
pub fn display_combined_tabled_map_strings(data: &MapStrings) -> String {
    let vec = [&data.0, &data.1, &data.2, &data.3, &data.4]
        .iter()
        .filter(|&&s| !s.is_empty() && s != constants::DEFAULT_NOT_SET)
        .map(|&s| s.as_str())
        .collect::<Vec<&str>>();

    vec.join("\n")
}

pub fn display_combined_map_strings(data: &MapStrings) -> String {
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

pub enum ListType {
    Full,
    Summary,
}
