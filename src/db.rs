use std::{fs, io};

use color_eyre::eyre::{self, Context};
use log::debug;
use sqlx::{
    migrate::{MigrateDatabase, Migrator},
    Sqlite, SqlitePool,
};

use crate::{data, paths};

const DB_URL: &str = "sqlite://sqlite.db";
const DB_FILE: &str = "sqlite.db";

static MIGRATOR: Migrator = sqlx::migrate!(); // this will pick up migrations from the ./migrations directory

async fn get_db() -> sqlx::Pool<Sqlite> {
    SqlitePool::connect(DB_URL).await.unwrap()
}
pub async fn database_exists() -> bool {
    Sqlite::database_exists(DB_URL).await.unwrap_or(false)
}

pub async fn create_db() -> Result<bool, eyre::Report> {
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
}

pub fn reset_db() -> Result<(), io::Error> {
    fs::remove_file(DB_FILE)
}

pub async fn is_empty_settings_table() -> Result<bool, eyre::Report> {
    let db = get_db().await;

    // Execute a query to check if the table is empty
    let result: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM settings")
        .fetch_one(&db)
        .await
        .wrap_err("Failed to check is Settings tasble is empty")?;

    // Determine if the table is empty
    Ok(result.0 == 0)
}

pub async fn add_engine(
    engine: &data::Engine,
) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let db = get_db().await;

    sqlx::query("INSERT INTO engines (path, version, game_engine_type) VALUES (?,?,?)")
        .bind(&engine.path)
        .bind(&engine.version)
        .bind(&engine.game_engine_type)
        .execute(&db)
        .await
        .wrap_err(format!("Failed to add engine '{:?}", engine))
}

pub async fn delete_engine(path: &str) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    sqlx::query("DELETE FROM engines WHERE path=$1 COLLATE NOCASE")
        .bind(path.to_lowercase())
        .execute(&db)
        .await
        .wrap_err(format!("Failed to delete engine '{}'", path))
}

pub async fn update_engine_version(
    id: i32,
    version: &str,
) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    sqlx::query("UPDATE engines SET version = $1 WHERE id=$3 COLLATE NOCASE")
        .bind(version)
        // .bind(Utc::now())
        .bind(id)
        .execute(&db)
        .await
        .wrap_err(format!(
            "Failed to update version '{}' for engine with id '{}'",
            version, id
        ))
}

pub async fn get_engines() -> Result<Vec<data::Engine>, eyre::Report> {
    let db = get_db().await;

    sqlx::query_as::<_, data::Engine>("SELECT * FROM engines")
        .fetch_all(&db)
        .await
        .wrap_err("Failed to get list of all engines")
}

pub async fn get_engine_by_id(id: i32) -> Result<data::Engine, eyre::Report> {
    let db = get_db().await;

    sqlx::query_as::<_, data::Engine>("SELECT * FROM engines WHERE id = ?")
        .bind(id)
        .fetch_one(&db)
        .await
        .wrap_err(format!("Failed to get engine with id '{}'", id))
}

pub async fn add_iwad(iwad: &data::Iwad) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let db = get_db().await;

    sqlx::query("INSERT INTO iwads (path, internal_wad_type) VALUES (?,?)")
        .bind(&iwad.path)
        .bind(&iwad.internal_wad_type)
        .execute(&db)
        .await
        .wrap_err(format!("Failed to add internal wad '{:?}", iwad))
}

pub async fn delete_iwad(path: &str) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    sqlx::query("DELETE FROM iwads WHERE path=$1 COLLATE NOCASE")
        .bind(path.to_lowercase())
        .execute(&db)
        .await
        .wrap_err(format!("Failed to delete iwad '{}'", path))
}

pub async fn get_iwads() -> Result<Vec<data::Iwad>, eyre::Report> {
    let db = get_db().await;

    sqlx::query_as::<_, data::Iwad>("SELECT * FROM iwads")
        .fetch_all(&db)
        .await
        .wrap_err("Failed to get list of all internal wads")
}

pub async fn get_iwad_by_id(id: i32) -> Result<data::Iwad, eyre::Report> {
    let db = get_db().await;

    sqlx::query_as::<_, data::Iwad>("SELECT * FROM iwads WHERE id = ?")
        .bind(id)
        .fetch_one(&db)
        .await
        .wrap_err(format!("Failed to get internal wad with id '{}'", id))
}

pub async fn get_pwads() -> Result<Vec<data::Pwad>, eyre::Report> {
    let db = get_db().await;

    sqlx::query_as::<_, data::Pwad>("SELECT * FROM pwads")
        .fetch_all(&db)
        .await
        .wrap_err("Failed to get list of all patch wads")
}

pub async fn add_pwad(pwad: &data::Pwad) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let db = get_db().await;

    sqlx::query("INSERT INTO pwads (name, path) VALUES (?,?)")
        .bind(&pwad.name)
        .bind(&pwad.path)
        .execute(&db)
        .await
        .wrap_err(format!("Failed to add patch wad '{:?}", pwad))
}

pub async fn delete_pwad(path: &str) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    sqlx::query("DELETE FROM pwads WHERE path=$1 COLLATE NOCASE")
        .bind(path.to_lowercase())
        .execute(&db)
        .await
        .wrap_err(format!("Failed to delete pwad '{}'", path))
}

pub async fn get_pwad_by_id(id: i32) -> Result<data::Pwad, eyre::Report> {
    let db = get_db().await;

    sqlx::query_as::<_, data::Pwad>("SELECT * FROM pwads WHERE id = ?")
        .bind(id)
        .fetch_one(&db)
        .await
        .wrap_err(format!("Failed to get patch wad with id '{}'", id))
}

pub async fn add_settings(
    settings: &data::Settings,
) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let db = get_db().await;

    sqlx::query(
        "INSERT INTO settings (active_profile_id, exe_search_folder, iwad_search_folder, pwad_search_folder) VALUES (?,?,?,?)",
    )
    .bind(settings.active_profile_id)
    .bind(&settings.exe_search_folder)
    .bind(&settings.iwad_search_folder)
    .bind(&settings.pwad_search_folder)
    .execute(&db)
    .await
    .wrap_err(format!(
        "Failed to add internal wad '{:?}", settings
    ))
}

pub async fn get_settings() -> Result<data::Settings, eyre::Report> {
    let db = get_db().await;

    sqlx::query_as::<_, data::Settings>("SELECT * FROM settings")
        .fetch_one(&db)
        .await
        .wrap_err("Failed to get settings".to_string())
}

pub async fn update_settings_active_profile(
    id: i32,
    active_profile_id: i32,
) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    sqlx::query("UPDATE settings SET active_profile_id = $1 WHERE id=$2 COLLATE NOCASE")
        .bind(active_profile_id)
        // .bind(Utc::now())
        .bind(id)
        .execute(&db)
        .await
        .wrap_err(format!(
            "Failed to update active_profile_id '{}' for settings with id '{}'",
            active_profile_id, id
        ))
}

pub async fn add_profile(
    profile: data::Profile,
) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    sqlx::query("INSERT INTO profiles (name, engine_id, iwad_id, pwad_id) VALUES (?,?,?,?)")
        .bind(&profile.name)
        .bind(profile.engine_id)
        .bind(profile.iwad_id)
        .bind(profile.pwad_id)
        .execute(&db)
        .await
        .wrap_err(format!("Failed to add profile '{:?}", profile))
}

pub async fn delete_profile(id: i32) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    sqlx::query("DELETE FROM profiles WHERE id=$1")
        .bind(id)
        .execute(&db)
        .await
        .wrap_err(format!("Failed to delete profile '{}'", id))
}

pub async fn get_profiles() -> Result<Vec<data::Profile>, eyre::Report> {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    sqlx::query_as::<_, data::Profile>("SELECT * FROM profiles")
        .fetch_all(&db)
        .await
        .wrap_err("Failed to get list of all profiles")
}

pub async fn get_profile_by_id(id: i32) -> Result<data::Profile, eyre::Report> {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    sqlx::query_as::<_, data::Profile>("SELECT * FROM profiles WHERE id = ?")
        .bind(id)
        .fetch_one(&db)
        .await
        .wrap_err(format!("Failed to get profile with id '{}'", id))
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
        engine_path: paths::extract_path(&engine.path),
        engine_file: paths::extract_file_name(&engine.path),
        engine_version: engine.version.clone(),
        iwad_path: paths::extract_path(&iwad.path),
        iwad_file: paths::extract_file_name(&iwad.path),
        pwad_path: paths::extract_path(&pwad.path),
        pwad_file: paths::extract_file_name(&pwad.path),
        additional_arguments: profile.additional_arguments.unwrap_or_default(),
    }
}

pub async fn get_profile_display_list() -> Result<Vec<data::ProfileDisplay>, eyre::Report> {
    let profiles = get_profiles().await?;
    let engines = get_engines().await?;
    let iwads = get_iwads().await?;
    let pwads = get_pwads().await?;

    let mut profile_list: Vec<data::ProfileDisplay> = Vec::new();
    for profile in profiles {
        let engine = engines
            .iter()
            .find(|e| e.id == profile.engine_id.unwrap())
            .unwrap();
        let iwad = iwads
            .iter()
            .find(|i| i.id == profile.iwad_id.unwrap())
            .unwrap();
        let pwad = pwads
            .iter()
            .find(|p| p.id == profile.pwad_id.unwrap())
            .unwrap();

        profile_list.push(get_profile_display(
            profile.clone(),
            engine.clone(),
            iwad.clone(),
            pwad.clone(),
        ));
    }

    Ok(profile_list)
}

pub async fn get_profile_display_by_id(id: i32) -> Result<data::ProfileDisplay, eyre::Report> {
    let profile = get_profile_by_id(id).await?;

    let engine = get_engine_by_id(profile.engine_id.unwrap()).await?;
    let iwad = get_iwad_by_id(profile.iwad_id.unwrap()).await?;
    let pwad = get_pwad_by_id(profile.pwad_id.unwrap()).await?;

    Ok(get_profile_display(profile, engine, iwad, pwad))
}
