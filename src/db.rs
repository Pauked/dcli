use chrono::Utc;
use color_eyre::eyre::{self, Context};
use log::debug;
use sqlx::{
    migrate::{MigrateDatabase, Migrator},
    Sqlite, SqlitePool,
};

use crate::{constants, data, paths};

const DB_URL: &str = "sqlite://dcli.db";
const DB_FILE: &str = "dcli.db";

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

pub fn reset_db() -> Result<(), std::io::Error> {
    paths::delete_file(DB_FILE)
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
            "INSERT INTO engines (app_name, path, internal_path, version, game_engine_type) VALUES (?,?,?,?,?)",
        )
        .bind(&engine.app_name)
        .bind(&engine.path)
        .bind(&engine.internal_path)
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

        sqlx::query("UPDATE engines SET version = $1 WHERE id=$2")
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

pub fn get_engine_by_path(path: &str) -> Result<data::Engine, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query_as::<_, data::Engine>("SELECT * FROM engines WHERE path = $1 COLLATE NOCASE")
            .bind(path.to_lowercase())
            .fetch_one(&db)
            .await
            .wrap_err(format!("Failed to get engine with path '{}'", path))
    })
}

pub fn is_engine_linked_to_profiles(id: i32) -> Result<bool, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        let result: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM profiles WHERE engine_id = ?")
            .bind(id)
            .fetch_one(&db)
            .await
            .wrap_err("Failed to check if Engines linked to any Profiles")?;

        Ok(result.0 > 0)
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

pub fn get_iwad_by_path(path: &str) -> Result<data::Iwad, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query_as::<_, data::Iwad>("SELECT * FROM iwads WHERE path = $1 COLLATE NOCASE")
            .bind(path.to_lowercase())
            .fetch_one(&db)
            .await
            .wrap_err(format!("Failed to get IWAD with path '{}'", path))
    })
}

pub fn is_iwad_linked_to_profiles(id: i32) -> Result<bool, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        let result: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM profiles WHERE iwad_id = ?")
            .bind(id)
            .fetch_one(&db)
            .await
            .wrap_err("Failed to check if IWAD linked to any Profiles")?;

        Ok(result.0 > 0)
    })
}

pub fn get_maps() -> Result<Vec<data::Map>, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query_as::<_, data::Map>("SELECT * FROM maps ORDER BY title")
            .fetch_all(&db)
            .await
            .wrap_err("Failed to get list of all maps")
    })
}

pub fn add_map(map: &data::Map) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query("INSERT INTO maps (title, author, path, doomworld_id, doomworld_url) VALUES (?,?,?,?,?)")
            .bind(&map.title)
            .bind(&map.author)
            .bind(&map.path)
            .bind(map.doomworld_id)
            .bind(&map.doomworld_url)
            .execute(&db)
            .await
            .wrap_err(format!("Failed to add map '{:?}", map))
    })
}

pub fn update_map(map: data::Map) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query(
            "UPDATE maps SET title = $2, author = $3, path = $4,
            doomworld_id = $5, doomworld_url = $6
            WHERE id = $1",
        )
        .bind(map.id)
        .bind(&map.title)
        .bind(&map.author)
        .bind(&map.path)
        .bind(map.doomworld_id)
        .bind(&map.doomworld_url)
        .execute(&db)
        .await
        .wrap_err(format!("Failed to update map '{:?}", map))
    })
}

pub fn delete_map(path: &str) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query("DELETE FROM maps WHERE path = $1 COLLATE NOCASE")
            .bind(path.to_lowercase())
            .execute(&db)
            .await
            .wrap_err(format!("Failed to delete map '{}'", path))
    })
}

pub fn get_map_by_id(id: i32) -> Result<data::Map, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query_as::<_, data::Map>("SELECT * FROM maps WHERE id = ?")
            .bind(id)
            .fetch_one(&db)
            .await
            .wrap_err(format!("Failed to get map with id '{}'", id))
    })
}

pub fn get_maps_by_ids(map_ids: data::MapIds) -> Result<Vec<data::Map>, eyre::Report> {
    let mut result = vec![];
    let map_ids_array = [map_ids.0, map_ids.1, map_ids.2, map_ids.3, map_ids.4];
    for &id in &map_ids_array {
        if id != 0 {
            result.push(get_map_by_id(id)?);
        }
    }

    Ok(result)
}

pub fn get_map_by_path(path: &str) -> Result<data::Map, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query_as::<_, data::Map>("SELECT * FROM maps WHERE path = $1 COLLATE NOCASE")
            .bind(path.to_lowercase())
            .fetch_one(&db)
            .await
            .wrap_err(format!("Failed to get Map with path '{}'", path))
    })
}

pub fn is_map_linked_to_profiles(id: i32) -> Result<bool, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        let result: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM profiles WHERE map_id = $1 OR map_id2 = $1 OR map_id3 = $1 OR map_id4 = $1 OR map_id5 = $1")
            .bind(id) // The same ID is bound to all placeholders
            .fetch_one(&db)
            .await
            .wrap_err("Failed to check if Map is linked to any Profiles")?;

        Ok(result.0 > 0)
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
            "INSERT INTO app_settings (default_profile_id, last_profile_id, default_engine_id,
                default_iwad_id, default_editor_id, engine_search_folder, iwad_search_folder,
                map_search_folder, editor_search_folder, menu_mode, use_doomworld_api)
                VALUES (?,?,?,?,?,?,?,?,?,?,?)",
        )
        .bind(app_settings.default_profile_id)
        .bind(app_settings.last_profile_id)
        .bind(app_settings.default_engine_id)
        .bind(app_settings.default_iwad_id)
        .bind(app_settings.default_editor_id)
        .bind(&app_settings.engine_search_folder)
        .bind(&app_settings.iwad_search_folder)
        .bind(&app_settings.map_search_folder)
        .bind(&app_settings.editor_search_folder)
        .bind(&app_settings.menu_mode)
        .bind(app_settings.use_doomworld_api)
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

        sqlx::query(
            "UPDATE app_settings SET default_profile_id = $2, last_profile_id = $3,
        default_engine_id = $4, default_iwad_id = $5, default_editor_id = $6,
        engine_search_folder = $7, iwad_search_folder = $8, map_search_folder = $9,
        editor_search_folder = $10, menu_mode = $11, use_doomworld_api = $12
        WHERE id = $1",
        )
        .bind(app_settings.id)
        .bind(app_settings.default_profile_id)
        .bind(app_settings.last_profile_id)
        .bind(app_settings.default_engine_id)
        .bind(app_settings.default_iwad_id)
        .bind(app_settings.default_editor_id)
        .bind(&app_settings.engine_search_folder)
        .bind(&app_settings.iwad_search_folder)
        .bind(&app_settings.map_search_folder)
        .bind(&app_settings.editor_search_folder)
        .bind(&app_settings.menu_mode)
        .bind(app_settings.use_doomworld_api)
        .execute(&db)
        .await
        .wrap_err(format!("Failed to update app settings '{:?}", app_settings))
    })
}

pub fn get_app_settings() -> Result<data::AppSettings, eyre::Report> {
    if !database_exists() {
        return Ok(data::AppSettings::default());
    }

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
    let default_profile: String = match app_settings.default_profile_id {
        Some(id) => {
            let profile = get_profile_display_by_id(id)?;
            profile.simple_display()
        }
        None => constants::DEFAULT_NOT_SET.to_string(),
    };
    let last_profile = match app_settings.last_profile_id {
        Some(id) => {
            let profile = get_profile_display_by_id(id)?;
            profile.simple_display()
        }
        None => constants::DEFAULT_NOT_SET.to_string(),
    };
    let default_engine = match app_settings.default_engine_id {
        Some(id) => {
            let engine = get_engine_by_id(id)?;
            engine.simple_display()
        }
        None => constants::DEFAULT_NOT_SET.to_string(),
    };
    let default_iwad = match app_settings.default_iwad_id {
        Some(id) => {
            let iwad = get_iwad_by_id(id)?;
            iwad.simple_display()
        }
        None => constants::DEFAULT_NOT_SET.to_string(),
    };
    let default_editor = match app_settings.default_editor_id {
        Some(id) => {
            let editor = get_editor_by_id(id)?;
            editor.simple_display()
        }
        None => constants::DEFAULT_NOT_SET.to_string(),
    };
    let engine_search_folder = app_settings
        .engine_search_folder
        .unwrap_or(constants::DEFAULT_NOT_SET.to_string());
    let iwad_search_folder = app_settings
        .iwad_search_folder
        .unwrap_or(constants::DEFAULT_NOT_SET.to_string());
    let map_search_folder = app_settings
        .map_search_folder
        .unwrap_or(constants::DEFAULT_NOT_SET.to_string());
    let editor_search_folder = app_settings
        .editor_search_folder
        .unwrap_or(constants::DEFAULT_NOT_SET.to_string());

    Ok(data::AppSettingsDisplay {
        default_profile,
        last_profile,
        default_engine,
        default_iwad,
        default_editor,
        engine_search_folder,
        iwad_search_folder,
        map_search_folder,
        editor_search_folder,
        menu_mode: app_settings.menu_mode.to_string(),
        use_doomworld_api: app_settings.use_doomworld_api,
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
            map_id, map_id2, map_id3, map_id4, map_id5, additional_arguments,
            date_created, date_edited, date_last_run, save_game, run_count)
            VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?)",
        )
        .bind(&profile.name)
        .bind(profile.engine_id)
        .bind(profile.iwad_id)
        .bind(profile.map_id)
        .bind(profile.map_id2)
        .bind(profile.map_id3)
        .bind(profile.map_id4)
        .bind(profile.map_id5)
        .bind(&profile.additional_arguments)
        .bind(profile.date_created)
        .bind(profile.date_edited)
        .bind(profile.date_last_run)
        .bind(&profile.save_game)
        .bind(profile.run_count)
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
            "UPDATE profiles SET name = $2, engine_id = $3, iwad_id = $4,
            map_id = $5, map_id2 = $6, map_id3 = $7, map_id4 = $8, map_id5 = $9,
            additional_arguments = $10, date_created = $11, date_edited = $12,
            date_last_run = $13, save_game = $14, run_count = $15 WHERE id=$1",
        )
        .bind(profile.id)
        .bind(&profile.name)
        .bind(profile.engine_id)
        .bind(profile.iwad_id)
        .bind(profile.map_id)
        .bind(profile.map_id2)
        .bind(profile.map_id3)
        .bind(profile.map_id4)
        .bind(profile.map_id5)
        .bind(profile.additional_arguments)
        .bind(profile.date_created)
        .bind(profile.date_edited)
        .bind(profile.date_last_run)
        .bind(&profile.save_game)
        .bind(profile.run_count)
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

pub fn update_profile_date_last_run_and_run_count(
    id: i32,
    run_count: i32,
) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query("UPDATE profiles SET date_last_run = $2, run_count = $3 WHERE id = $1")
            .bind(id)
            .bind(Utc::now())
            .bind(run_count)
            .execute(&db)
            .await
            .wrap_err(format!(
                "Failed to update last run for profile with id '{}'",
                id
            ))
    })
}

pub fn update_profile_save_game(
    id: i32,
    save_game: Option<String>,
) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query("UPDATE profiles SET save_game = $2 WHERE id=$1")
            .bind(id)
            .bind(save_game)
            .execute(&db)
            .await
            .wrap_err(format!(
                "Failed to update save game for profile with id '{}'",
                id
            ))
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

pub fn get_profiles_ordered_by_date_last_run() -> Result<Vec<data::Profile>, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query_as::<_, data::Profile>("SELECT * FROM profiles ORDER BY date_last_run DESC")
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

pub fn get_profile_by_name(name: &str) -> Result<data::Profile, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query_as::<_, data::Profile>("SELECT * FROM profiles WHERE name = $1 COLLATE NOCASE")
            .bind(name.to_lowercase())
            .fetch_one(&db)
            .await
            .wrap_err(format!("Failed to get profile with name '{}'", name))
    })
}

fn get_profile_display(
    profile: data::Profile,
    engine: data::Engine,
    iwad: data::Iwad,
    maps: Vec<data::Map>,
) -> data::ProfileDisplay {
    data::ProfileDisplay {
        id: profile.id,
        name: profile.name,
        engine_id: profile.engine_id.unwrap_or(0),
        engine_app_name: engine.app_name.clone(),
        engine_path: paths::extract_path(&engine.path),
        engine_file: paths::extract_file_name(&engine.path),
        engine_version: engine.version.clone(),
        iwad_id: profile.iwad_id.unwrap_or(0),
        iwad_path: paths::extract_path(&iwad.path),
        iwad_file: paths::extract_file_name(&iwad.path),
        map_ids: (
            profile.map_id.unwrap_or(0),
            profile.map_id2.unwrap_or(0),
            profile.map_id3.unwrap_or(0),
            profile.map_id4.unwrap_or(0),
            profile.map_id5.unwrap_or(0),
        ),
        map_paths: (
            paths::extract_path(&maps[0].path),
            paths::extract_path(&maps[1].path),
            paths::extract_path(&maps[2].path),
            paths::extract_path(&maps[3].path),
            paths::extract_path(&maps[4].path),
        ),
        map_files: (
            paths::extract_file_name(&maps[0].path),
            paths::extract_file_name(&maps[1].path),
            paths::extract_file_name(&maps[2].path),
            paths::extract_file_name(&maps[3].path),
            paths::extract_file_name(&maps[4].path),
        ),
        additional_arguments: profile.additional_arguments.unwrap_or_default(),
        date_created: profile.date_created,
        date_edited: profile.date_edited,
        date_last_run: profile.date_last_run,
        save_game: profile.save_game.unwrap_or_default(),
        run_count: profile.run_count,
    }
}

pub fn get_profile_display_list(
    profile_order: data::ProfileOrder,
) -> Result<Vec<data::ProfileDisplay>, eyre::Report> {
    let profiles = match profile_order {
        data::ProfileOrder::Name => get_profiles()?,
        data::ProfileOrder::DateLastRun => get_profiles_ordered_by_date_last_run()?,
    };
    let engines = get_engines()?;
    let iwads = get_iwads()?;
    let maps = get_maps()?;

    let default_engine = data::Engine::default();
    let default_iwad = data::Iwad::default();
    let default_map = data::Map::default();

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
        let map = maps
            .iter()
            .find(|p| p.id == profile.map_id.unwrap_or(0))
            .unwrap_or(&default_map);
        let map2 = maps
            .iter()
            .find(|p| p.id == profile.map_id2.unwrap_or(0))
            .unwrap_or(&default_map);
        let map3 = maps
            .iter()
            .find(|p| p.id == profile.map_id3.unwrap_or(0))
            .unwrap_or(&default_map);
        let map4 = maps
            .iter()
            .find(|p| p.id == profile.map_id4.unwrap_or(0))
            .unwrap_or(&default_map);
        let map5 = maps
            .iter()
            .find(|p| p.id == profile.map_id5.unwrap_or(0))
            .unwrap_or(&default_map);

        profile_list.push(get_profile_display(
            profile.clone(),
            engine.clone(),
            iwad.clone(),
            vec![
                map.clone(),
                map2.clone(),
                map3.clone(),
                map4.clone(),
                map5.clone(),
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
    let map = match profile.map_id {
        Some(id) => get_map_by_id(id)?,
        None => data::Map::default(),
    };
    let map2 = match profile.map_id2 {
        Some(id) => get_map_by_id(id)?,
        None => data::Map::default(),
    };
    let map3 = match profile.map_id3 {
        Some(id) => get_map_by_id(id)?,
        None => data::Map::default(),
    };
    let map4 = match profile.map_id4 {
        Some(id) => get_map_by_id(id)?,
        None => data::Map::default(),
    };
    let map5 = match profile.map_id5 {
        Some(id) => get_map_by_id(id)?,
        None => data::Map::default(),
    };

    Ok(get_profile_display(
        profile,
        engine,
        iwad,
        vec![map, map2, map3, map4, map5],
    ))
}

pub fn add_queue(queue: data::Queue) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query("INSERT INTO profile_queues (name, date_created, date_edited) VALUES (?,?,?)")
            .bind(&queue.name)
            .bind(queue.date_created)
            .bind(queue.date_edited)
            .execute(&db)
            .await
            .wrap_err(format!("Failed to add queue '{:?}", queue))
    })
}

pub fn update_queue(queue: data::Queue) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query("UPDATE profile_queues SET name = $2, date_edited = $3 WHERE id=$1")
            .bind(queue.id)
            .bind(&queue.name)
            .bind(queue.date_edited)
            .execute(&db)
            .await
            .wrap_err(format!(
                "Failed to update queue '{}', id '{}'",
                queue.name, queue.id
            ))
    })
}

pub fn delete_queue(queue_id: i32) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query("DELETE FROM profile_queues WHERE id=$1")
            .bind(queue_id)
            .execute(&db)
            .await
            .wrap_err(format!("Failed to delete queue with id '{}'", queue_id))
    })
}

fn get_queues() -> Result<Vec<data::Queue>, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;
        sqlx::query_as::<_, data::Queue>("SELECT * FROM profile_queues ORDER BY name")
            .fetch_all(&db)
            .await
            .wrap_err("Failed to get list of all queues")
    })
}

pub fn get_queue_by_name(name: &str) -> Result<data::Queue, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;
        sqlx::query_as::<_, data::Queue>(
            "SELECT * FROM profile_queues WHERE name = $1 COLLATE NOCASE",
        )
        .bind(name.to_lowercase())
        .fetch_one(&db)
        .await
        .wrap_err(format!("Failed to get queue with name '{}'", name))
    })
}

pub fn add_queue_item(
    queue_item: data::QueueItem,
) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query("INSERT INTO profile_queue_items (profile_queue_id, profile_id, order_index) VALUES (?,?,?)")
            .bind(queue_item.profile_queue_id)
            .bind(queue_item.profile_id)
            .bind(queue_item.order_index)
            .execute(&db)
            .await
            .wrap_err(format!("Failed to add queue item '{:?}", queue_item))
    })
}

pub fn update_queue_item_order_index(
    id: i32,
    order_index: i32,
) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;
        sqlx::query("UPDATE profile_queue_items SET order_index = $2 WHERE id=$1")
            .bind(id)
            .bind(order_index)
            .execute(&db)
            .await
            .wrap_err(format!(
                "Failed to update queue item order index '{}', id '{}'",
                order_index, id
            ))
    })
}

pub fn delete_queue_item(
    queue_item: &data::QueueItem,
) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    // We want to delete this item but make sure the ordering is fixed for all the other items after it
    let queue_items = get_queue_items(queue_item.profile_queue_id)?;

    // Fix the ordering on the remaining items. Only do if more than one item in the queue
    if queue_items.len() > 1 {
        let mut order_index = queue_item.order_index;
        for item in queue_items {
            if item.order_index > order_index {
                update_queue_item_order_index(item.id, order_index)?;
                order_index += 1;
            }
        }
    }

    // Now delete the queue item
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;
        sqlx::query("DELETE FROM profile_queue_items WHERE id=$1")
            .bind(queue_item.id)
            .execute(&db)
            .await
            .wrap_err(format!(
                "Failed to delete queue item with id '{}'",
                queue_item.id
            ))
    })
}

pub fn delete_all_queue_items(
    queue_id: i32,
) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query("DELETE FROM profile_queue_items WHERE profile_queue_id=$1")
            .bind(queue_id)
            .execute(&db)
            .await
            .wrap_err(format!(
                "Failed to delete queue items for queue with id '{}'",
                queue_id
            ))
    })
}

pub fn delete_profile_from_queues(profile_id: i32) -> Result<String, eyre::Report> {
    // FIXME: This is not very efficient.

    // Get a list of all the queue items containing this profile
    let queue_items = get_queue_items_containing_profile(profile_id)?;

    // For each queue item, we need to remove it from the queue and also make
    // sure the remaining queue is ordered correctly
    for queue_item in &queue_items {
        delete_queue_item(queue_item)?;
    }

    Ok(format!(
        "Deleted profile from {} queue(s)",
        queue_items.len()
    ))
}

pub fn get_queue_items(queue_id: i32) -> Result<Vec<data::QueueItem>, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;
        sqlx::query_as::<_, data::QueueItem>(
            "SELECT * FROM profile_queue_items WHERE profile_queue_id = ? ORDER BY order_index",
        )
        .bind(queue_id)
        .fetch_all(&db)
        .await
        .wrap_err(format!(
            "Failed to get list of all queue items for queue_id '{}'",
            queue_id
        ))
    })
}

fn get_queue_items_containing_profile(
    profile_id: i32,
) -> Result<Vec<data::QueueItem>, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;
        sqlx::query_as::<_, data::QueueItem>(
            "SELECT * FROM profile_queue_items WHERE profile_id = ?",
        )
        .bind(profile_id)
        .fetch_all(&db)
        .await
        .wrap_err(format!(
            "Failed to get list of all queue items containing profile_id '{}'",
            profile_id
        ))
    })
}

pub fn get_queue_display_list() -> Result<Vec<data::QueueDisplay>, eyre::Report> {
    let queues = get_queues()?;
    let mut queue_list: Vec<data::QueueDisplay> = Vec::new();

    for queue in queues {
        let queue_items = get_queue_items(queue.id)?;
        let mut profile_list: Vec<data::ProfileDisplay> = Vec::new();
        for queue_item in queue_items {
            let profile = get_profile_display_by_id(queue_item.profile_id)?;
            profile_list.push(profile);
        }

        queue_list.push(data::QueueDisplay {
            id: queue.id,
            name: queue.name,
            profiles: profile_list,
            date_created: queue.date_created,
        });
    }

    Ok(queue_list)
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
            "INSERT INTO play_settings (comp_level, config_file, fast_monsters, no_monsters,
            respawn_monsters, warp, skill, turbo, timer, width, height, full_screen,
            windowed, additional_arguments) VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?)",
        )
        .bind(&play_settings.comp_level)
        .bind(&play_settings.config_file)
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
            "UPDATE play_settings SET comp_level = $2, config_file = $3,
                fast_monsters = $4, no_monsters = $5, respawn_monsters = $6,
                warp = $7, skill = $8, turbo = $9, timer = $10, width = $11,
                height = $12, full_screen = $13, windowed = $14,
                additional_arguments = $15
                WHERE id=$1",
        )
        .bind(play_settings.id)
        .bind(&play_settings.comp_level)
        .bind(&play_settings.config_file)
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

pub fn add_editor(editor: &data::Editor) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query("INSERT INTO editors (app_name, path, version, load_file_argument, additional_arguments) VALUES (?,?,?,?,?)")
            .bind(&editor.app_name)
            .bind(&editor.path)
            .bind(&editor.version)
            .bind(&editor.load_file_argument)
            .bind(&editor.additional_arguments)
            .execute(&db)
            .await
            .wrap_err(format!("Failed to add editor '{:?}", editor))
    })
}

pub fn delete_editor(id: i32) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query("DELETE FROM editors WHERE id=$1")
            .bind(id)
            .execute(&db)
            .await
            .wrap_err(format!("Failed to delete editor with id '{}'", id))
    })
}

pub fn update_editor_version(
    id: i32,
    version: &str,
) -> Result<sqlx::sqlite::SqliteQueryResult, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query("UPDATE editors SET version = $1 WHERE id=$2")
            .bind(version)
            .bind(id)
            .execute(&db)
            .await
            .wrap_err(format!(
                "Failed to update version '{}' for editor with id '{}'",
                version, id
            ))
    })
}

pub fn get_editors() -> Result<Vec<data::Editor>, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query_as::<_, data::Editor>("SELECT * FROM editors ORDER BY app_name")
            .fetch_all(&db)
            .await
            .wrap_err("Failed to get list of all Editors")
    })
}

pub fn get_editor_by_id(id: i32) -> Result<data::Editor, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query_as::<_, data::Editor>("SELECT * FROM editors WHERE id = ?")
            .bind(id)
            .fetch_one(&db)
            .await
            .wrap_err(format!("Failed to get Editor with id '{}'", id))
    })
}

pub fn get_editor_by_path(path: &str) -> Result<data::Editor, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        sqlx::query_as::<_, data::Editor>("SELECT * FROM editors WHERE path = $1 COLLATE NOCASE")
            .bind(path.to_lowercase())
            .fetch_one(&db)
            .await
            .wrap_err(format!("Failed to get Editor with path '{}'", path))
    })
}

pub fn get_editor_count() -> Result<i64, eyre::Report> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = get_db().await;

        let result: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM editors")
            .fetch_one(&db)
            .await
            .wrap_err("Failed to get Editor count")?;

        Ok(result.0)
    })
}
