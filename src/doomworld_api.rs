use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};
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

const EXPECTED_API_VERSION: i32 = 3;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoomworldFile {
    pub id: i32,
    #[serde(deserialize_with = "deserialize_string_or_default")]
    pub title: String,
    #[serde(deserialize_with = "deserialize_string_or_default")]
    pub author: String,
    pub filename: String,
    pub url: String,
    pub dir: String,
}

fn deserialize_string_or_default<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer);
    match opt {
        Ok(Some(value)) => Ok(value),
        Ok(None) | Err(_) => Ok(constants::DEFAULT_UNKNOWN.to_string()),
    }
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
pub struct Error {
    #[serde(rename = "type")]
    pub error_type: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Warning {
    #[serde(rename = "type")]
    pub warning_type: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    pub file: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub version: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub error: Option<Error>,
    pub warning: Option<Warning>,
    pub content: Option<Content>,
    pub meta: Meta,
}

pub struct ApiResult {
    pub message: String,
    pub files: Vec<DoomworldFile>,
}

pub fn lookup_map_from_doomworld_api(
    map_path: &str,
) -> Result<(String, String, Option<i32>, Option<String>), eyre::Report> {
    let filename = format!("{}.zip", paths::extract_file_name_no_extension(map_path));
    let api_result = search_doomworld_api(&filename, SEARCH_FILENAME, SORT_DATE)?;
    if api_result.files.is_empty() {
        return Ok((
            constants::DEFAULT_UNKNOWN.to_string(),
            constants::DEFAULT_UNKNOWN.to_string(),
            None,
            None,
        ));
    }

    // FIXME: Assumption corner on multiple files... (and first one could be wrong!)
    let file = api_result.files[0].clone();
    Ok((file.title, file.author, Some(file.id), Some(file.url)))
}

pub fn search_doomworld_api(
    search_query: &str,
    search_type: &str,
    sort_type: &str,
) -> Result<ApiResult, eyre::Report> {
    // Help guide: https://www.doomworld.com/idgames/api/
    let base_url = "https://www.doomworld.com/idgames/api/api.php";

    let params = [
        ("action", "search"),
        ("query", search_query),
        ("type", search_type),
        ("sort", sort_type),
        ("out", "json"),
    ];
    let querystring: String = params
        .iter()
        .map(|(key, value)| format!("{}={}", key, urlencoding::encode(value)))
        .collect::<Vec<_>>()
        .join("&");

    log::debug!("  Sending request to: {}?{}", base_url, querystring);
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

    parse_doomworld_api_response(response)
}

fn parse_doomworld_api_response(response: ApiResponse) -> Result<ApiResult, eyre::Report> {
    if response.meta.version != EXPECTED_API_VERSION {
        return Err(eyre::eyre!(
            "Error, unexpected Doomworld API version: {} (expected {})",
            response.meta.version,
            EXPECTED_API_VERSION
        ));
    }

    let mut api_result: ApiResult = ApiResult {
        message: "".to_string(),
        files: vec![],
    };

    if let Some(error) = response.error {
        log::debug!("  Error: {:?}", error);
        api_result.message = format!("Error - {}", error.message);
    }

    if let Some(warning) = response.warning {
        log::debug!("  Warning: {:?}", warning);
        api_result.message = format!("Warning - {}", warning.message);
    }

    log::debug!("  Getting file info");
    api_result.files =
        response
            .content
            .and_then(|c| c.file)
            .map_or_else(std::vec::Vec::new, |file| match file {
                Value::Object(_) => vec![serde_json::from_value(file).unwrap()],
                Value::Array(_) => serde_json::from_value(file).unwrap(),
                _ => vec![],
            });
    log::debug!("  Got {} files", api_result.files.len());
    log::debug!("  File details {:?}", api_result.files);

    Ok(api_result)
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use crate::{constants, doomworld_api};

    #[test]
    fn happy_path_e1m4b() {
        // Arrange
        let path = Path::new("./test-data/e1m4b.json");
        let content = fs::read_to_string(path).unwrap();
        let response: doomworld_api::ApiResponse =
            serde_json::from_str(&content).expect("Failed to deserialize the JSON");

        // Act
        let api_result = doomworld_api::parse_doomworld_api_response(response).unwrap();

        // Assert
        assert_eq!(api_result.files.len(), 1);
        assert_eq!(api_result.files[0].id, 18412);
        assert_eq!(api_result.files[0].title, "Phobos Mission Control");
        assert_eq!(api_result.files[0].author, "John Romero");
        assert_eq!(api_result.files[0].filename, "e1m4b.zip");
        assert_eq!(
            api_result.files[0].url,
            "https://www.doomworld.com/idgames/levels/doom/Ports/d-f/e1m4b"
        );
        assert!(api_result.message.is_empty());
    }

    #[test]
    fn happy_path_valiant() {
        // Arrange
        let path = Path::new("./test-data/valiant.json");
        let content = fs::read_to_string(path).unwrap();
        let response: doomworld_api::ApiResponse =
            serde_json::from_str(&content).expect("Failed to deserialize the JSON");

        // Act
        let api_result = doomworld_api::parse_doomworld_api_response(response).unwrap();

        // Assert
        assert_eq!(api_result.files.len(), 1);
        assert_eq!(api_result.files[0].id, 18049);
        assert_eq!(api_result.files[0].title, "Valiant");
        assert_eq!(api_result.files[0].author, "Paul \"skillsaw\" DeBruyne");
        assert_eq!(api_result.files[0].filename, "valiant.zip");
        assert_eq!(
            api_result.files[0].url,
            "https://www.doomworld.com/idgames/levels/doom2/Ports/megawads/valiant"
        );
        assert!(api_result.message.is_empty());
    }

    #[test]
    fn handle_no_results() {
        // Arrange
        let path = Path::new("./test-data/no_results.json");
        let content = fs::read_to_string(path).unwrap();
        let response: doomworld_api::ApiResponse =
            serde_json::from_str(&content).expect("Failed to deserialize the JSON");

        // Act
        let api_result = doomworld_api::parse_doomworld_api_response(response).unwrap();

        // Assert
        assert_eq!(api_result.files.len(), 0);
        assert_eq!(
            api_result.message,
            "Warning - No files returned for query \"lemonbiscuitbase\"."
        );
    }

    #[test]
    fn handle_nulls() {
        // Arrange
        let path = Path::new("./test-data/nulls.json");
        let content = fs::read_to_string(path).unwrap();
        let response: doomworld_api::ApiResponse =
            serde_json::from_str(&content).expect("Failed to deserialize the JSON");

        // Act
        let api_result = doomworld_api::parse_doomworld_api_response(response).unwrap();

        // Assert
        assert_eq!(api_result.files.len(), 1);
        assert_eq!(api_result.files[0].id, 11528);
        assert_eq!(
            api_result.files[0].title,
            constants::DEFAULT_UNKNOWN.to_string()
        );
        assert_eq!(
            api_result.files[0].author,
            constants::DEFAULT_UNKNOWN.to_string()
        );
        assert_eq!(api_result.files[0].filename, "0scraps.zip");
        assert_eq!(
            api_result.files[0].url,
            "https://www.doomworld.com/idgames/levels/doom2/0-9/0scraps"
        );
        assert!(api_result.message.is_empty());
    }

    #[test]
    fn happy_path_22_files() {
        // Arrange
        let path = Path::new("./test-data/22_files-final_search.json");
        let content = fs::read_to_string(path).unwrap();
        let response: doomworld_api::ApiResponse =
            serde_json::from_str(&content).expect("Failed to deserialize the JSON");

        // Act
        let api_result = doomworld_api::parse_doomworld_api_response(response).unwrap();

        // Assert
        assert_eq!(api_result.files.len(), 22);

        assert_eq!(api_result.files[0].id, 1227);
        assert_eq!(api_result.files[0].title, "Final_2.wad");
        assert_eq!(api_result.files[0].author, "Chris Kleymeer");
        assert_eq!(api_result.files[0].filename, "final_2.zip");
        assert_eq!(
            api_result.files[0].url,
            "https://www.doomworld.com/idgames/levels/doom2/d-f/final_2"
        );

        assert_eq!(api_result.files[21].id, 20612);
        assert_eq!(api_result.files[21].title, "Machete");
        assert_eq!(api_result.files[21].author, "A2Rob");
        assert_eq!(api_result.files[21].filename, "machetefinal.zip");
        assert_eq!(
            api_result.files[21].url,
            "https://www.doomworld.com/idgames/levels/doom2/Ports/megawads/machetefinal"
        );
        assert!(api_result.message.is_empty());
    }

    #[test]
    fn happy_path_100_files_with_warning() {
        // Arrange
        let path = Path::new("./test-data/100_files-paul_search.json");
        let content = fs::read_to_string(path).unwrap();
        let response: doomworld_api::ApiResponse =
            serde_json::from_str(&content).expect("Failed to deserialize the JSON");

        // Act
        let api_result = doomworld_api::parse_doomworld_api_response(response).unwrap();

        // Assert
        assert_eq!(api_result.files.len(), 100);

        assert_eq!(api_result.files[0].id, 13327);
        assert_eq!(api_result.files[0].title, "000 EMERGANCY");
        assert_eq!(api_result.files[0].author, "Paul Corfiatis");
        assert_eq!(api_result.files[0].filename, "000emg.zip");
        assert_eq!(
            api_result.files[0].url,
            "https://www.doomworld.com/idgames/levels/doom2/0-9/000emg"
        );

        assert_eq!(api_result.files[99].id, 11499);
        assert_eq!(
            api_result.files[99].title,
            "Fras v9.3 Map 1: Turks Map 2: New Map 3: Brick Map 4: Biff! Kazam! Map 5: Brick II Map 6: Sky Map 7:"
        );
        assert_eq!(
            api_result.files[99].author,
            "Maps 1,3,5,7,(10,12,14): Paul O'Neill Maps 2,4,6,9,11,(13,15): Jerry P."
        );
        assert_eq!(api_result.files[99].filename, "fras.zip");
        assert_eq!(
            api_result.files[99].url,
            "https://www.doomworld.com/idgames/levels/doom2/deathmatch/d-f/fras"
        );
        assert_eq!(
            api_result.message,
            "Warning - Result limit reached. Returning 100 files."
        );
    }

    #[test]
    fn handle_error() {
        // Arrange
        let path = Path::new("./test-data/error_query_too_small.json");
        let content = fs::read_to_string(path).unwrap();
        let response: doomworld_api::ApiResponse =
            serde_json::from_str(&content).expect("Failed to deserialize the JSON");

        // Act
        let api_result = doomworld_api::parse_doomworld_api_response(response).unwrap();

        // Assert
        assert_eq!(api_result.files.len(), 0);
        assert_eq!(
            api_result.message,
            "Error - Query string is too small. Must be at least 3 characters."
        );
    }
}
