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