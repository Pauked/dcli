use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::runtime;

use crate::{constants, paths};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DoomworldFile {
    id: i32,
    title: String,
    author: String,
    description: Option<String>,
    filename: String,
    size: i32,
    url: String,
    // ...other fields
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

pub fn get_details_from_doomworld_api(
    map_path: &str,
) -> Result<(String, String, Option<i32>, Option<String>), eyre::Report> {
    let filename = format!("{}.zip", paths::extract_file_name_no_extension(map_path));
    let files = get_file_from_doomworld_api(&filename)?;
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

fn get_file_from_doomworld_api(filename: &str) -> Result<Vec<DoomworldFile>, eyre::Report> {
    let base_url = "https://www.doomworld.com/idgames/api/api.php";

    let params = [
        ("action", "search"),
        ("query", filename),
        ("type", "filename"),
        ("sort", "date"),
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
