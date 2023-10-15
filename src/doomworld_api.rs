use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::runtime;

use crate::{constants, data, paths};

pub const SEARCH_FILENAME: &str = "filename";
pub const SEARCH_AUTHOR: &str = "author";
pub const SEARCH_TITLE: &str = "title";

pub const SORT_DATE: &str = "date";
pub const SORT_FILENAME: &str = "filename";

const DISPLAY_WIDTH1: usize = 45;
const DISPLAY_WIDTH2: usize = 35;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoomworldFile {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub description: Option<String>,
    pub filename: String,
    pub size: i32,
    pub url: String,
    pub dir: String,
    // ...other fields
}

impl DoomworldFile {
    pub fn short_display(&self) -> String {
        format!("{} by {} ({})", self.title, self.author, self.filename,)
    }
}

impl fmt::Display for DoomworldFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = DISPLAY_WIDTH1;
        let width2 = DISPLAY_WIDTH2;
        write!(
            f,
            "{:<width$} | {:<width2$} | {:<}",
            data::truncate_string_end(&self.title, width),
            data::truncate_string_end(&self.author, width2),
            &self.filename,
            width = width,
            width2 = width2,
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Warning {
    #[serde(rename = "type")]
    warning_type: String,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Content {
    file: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Meta {
    version: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    warning: Option<Warning>,
    content: Option<Content>,
    meta: Meta,
}

pub fn lookup_map_from_doomworld_api(
    map_path: &str,
) -> Result<(String, String, Option<i32>, Option<String>), eyre::Report> {
    let filename = format!("{}.zip", paths::extract_file_name_no_extension(map_path));
    let files = search_doomworld_api(&filename, SEARCH_FILENAME, SORT_DATE)?;
    if files.is_empty() {
        return Ok((
            constants::DEFAULT_UNKNOWN.to_string(),
            constants::DEFAULT_UNKNOWN.to_string(),
            None,
            None,
        ));
    }

    // FIXME: Assumption corner on multiple files... (and first one could be wrong!)
    let file = files[0].clone();
    Ok((file.title, file.author, Some(file.id), Some(file.url)))
}

pub fn search_doomworld_api(
    search_query: &str,
    search_type: &str,
    sort_type: &str,
) -> Result<Vec<DoomworldFile>, eyre::Report> {
    // Help guide: https://www.doomworld.com/idgames/api/
    let base_url = "https://www.doomworld.com/idgames/api/api.php";

    let params = [
        ("action", "search"),
        ("query", search_query),
        ("type", search_type),
        ("sort", sort_type),
        ("out", "json"),
    ];

    log::debug!("  Sending request to: {}", base_url);
    let runtime: runtime::Runtime = tokio::runtime::Runtime::new().unwrap();
    let response: ApiResponse = runtime.block_on(async {
        reqwest::Client::new()
            .get(base_url)
            .query(&params)
            .send()
            .await?
            .json()
            .await
    })?;
    log::debug!("  Response: {:?}", response);

    if let Some(warning) = response.warning {
        log::debug!("  Warning: {:?}", warning);
        return Ok(vec![]);
    }

    log::debug!("  Getting file info");
    let files: Vec<DoomworldFile> =
        response
            .content
            .and_then(|c| c.file)
            .map_or_else(std::vec::Vec::new, |file| match file {
                Value::Object(_) => vec![serde_json::from_value(file).unwrap()],
                Value::Array(_) => serde_json::from_value(file).unwrap_or_default(),
                _ => vec![],
            });

    Ok(files)
}
