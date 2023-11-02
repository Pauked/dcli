use std::{io::Cursor, path::PathBuf};

use owo_colors::OwoColorize;
use reqwest::StatusCode;
use zip::ZipArchive;

use crate::{data, db, doomworld_api::DoomworldFile, files, menu_profiles, paths};

// Germany      - https://www.quaddicted.com/files/idgames/
// Sweden       - https://ftpmirror1.infania.net/pub/idgames/
// New York     - https://youfailit.net/pub/idgames/
// Virginia     - https://www.gamers.org/pub/idgames/
const DOWNLOAD_URL: &str = "https://www.quaddicted.com/files/idgames/";
const DOWNLOAD_FOLDER: &str = "!dcli-downloads";

pub fn download_and_extract_map_files(
    doomworld_files: Vec<DoomworldFile>,
) -> Result<String, eyre::Report> {
    // Thanks and goodbye
    if doomworld_files.is_empty() {
        return Err(eyre::eyre!("No files selected to download"));
    }

    // Get the maps folder from app_settings (should be set if using the app)
    let app_settings = db::get_app_settings()?;
    let maps_folder = match app_settings.map_search_folder {
        Some(maps_folder) => maps_folder,
        None => {
            return Err(eyre::eyre!(
                "Unable to download map files as the Map folder is not set"
            ))
        }
    };

    // Build up our local download folder, based off the maps folder
    // We put it in it's own sub-folder so it can be easily deleted/moved
    let mut download_file_path = PathBuf::new();
    download_file_path.push(maps_folder);
    download_file_path.push(DOWNLOAD_FOLDER);
    if !download_file_path.exists() {
        paths::create_folder(&download_file_path.display().to_string())?;
    }

    let mut map_count = 0;

    // Loop through the files and download/extract them
    for doomworld_file in doomworld_files {
        log::info!("Downloading map '{}'", doomworld_file.title.blue());

        // Work out the local file name to download to
        let mut zip_file_path = download_file_path.clone();
        zip_file_path.push(&doomworld_file.filename);
        let downloaded_file = zip_file_path.display().to_string();
        if paths::file_exists(&downloaded_file) {
            log::info!(
                "File exists, deleting so latest copy can be downloaded: {}",
                downloaded_file.yellow()
            );
            paths::delete_file(&downloaded_file)?;
        }

        // Work out where we are going to extract the downloaded file to (sub-folder of the download folder based off zip file name)
        let map_sub_folder = paths::extract_file_name_no_extension(&doomworld_file.filename);
        let extract_folder =
            paths::get_full_path(&download_file_path.display().to_string(), &map_sub_folder);
        // Nuke it if it exists, we want it clean
        if paths::folder_exists(&extract_folder) {
            paths::delete_folder(&extract_folder)?;
        }
        paths::create_folder(&extract_folder)?;

        // Work out the URL to download the file from
        let url = format!(
            "{}{}{}",
            DOWNLOAD_URL, doomworld_file.dir, doomworld_file.filename
        );

        // Download it locally
        if download_file(&url, &downloaded_file)? {
            // Extract it to the new sub folder
            let extracted_files =
                extract_zip(&zip_file_path.display().to_string(), &extract_folder)?;

            // Go through what was extracted look for maps
            for extracted_file in extracted_files {
                if files::map_file_extension(&extracted_file)? {
                    // Check if it's in the db...
                    let existing_map = db::get_map_by_path(&extracted_file);

                    // ...handle accordingly
                    match existing_map {
                        Ok(existing_map) => {
                            // Already exists, so just log it
                            log::info!(
                                "Map already exists, no need to add: {}",
                                existing_map.simple_display().yellow()
                            );
                            log::debug!("  Map {:?}", existing_map);
                        }
                        Err(_) => {
                            // Doesn't exist, so add it and prompt for profile
                            add_map_and_create_optional_profile(
                                &doomworld_file,
                                extracted_file,
                                &mut map_count,
                            )?;
                        }
                    };
                }
            }
        }
    }

    if map_count == 0 {
        return Err(eyre::eyre!("No maps were downloaded"));
    }

    let result_message = format!("Successfully added {} Maps", map_count);
    log::info!("{}", result_message.green());
    inquire::Text::new("Press any key to continue...").prompt_skippable()?;

    Ok(result_message)
}

fn add_map_and_create_optional_profile(
    doomworld_file: &DoomworldFile,
    extracted_file: String,
    map_count: &mut i32,
) -> Result<(), eyre::Error> {
    let map = data::Map {
        id: 0,
        title: doomworld_file.title.clone(),
        author: doomworld_file.author.clone(),
        path: extracted_file,
        doomworld_id: Some(doomworld_file.id),
        doomworld_url: Some(doomworld_file.url.clone()),
    };

    let add_result = db::add_map(&map)?;
    let add_map_id: i32 = add_result.last_insert_rowid().try_into().unwrap();
    log::info!("Added Map: {}", map.simple_display().blue());
    log::debug!("  Map {:?}", map);

    *map_count += 1;

    let prompt_result = inquire::Confirm::new(&format!(
        "Would you like to create a Profile for '{}'?",
        doomworld_file.title
    ))
    .with_default(false)
    .prompt_skippable()?;

    if let Some(true) = prompt_result {
        let add_profile_result = menu_profiles::add_profile(Some(add_map_id));
        match add_profile_result {
            Ok(_) => {
                log::info!("  Added Profile for Map: {}", map.simple_display().blue());
            }
            Err(e) => {
                log::error!(
                    "  Unable to add Profile for Map: {}",
                    map.simple_display().blue()
                );
                log::error!("  Error: {}", e);
            }
        }
    }

    Ok(())
}

async fn check_url(url: &str) -> Result<bool, eyre::Report> {
    log::debug!("  Sending request to: {}", url);
    let client = reqwest::Client::new();
    let response = client.head(url).send().await?;
    if response.status() != StatusCode::OK {
        log::debug!("  Response: {:?}", response);
        return Err(eyre::eyre!(
            "URL is not reachable or returned a non-OK status: '{}' for '{}'",
            response.status(),
            url
        ));
    }

    Ok(true)
}

fn download_file(url: &str, file_name: &str) -> Result<bool, eyre::Report> {
    log::info!("  Downloading file from '{}' to '{}'", url, file_name);
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        // Is the URL reachable?
        if !check_url(url).await? {
            return Ok(false);
        }

        // Looks to be, so download it
        let response = reqwest::get(url).await?;
        let mut file = paths::create_file(file_name)?;
        let mut content = Cursor::new(response.bytes().await?);
        std::io::copy(&mut content, &mut file)?;

        Ok(paths::file_exists(file_name))
    })
}

fn extract_zip(zip_path: &str, output_folder: &str) -> Result<Vec<String>, eyre::Report> {
    log::info!("  Extracting file '{}' to '{}'", zip_path, output_folder);

    // We want a list of the extracted files as the return value
    let mut extracted_files = Vec::new();

    // Open the ZIP file
    let reader = paths::open_file(zip_path)?;
    let mut archive = ZipArchive::new(reader)?;

    // Extract files
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let mut path = PathBuf::new();
        path.push(output_folder);
        let outpath = path.join(file.name());

        // Create folders
        if (file.name()).ends_with('/') {
            paths::create_folder(&outpath.display().to_string())?;
        } else {
            // Some files need folders create
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    paths::create_folder(&p.display().to_string())?;
                }
            }
            let mut outfile = paths::create_file(&outpath.display().to_string())?;
            log::debug!("    Extracting file: {}", outpath.display());
            std::io::copy(&mut file, &mut outfile)?;

            // Add the file to the list of extracted files
            extracted_files.push(outpath.display().to_string())
        }
    }

    Ok(extracted_files)
}
