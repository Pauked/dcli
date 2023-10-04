use std::env;

use strum_macros::Display;

use crate::constants;

pub const EXT_WAD: &str = "wad";
pub const EXT_PK3: &str = "pk3";
pub const EXT_PKE: &str = "pke";
pub const EXT_TXT: &str = "txt";

pub const GAME_FILES: [&str; 3] = [EXT_WAD, EXT_PK3, EXT_PKE];

pub const IWAD_IDENTIFIER: [u8; 4] = *b"IWAD";
pub const PWAD_IDENTIFIER: [u8; 4] = *b"PWAD";

#[derive(Clone, Debug, PartialEq, sqlx::Type, Display)]
pub enum GameEngineType {
    #[strum(serialize = "GZDoom")]
    GzDoom,
    #[strum(serialize = "PrBoom+")]
    PrBoomPlus,
    #[strum(serialize = "Crispy Doom")]
    CrispyDoom,
    #[strum(serialize = "Eternity Engine")]
    EternityEngine,
    #[strum(serialize = "Doom Retro")]
    DoomRetro,
    #[strum(serialize = "Woof!")]
    Woof,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, sqlx::Type, Display)]
pub enum OperatingSystem {
    Windows,
    Linux,
    MacOs,
}

pub fn get_operating_system() -> OperatingSystem {
    match env::consts::OS {
        constants::OS_WINDOWS => OperatingSystem::Windows,
        constants::OS_LINUX => OperatingSystem::Linux,
        constants::OS_MACOS => OperatingSystem::MacOs,
        _ => panic!("Unsupported OS: {}", env::consts::OS),
    }
}

#[derive(Clone, Debug)]
pub struct GameEngine {
    pub exe_name: String,
    pub internal_path: Option<String>,
    pub game_engine_type: GameEngineType,
    pub operating_system: OperatingSystem,
}

pub fn get_engine_list(operating_system: OperatingSystem) -> Vec<GameEngine> {
    let result = vec![
        GameEngine {
            exe_name: "gzdoom.exe".to_string(),
            internal_path: None,
            game_engine_type: GameEngineType::GzDoom,
            operating_system: OperatingSystem::Windows,
        },
        GameEngine {
            exe_name: "dsda-doom.exe".to_string(),
            internal_path: None,
            game_engine_type: GameEngineType::PrBoomPlus,
            operating_system: OperatingSystem::Windows,
        },
        GameEngine {
            exe_name: "crispy-doom.exe".to_string(),
            internal_path: None,
            game_engine_type: GameEngineType::CrispyDoom,
            operating_system: OperatingSystem::Windows,
        },
        GameEngine {
            exe_name: "eternity.exe".to_string(),
            internal_path: None,
            game_engine_type: GameEngineType::EternityEngine,
            operating_system: OperatingSystem::Windows,
        },
        GameEngine {
            exe_name: "doomretro.exe".to_string(),
            internal_path: None,
            game_engine_type: GameEngineType::DoomRetro,
            operating_system: OperatingSystem::Windows,
        },
        GameEngine {
            exe_name: "woof.exe".to_string(),
            internal_path: None,
            game_engine_type: GameEngineType::Woof,
            operating_system: OperatingSystem::Windows,
        },
        GameEngine {
            exe_name: "gzdoom.app".to_string(),
            internal_path: Some("Contents/MacOS/gzdoom".to_string()),
            game_engine_type: GameEngineType::GzDoom,
            operating_system: OperatingSystem::MacOs,
        },
    ];

    result
        .into_iter()
        .filter(|engine| engine.operating_system == operating_system)
        .collect()
}

#[derive(Clone, Debug, PartialEq, sqlx::Type, Display)]
pub enum InternalWadType {
    #[strum(serialize = "Doom")]
    Doom,
    #[strum(serialize = "Doom Shareware")]
    DoomShareware,
    #[strum(serialize = "Doom II")]
    Doom2,
    #[strum(serialize = "TNT: Evilution (Final Doom)")]
    Tnt,
    #[strum(serialize = "The Plutonia Experiment (Final Doom)")]
    Plutonia,
    #[strum(serialize = "Heretic")]
    Heretic,
    #[strum(serialize = "Heretic Shareware")]
    HereticShareware,
    #[strum(serialize = "Hexen")]
    Hexen,
    #[strum(serialize = "Hexen: Deathkings of the Dark Citadel")]
    HexenDeathkings,
    #[strum(serialize = "Strife Teaser")]
    StrifeTeaser,
    #[strum(serialize = "Strife")]
    Strife,
    #[strum(serialize = "Strife Voices")]
    StrifeVoices,
    Unknown,
}

#[derive(Clone, Debug)]
pub struct InternalWad {
    pub file_name: String,
    pub name: String,
    pub internal_wad_type: InternalWadType,
}

pub fn get_internal_wad_list() -> Vec<InternalWad> {
    // Via: https://doom.fandom.com/wiki/IWAD
    vec![
        InternalWad {
            file_name: "DOOM1.WAD".to_string(),
            name: InternalWadType::DoomShareware.to_string(),
            internal_wad_type: InternalWadType::DoomShareware,
        },
        InternalWad {
            file_name: "DOOM.WAD".to_string(),
            name: InternalWadType::Doom.to_string(),
            internal_wad_type: InternalWadType::Doom,
        },
        InternalWad {
            file_name: "DOOM2.WAD".to_string(),
            name: InternalWadType::Doom2.to_string(),
            internal_wad_type: InternalWadType::Doom2,
        },
        InternalWad {
            file_name: "TNT.WAD".to_string(),
            name: InternalWadType::Tnt.to_string(),
            internal_wad_type: InternalWadType::Tnt,
        },
        InternalWad {
            file_name: "PLUTONIA.WAD".to_string(),
            name: InternalWadType::Plutonia.to_string(),
            internal_wad_type: InternalWadType::Plutonia,
        },
        InternalWad {
            file_name: "HERETIC1.WAD".to_string(),
            name: InternalWadType::HereticShareware.to_string(),
            internal_wad_type: InternalWadType::HereticShareware,
        },
        InternalWad {
            file_name: "HERETIC.WAD".to_string(),
            name: InternalWadType::Heretic.to_string(),
            internal_wad_type: InternalWadType::Heretic,
        },
        InternalWad {
            file_name: "HEXEN.WAD".to_string(),
            name: InternalWadType::Hexen.to_string(),
            internal_wad_type: InternalWadType::Hexen,
        },
        InternalWad {
            file_name: "HEXDD.WAD".to_string(),
            name: InternalWadType::HexenDeathkings.to_string(),
            internal_wad_type: InternalWadType::HexenDeathkings,
        },
        InternalWad {
            file_name: "STRIFE0.WAD".to_string(),
            name: InternalWadType::StrifeTeaser.to_string(),
            internal_wad_type: InternalWadType::StrifeTeaser,
        },
        InternalWad {
            file_name: "STRIFE1.WAD".to_string(),
            name: InternalWadType::Strife.to_string(),
            internal_wad_type: InternalWadType::Strife,
        },
        InternalWad {
            file_name: "VOICES.WAD".to_string(),
            name: InternalWadType::StrifeVoices.to_string(),
            internal_wad_type: InternalWadType::StrifeVoices,
        },
    ]
}
