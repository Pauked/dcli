use color_eyre::{Report, Result};

use crate::db;

pub async fn create_db() -> Result<bool, Report> {
    db::create_db().await
}