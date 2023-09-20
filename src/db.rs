use std::{fs, io};

use color_eyre::eyre::{self, Context};
use log::debug;
use sqlx::{
    migrate::{MigrateDatabase, Migrator},
    Sqlite, SqlitePool,
};

use crate::{constants, data, paths};

const DB_URL: &str = "sqlite://sqlite.db";
const DB_FILE: &str = "sqlite.db";

static MIGRATOR: Migrator = sqlx::migrate!(); // this will pick up migrations from the ./migrations directory

async fn get_db() -> sqlx::Pool<Sqlite> {
    SqlitePool::connect(DB_URL).await.unwrap()
}
pub fn database_exists() -> bool {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    runtime.block_on(async { Sqlite::database_exists(DB_URL).await.unwrap_or(false) })
}

pub fn create_db() -> Result<bool, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
            debug!("Creating database {}", DB_URL);
            Sqlite::create_database(DB_URL)
                .await
                .wrap_err("Unable to create database")?;

            debug!("Successfully created database");
        } else {
            debug!("Database already exists");
        }

        let db = get_db().await;
        MIGRATOR
            .run(&db)
            .await
            .wrap_err("Unable to run database migrations")?;
        debug!("Migration success");
        Ok(true)
    })
}

pub fn reset_db() -> Result<(), io::Error> {
    fs::remove_file(DB_FILE)
}

pub fn is_empty_app_settings_table() -> Result<bool, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        // Execute a query to check if the table is empty
        let result: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM app_settings")
            .fetch_one(&db)
            .await
            .wrap_err("Failed to check if App Settings table is empty")?;

        // Determine if the table is empty
        Ok(result.0 == 0)
    })
}

pub fn add_engine(engine: &data::Engine) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query(
            "INSERT INTO engines (app_name, path, version, game_engine_type) VALUES (?,?,?,?)",
        )
        .bind(&engine.app_name)
        .bind(&engine.path)
        .bind(&engine.version)
        .bind(&engine.game_engine_type)
        .execute(&db)
        .await
        .wrap_err(format!("Failed to add engine '{:?}", engine))
    })
}

pub fn delete_engine(path: &str) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query("DELETE FROM engines WHERE path=$1 COLLATE NOCASE")
            .bind(path.to_lowercase())
            .execute(&db)
            .await
            .wrap_err(format!("Failed to delete engine '{}'", path))
    })
}

pub fn update_engine_version(
    id: i32,
    version: &str,
) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query("UPDATE engines SET version = $1 WHERE id=$2 COLLATE NOCASE")
            .bind(version)
            // .bind(Utc::now())
            .bind(id)
            .execute(&db)
            .await
            .wrap_err(format!(
                "Failed to update version '{}' for engine with id '{}'",
                version, id
            ))
    })
}

pub fn get_engines() -> Result<Vec<data::Engine>, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query_as::<_, data::Engine>("SELECT * FROM engines ORDER BY app_name")
            .fetch_all(&db)
            .await
            .wrap_err("Failed to get list of all engines")
    })
}

pub fn get_engine_by_id(id: i32) -> Result<data::Engine, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query_as::<_, data::Engine>("SELECT * FROM engines WHERE id = ?")
            .bind(id)
            .fetch_one(&db)
            .await
            .wrap_err(format!("Failed to get engine with id '{}'", id))
    })
}

pub fn add_iwad(iwad: &data::Iwad) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query("INSERT INTO iwads (path, internal_wad_type) VALUES (?,?)")
            .bind(&iwad.path)
            .bind(&iwad.internal_wad_type)
            .execute(&db)
            .await
            .wrap_err(format!("Failed to add internal wad '{:?}", iwad))
    })
}

pub fn delete_iwad(path: &str) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query("DELETE FROM iwads WHERE path=$1 COLLATE NOCASE")
            .bind(path.to_lowercase())
            .execute(&db)
            .await
            .wrap_err(format!("Failed to delete iwad '{}'", path))
    })
}

pub fn get_iwads() -> Result<Vec<data::Iwad>, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query_as::<_, data::Iwad>("SELECT * FROM iwads ORDER BY internal_wad_type")
            .fetch_all(&db)
            .await
            .wrap_err("Failed to get list of all internal wads")
    })
}

pub fn get_iwad_by_id(id: i32) -> Result<data::Iwad, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query_as::<_, data::Iwad>("SELECT * FROM iwads WHERE id = ?")
            .bind(id)
            .fetch_one(&db)
            .await
            .wrap_err(format!("Failed to get internal wad with id '{}'", id))
    })
}

pub fn get_pwads() -> Result<Vec<data::Pwad>, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query_as::<_, data::Pwad>("SELECT * FROM pwads ORDER BY title")
            .fetch_all(&db)
            .await
            .wrap_err("Failed to get list of all patch wads")
    })
}

pub fn add_pwad(pwad: &data::Pwad) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query("INSERT INTO pwads (title, author, path) VALUES (?,?,?)")
            .bind(&pwad.title)
            .bind(&pwad.author)
            .bind(&pwad.path)
            .execute(&db)
            .await
            .wrap_err(format!("Failed to add patch wad '{:?}", pwad))
    })
}

pub fn delete_pwad(path: &str) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query("DELETE FROM pwads WHERE path=$1 COLLATE NOCASE")
            .bind(path.to_lowercase())
            .execute(&db)
            .await
            .wrap_err(format!("Failed to delete pwad '{}'", path))
    })
}

pub fn get_pwad_by_id(id: i32) -> Result<data::Pwad, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query_as::<_, data::Pwad>("SELECT * FROM pwads WHERE id = ?")
            .bind(id)
            .fetch_one(&db)
            .await
            .wrap_err(format!("Failed to get patch wad with id '{}'", id))
    })
}

pub fn get_pwads_by_ids(pwad_ids: data::PwadIds) -> Result<Vec<data::Pwad>, eyre::Report> {
    let mut result = vec![];
    let pwad_ids_array = [pwad_ids.0, pwad_ids.1, pwad_ids.2, pwad_ids.3, pwad_ids.4];
    for &id in &pwad_ids_array {
        if id != 0 {
            result.push(get_pwad_by_id(id)?);
        }
    }

    Ok(result)
}

pub fn save_app_settings(
    app_settings: data::AppSettings,
) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    if app_settings.id == 0 {
        add_app_settings(app_settings)
    } else {
        update_app_settings(app_settings)
    }
}

fn add_app_settings(
    app_settings: data::AppSettings,
) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query(
            "INSERT INTO app_settings (active_profile_id, last_profile_id, exe_search_folder,
                iwad_search_folder, pwad_search_folder, map_editor_search_folder, active_map_editor_id) VALUES (?,?,?,?,?,?,?)",
        )
        .bind(app_settings.active_profile_id)
        .bind(app_settings.last_profile_id)
        .bind(&app_settings.exe_search_folder)
        .bind(&app_settings.iwad_search_folder)
        .bind(&app_settings.pwad_search_folder)
        .bind(&app_settings.map_editor_search_folder)
        .bind(app_settings.active_map_editor_id)
        .execute(&db)
        .await
        .wrap_err(format!("Failed to add app settings '{:?}", app_settings))
    })
}

fn update_app_settings(
    app_settings: data::AppSettings,
) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query("UPDATE app_settings SET active_profile_id = $1, last_profile_id = $2, exe_search_folder = $3,
        iwad_search_folder = $4, pwad_search_folder = $5, map_editor_search_folder = $6, active_map_editor_id = $7
        WHERE id = $8 COLLATE NOCASE")
            .bind(app_settings.active_profile_id)
            .bind(app_settings.last_profile_id)
            .bind(&app_settings.exe_search_folder)
            .bind(&app_settings.iwad_search_folder)
            .bind(&app_settings.pwad_search_folder)
            .bind(&app_settings.map_editor_search_folder)
            .bind(app_settings.active_map_editor_id)
            .bind(app_settings.id)
            .execute(&db)
            .await
            .wrap_err(format!(
                "Failed to update app settings '{:?}", app_settings
            ))
    })
}

pub fn get_app_settings() -> Result<data::AppSettings, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        let result = sqlx::query_as::<_, data::AppSettings>("SELECT * FROM app_settings")
            .fetch_one(&db)
            .await
            .wrap_err("Failed to get settings".to_string());

        match result {
            Ok(app_settings) => Ok(app_settings),
            Err(_) => Ok(data::AppSettings::default()),
        }
    })
}

pub fn get_app_settings_display() -> Result<data::AppSettingsDisplay, eyre::Report> {
    let app_settings = get_app_settings()?;
    let active_profile: String = match app_settings.active_profile_id {
        Some(id) => {
            let profile = get_profile_display_by_id(id)?;
            profile.to_string()
        }
        None => constants::DEFAULT_NOT_SET.to_string(),
    };
    let last_profile = match app_settings.last_profile_id {
        Some(id) => {
            let profile = get_profile_display_by_id(id)?;
            profile.to_string()
        }
        None => constants::DEFAULT_NOT_SET.to_string(),
    };
    let active_map_editor = match app_settings.active_map_editor_id {
        Some(id) => {
            let map_editor = get_map_editor_by_id(id)?;
            map_editor.to_string()
        }
        None => constants::DEFAULT_NOT_SET.to_string(),
    };
    let exe_search_folder = app_settings
        .exe_search_folder
        .unwrap_or(constants::DEFAULT_NOT_SET.to_string());
    let iwad_search_folder = app_settings
        .iwad_search_folder
        .unwrap_or(constants::DEFAULT_NOT_SET.to_string());
    let pwad_search_folder = app_settings
        .pwad_search_folder
        .unwrap_or(constants::DEFAULT_NOT_SET.to_string());
    let map_editor_search_folder = app_settings
        .map_editor_search_folder
        .unwrap_or(constants::DEFAULT_NOT_SET.to_string());

    Ok(data::AppSettingsDisplay {
        active_profile,
        last_profile,
        active_map_editor,
        exe_search_folder,
        iwad_search_folder,
        pwad_search_folder,
        map_editor_search_folder,
    })
}

pub fn add_profile(
    profile: data::Profile,
) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query(
            "INSERT INTO profiles (name, engine_id, iwad_id,
            pwad_id, pwad_id2, pwad_id3, pwad_id4, pwad_id5, additional_arguments)
            VALUES (?,?,?,?,?,?,?,?,?)",
        )
        .bind(&profile.name)
        .bind(profile.engine_id)
        .bind(profile.iwad_id)
        .bind(profile.pwad_id)
        .bind(profile.pwad_id2)
        .bind(profile.pwad_id3)
        .bind(profile.pwad_id4)
        .bind(profile.pwad_id5)
        .bind(&profile.additional_arguments)
        .execute(&db)
        .await
        .wrap_err(format!("Failed to add profile '{:?}", profile))
    })
}

pub fn update_profile(
    profile: data::Profile,
) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query(
            "UPDATE profiles SET name = $1, engine_id = $2, iwad_id = $3,
    pwad_id = $4, pwad_id2 = $5, pwad_id3 = $6, pwad_id4 = $7, pwad_id5 = $8,
    additional_arguments = $9 WHERE id=$10 COLLATE NOCASE",
        )
        .bind(&profile.name)
        .bind(profile.engine_id)
        .bind(profile.iwad_id)
        .bind(profile.pwad_id)
        .bind(profile.pwad_id2)
        .bind(profile.pwad_id3)
        .bind(profile.pwad_id4)
        .bind(profile.pwad_id5)
        .bind(profile.additional_arguments)
        .bind(profile.id)
        .execute(&db)
        .await
        .wrap_err(format!(
            "Failed to update profile '{}', id '{}'",
            profile.name, profile.id
        ))
    })
}

pub fn delete_profile(id: i32) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query("DELETE FROM profiles WHERE id=$1")
            .bind(id)
            .execute(&db)
            .await
            .wrap_err(format!("Failed to delete profile with id '{}'", id))
    })
}

pub fn get_profiles() -> Result<Vec<data::Profile>, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query_as::<_, data::Profile>("SELECT * FROM profiles ORDER BY name")
            .fetch_all(&db)
            .await
            .wrap_err("Failed to get list of all profiles")
    })
}

pub fn get_profile_by_id(id: i32) -> Result<data::Profile, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query_as::<_, data::Profile>("SELECT * FROM profiles WHERE id = ?")
            .bind(id)
            .fetch_one(&db)
            .await
            .wrap_err(format!("Failed to get profile with id '{}'", id))
    })
}

fn get_profile_display(
    profile: data::Profile,
    engine: data::Engine,
    iwad: data::Iwad,
    pwads: Vec<data::Pwad>,
) -> data::ProfileDisplay {
    data::ProfileDisplay {
        id: profile.id,
        name: profile.name,
        engine_id: profile.engine_id.unwrap_or(0),
        engine_path: paths::extract_path(&engine.path),
        engine_file: paths::extract_file_name(&engine.path),
        engine_version: engine.version.clone(),
        iwad_id: profile.iwad_id.unwrap_or(0),
        iwad_path: paths::extract_path(&iwad.path),
        iwad_file: paths::extract_file_name(&iwad.path),
        pwad_ids: (
            profile.pwad_id.unwrap_or(0),
            profile.pwad_id2.unwrap_or(0),
            profile.pwad_id3.unwrap_or(0),
            profile.pwad_id4.unwrap_or(0),
            profile.pwad_id5.unwrap_or(0),
        ),
        pwad_paths: (
            paths::extract_path(&pwads[0].path),
            paths::extract_path(&pwads[1].path),
            paths::extract_path(&pwads[2].path),
            paths::extract_path(&pwads[3].path),
            paths::extract_path(&pwads[4].path),
        ),
        pwad_files: (
            paths::extract_file_name(&pwads[0].path),
            paths::extract_file_name(&pwads[1].path),
            paths::extract_file_name(&pwads[2].path),
            paths::extract_file_name(&pwads[3].path),
            paths::extract_file_name(&pwads[4].path),
        ),
        additional_arguments: profile.additional_arguments.unwrap_or_default(),
    }
}

pub fn get_profile_display_list() -> Result<Vec<data::ProfileDisplay>, eyre::Report> {
    let profiles = get_profiles()?;
    let engines = get_engines()?;
    let iwads = get_iwads()?;
    let pwads = get_pwads()?;

    let default_engine = data::Engine::default();
    let default_iwad = data::Iwad::default();
    let default_pwad = data::Pwad::default();

    let mut profile_list: Vec<data::ProfileDisplay> = Vec::new();
    for profile in profiles {
        let engine = engines
            .iter()
            .find(|e| e.id == profile.engine_id.unwrap_or(0))
            .unwrap_or(&default_engine);
        let iwad = iwads
            .iter()
            .find(|i| i.id == profile.iwad_id.unwrap_or(0))
            .unwrap_or(&default_iwad);
        let pwad = pwads
            .iter()
            .find(|p| p.id == profile.pwad_id.unwrap_or(0))
            .unwrap_or(&default_pwad);
        let pwad2 = pwads
            .iter()
            .find(|p| p.id == profile.pwad_id2.unwrap_or(0))
            .unwrap_or(&default_pwad);
        let pwad3 = pwads
            .iter()
            .find(|p| p.id == profile.pwad_id3.unwrap_or(0))
            .unwrap_or(&default_pwad);
        let pwad4 = pwads
            .iter()
            .find(|p| p.id == profile.pwad_id4.unwrap_or(0))
            .unwrap_or(&default_pwad);
        let pwad5 = pwads
            .iter()
            .find(|p| p.id == profile.pwad_id5.unwrap_or(0))
            .unwrap_or(&default_pwad);

        profile_list.push(get_profile_display(
            profile.clone(),
            engine.clone(),
            iwad.clone(),
            vec![
                pwad.clone(),
                pwad2.clone(),
                pwad3.clone(),
                pwad4.clone(),
                pwad5.clone(),
            ],
        ));
    }

    Ok(profile_list)
}

pub fn get_profile_display_by_id(id: i32) -> Result<data::ProfileDisplay, eyre::Report> {
    let profile = get_profile_by_id(id)?;

    let engine = match profile.engine_id {
        Some(id) => get_engine_by_id(id)?,
        None => data::Engine::default(),
    };
    let iwad = match profile.iwad_id {
        Some(id) => get_iwad_by_id(id)?,
        None => data::Iwad::default(),
    };
    let pwad = match profile.pwad_id {
        Some(id) => get_pwad_by_id(id)?,
        None => data::Pwad::default(),
    };
    let pwad2 = match profile.pwad_id2 {
        Some(id) => get_pwad_by_id(id)?,
        None => data::Pwad::default(),
    };
    let pwad3 = match profile.pwad_id3 {
        Some(id) => get_pwad_by_id(id)?,
        None => data::Pwad::default(),
    };
    let pwad4 = match profile.pwad_id4 {
        Some(id) => get_pwad_by_id(id)?,
        None => data::Pwad::default(),
    };
    let pwad5 = match profile.pwad_id5 {
        Some(id) => get_pwad_by_id(id)?,
        None => data::Pwad::default(),
    };

    Ok(get_profile_display(
        profile,
        engine,
        iwad,
        vec![pwad, pwad2, pwad3, pwad4, pwad5],
    ))
}

pub fn save_play_settings(
    play_settings: data::PlaySettings,
) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    if play_settings.id == 0 {
        add_play_settings(play_settings)
    } else {
        update_play_settings(play_settings)
    }
}

fn add_play_settings(
    play_settings: data::PlaySettings,
) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query(
            "INSERT INTO play_settings (comp_level, fast_monsters, no_monsters,
            respawn_monsters, warp, skill, turbo, timer, width, height, full_screen,
            windowed, additional_arguments) VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?)",
        )
        .bind(&play_settings.comp_level)
        .bind(play_settings.fast_monsters)
        .bind(play_settings.no_monsters)
        .bind(play_settings.respawn_monsters)
        .bind(&play_settings.warp)
        .bind(play_settings.skill)
        .bind(play_settings.turbo)
        .bind(play_settings.timer)
        .bind(play_settings.width)
        .bind(play_settings.height)
        .bind(play_settings.full_screen)
        .bind(play_settings.windowed)
        .bind(&play_settings.additional_arguments)
        .execute(&db)
        .await
        .wrap_err(format!("Failed to add app settings '{:?}", play_settings))
    })
}

fn update_play_settings(
    play_settings: data::PlaySettings,
) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query(
            "UPDATE play_settings SET comp_level = $1,
                fast_monsters = $2, no_monsters = $3, respawn_monsters = $4,
                warp = $5, skill = $6, turbo = $7, timer = $8, width = $9, height = $10,
                full_screen = $11, windowed = $12, additional_arguments = $13
                WHERE id=$14",
        )
        .bind(&play_settings.comp_level)
        .bind(play_settings.fast_monsters)
        .bind(play_settings.no_monsters)
        .bind(play_settings.respawn_monsters)
        .bind(&play_settings.warp)
        .bind(play_settings.skill)
        .bind(play_settings.turbo)
        .bind(play_settings.timer)
        .bind(play_settings.width)
        .bind(play_settings.height)
        .bind(play_settings.full_screen)
        .bind(play_settings.windowed)
        .bind(&play_settings.additional_arguments)
        .bind(play_settings.id)
        .execute(&db)
        .await
        .wrap_err(format!(
            "Failed to update app settings '{:?}",
            play_settings
        ))
    })
}

pub fn get_play_settings() -> Result<data::PlaySettings, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        let result = sqlx::query_as::<_, data::PlaySettings>("SELECT * FROM play_settings")
            .fetch_one(&db)
            .await
            .wrap_err("Failed to get play settings".to_string());

        match result {
            Ok(play_settings) => Ok(play_settings),
            Err(_) => Ok(data::PlaySettings::default()),
        }
    })
}

pub fn add_map_editor(
    map_editor: &data::MapEditor,
) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query("INSERT INTO map_editors (app_name, path, version, load_file_argument, additional_arguments) VALUES (?,?,?,?,?)")
            .bind(&map_editor.app_name)
            .bind(&map_editor.path)
            .bind(&map_editor.version)
            .bind(&map_editor.load_file_argument)
            .bind(&map_editor.additional_arguments)
            .execute(&db)
            .await
            .wrap_err(format!("Failed to add map editor '{:?}", map_editor))
    })
}

pub fn delete_map_editor(id: i32) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query("DELETE FROM map_editors WHERE id=$1")
            .bind(id)
            .execute(&db)
            .await
            .wrap_err(format!("Failed to delete map editor with id '{}'", id))
    })
}

pub fn update_map_editor_version(
    id: i32,
    version: &str,
) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query("UPDATE engines SET version = $1 WHERE id=$2 COLLATE NOCASE")
            .bind(version)
            .bind(id)
            .execute(&db)
            .await
            .wrap_err(format!(
                "Failed to update version '{}' for engine with id '{}'",
                version, id
            ))
    })
}

pub fn get_map_editors() -> Result<Vec<data::MapEditor>, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query_as::<_, data::MapEditor>("SELECT * FROM map_editors ORDER BY app_name")
            .fetch_all(&db)
            .await
            .wrap_err("Failed to get list of all map editors")
    })
}

pub fn get_map_editor_by_id(id: i32) -> Result<data::MapEditor, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query_as::<_, data::MapEditor>("SELECT * FROM map_editors WHERE id = ?")
            .bind(id)
            .fetch_one(&db)
            .await
            .wrap_err(format!("Failed to get map editor with id '{}'", id))
    })
}
