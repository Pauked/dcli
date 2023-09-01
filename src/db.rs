use color_eyre::eyre::{Context, self};
use log::debug;
use sqlx::{Sqlite, migrate::{Migrator, MigrateDatabase}, SqlitePool};


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
