use std::{fs, io};

use color_eyre::eyre::{self, Context};
use log::debug;
use sqlx::{
    migrate::{MigrateDatabase, Migrator},
    Sqlite, SqlitePool,
};

use crate::data;

const DB_URL: &str = "sqlite://sqlite.db";
const DB_FILE: &str = "sqlite.db";

static MIGRATOR: Migrator = sqlx::migrate!(); // this will pick up migrations from the ./migrations directory

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

    let db = SqlitePool::connect(DB_URL).await.unwrap();
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
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    // Execute a query to check if the table is empty
    let result: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM settings")
        .fetch_one(&db)
        .await
        .wrap_err("Failed to check is Settings tasble is empty")?;

    // Determine if the table is empty
    Ok(result.0 == 0)
}

pub async fn add_engine(engine: &data::Engine) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    sqlx::query(
        "INSERT INTO engines (path, version, game_engine_type) VALUES (?,?,?)",
    )
    .bind(&engine.path)
    .bind(&engine.version)
    .bind(&engine.game_engine_type)
    .execute(&db)
    .await
    .wrap_err(format!(
        "Failed to add engine '{:?}", engine
    ))
}

pub async fn get_engines() -> Result<Vec<data::Engine>, eyre::Report> {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    sqlx::query_as::<_, data::Engine>("SELECT * FROM engines")
        .fetch_all(&db)
        .await
        .wrap_err("Failed to get list of all engines")
}

pub async fn get_engine_by_id(id: i32) -> Result<data::Engine, eyre::Report> {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    sqlx::query_as::<_, data::Engine>("SELECT * FROM engines WHERE id = ?")
        .bind(id)
        .fetch_one(&db)
        .await
        .wrap_err(format!("Failed to get engine with id '{}'", id))
}

pub async fn add_iwad(iwad: &data::Iwad) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    sqlx::query(
        "INSERT INTO iwads (path, internal_wad_type) VALUES (?,?)",
    )
    .bind(&iwad.path)
    .bind(&iwad.internal_wad_type)
    .execute(&db)
    .await
    .wrap_err(format!(
        "Failed to add internal wad '{:?}", iwad
    ))
}

pub async fn get_iwads() -> Result<Vec<data::Iwad>, eyre::Report> {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    sqlx::query_as::<_, data::Iwad>("SELECT * FROM iwads")
        .fetch_all(&db)
        .await
        .wrap_err("Failed to get list of all internal wads")
}

pub async fn get_iwad_by_id(id: i32) -> Result<data::Iwad, eyre::Report> {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    sqlx::query_as::<_, data::Iwad>("SELECT * FROM iwads WHERE id = ?")
        .bind(id)
        .fetch_one(&db)
        .await
        .wrap_err(format!("Failed to get internal wad with id '{}'", id))
}

pub async fn get_pwads() -> Result<Vec<data::Pwad>, eyre::Report> {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    sqlx::query_as::<_, data::Pwad>("SELECT * FROM pwads")
        .fetch_all(&db)
        .await
        .wrap_err("Failed to get list of all patch wads")
}

pub async fn add_pwad(pwad: &data::Pwad) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    sqlx::query(
        "INSERT INTO pwads (name, path) VALUES (?,?)",
    )
    .bind(&pwad.name)
    .bind(&pwad.path)
    .execute(&db)
    .await
    .wrap_err(format!(
        "Failed to add patch wad '{:?}", pwad
    ))
}

pub async fn get_pwad_by_id(id: i32) -> Result<data::Pwad, eyre::Report> {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    sqlx::query_as::<_, data::Pwad>("SELECT * FROM pwads WHERE id = ?")
        .bind(id)
        .fetch_one(&db)
        .await
        .wrap_err(format!("Failed to get patch wad with id '{}'", id))
}

pub async fn add_settings(settings: &data::Settings) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

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
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    sqlx::query_as::<_, data::Settings>("SELECT * FROM settings")
        .fetch_one(&db)
        .await
        .wrap_err("Failed to get settings".to_string())
}

pub async fn add_profile(profile: data::Profile) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    sqlx::query(
        "INSERT INTO profiles (name, engine_id, iwad_id, pwad_id) VALUES (?,?,?,?)",
    )
    .bind(&profile.name)
    .bind(profile.engine_id)
    .bind(profile.iwad_id)
    .bind(profile.pwad_id)
    .execute(&db)
    .await
    .wrap_err(format!(
        "Failed to add profile '{:?}", profile
    ))
}

pub async fn get_profiles() -> Result<Vec<data::Profile>, eyre::Report> {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    sqlx::query_as::<_, data::Profile>("SELECT * FROM profiles")
        .fetch_all(&db)
        .await
        .wrap_err("Failed to get list of all profiles")
}
