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

        sqlx::query_as::<_, data::Pwad>("SELECT * FROM pwads ORDER BY name")
            .fetch_all(&db)
            .await
            .wrap_err("Failed to get list of all patch wads")
    })
}

pub fn add_pwad(pwad: &data::Pwad) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query("INSERT INTO pwads (name, path) VALUES (?,?)")
            .bind(&pwad.name)
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

        sqlx::query("INSERT INTO profiles (name, engine_id, iwad_id, pwad_id, additional_arguments) VALUES (?,?,?,?,?)")
            .bind(&profile.name)
            .bind(profile.engine_id)
            .bind(profile.iwad_id)
            .bind(profile.pwad_id)
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

    sqlx::query("UPDATE profiles SET name = $1, engine_id = $2, iwad_id = $3, pwad_id = $4, additional_arguments = $5 WHERE id=$6 COLLATE NOCASE")
        .bind(&profile.name)
        .bind(profile.engine_id)
        .bind(profile.iwad_id)
        .bind(profile.pwad_id)
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
    pwad: data::Pwad,
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
        pwad_id: profile.pwad_id.unwrap_or(0),
        pwad_path: paths::extract_path(&pwad.path),
        pwad_file: paths::extract_file_name(&pwad.path),
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

        profile_list.push(get_profile_display(
            profile.clone(),
            engine.clone(),
            iwad.clone(),
            pwad.clone(),
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

    Ok(get_profile_display(profile, engine, iwad, pwad))
}

pub fn save_game_settings(
    game_settings: data::GameSettings,
) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    if game_settings.id == 0 {
        add_game_settings(game_settings)
    } else {
        update_game_settings(game_settings)
    }
}

fn add_game_settings(
    game_settings: data::GameSettings,
) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query(
            "INSERT INTO game_settings (comp_level, fast_monsters, no_monsters,
            respawn_monsters, warp, skill, turbo, timer, width, height, full_screen,
            windowed, additional_arguments) VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?)",
        )
        .bind(&game_settings.comp_level)
        .bind(game_settings.fast_monsters)
        .bind(game_settings.no_monsters)
        .bind(game_settings.respawn_monsters)
        .bind(&game_settings.warp)
        .bind(game_settings.skill)
        .bind(game_settings.turbo)
        .bind(game_settings.timer)
        .bind(game_settings.width)
        .bind(game_settings.height)
        .bind(game_settings.full_screen)
        .bind(game_settings.windowed)
        .bind(&game_settings.additional_arguments)
        .execute(&db)
        .await
        .wrap_err(format!("Failed to add app settings '{:?}", game_settings))
    })
}

fn update_game_settings(
    game_settings: data::GameSettings,
) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query(
            "UPDATE game_settings SET comp_level = $1,
                fast_monsters = $2, no_monsters = $3, respawn_monsters = $4,
                warp = $5, skill = $6, turbo = $7, timer = $8, width = $9, height = $10,
                full_screen = $11, windowed = $12, additional_arguments = $13
                WHERE id=$14",
        )
        .bind(&game_settings.comp_level)
        .bind(game_settings.fast_monsters)
        .bind(game_settings.no_monsters)
        .bind(game_settings.respawn_monsters)
        .bind(&game_settings.warp)
        .bind(game_settings.skill)
        .bind(game_settings.turbo)
        .bind(game_settings.timer)
        .bind(game_settings.width)
        .bind(game_settings.height)
        .bind(game_settings.full_screen)
        .bind(game_settings.windowed)
        .bind(&game_settings.additional_arguments)
        .bind(game_settings.id)
        .execute(&db)
        .await
        .wrap_err(format!(
            "Failed to update app settings '{:?}",
            game_settings
        ))
    })
}

pub fn get_game_settings() -> Result<data::GameSettings, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        let result = sqlx::query_as::<_, data::GameSettings>("SELECT * FROM game_settings")
            .fetch_one(&db)
            .await
            .wrap_err("Failed to get game settings".to_string());

        match result {
            Ok(game_settings) => Ok(game_settings),
            Err(_) => Ok(data::GameSettings::default()),
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
