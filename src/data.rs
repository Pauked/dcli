use sqlx::FromRow;
use tabled::Tabled;

use crate::doom_data;

#[derive(Clone, Debug, FromRow, Tabled)]
pub struct Engine {
    #[tabled(skip)]
    pub id: i32,
    pub path: String,
    pub version: String,
    #[tabled(rename = "Engine Type")]
    pub game_engine_type: doom_data::GameEngineType,
}

#[derive(Clone, Debug, FromRow, Tabled)]
pub struct Iwad {
    #[tabled(skip)]
    pub id: i32,
    pub path: String,
    #[tabled(rename = "Internal WAD Type")]
    pub internal_wad_type: doom_data::InternalWadType,
}