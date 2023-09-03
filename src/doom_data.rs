use strum_macros::Display;

#[derive(Clone, Debug, PartialEq, sqlx::Type, Display)]
pub enum GameEngineType {
    Doom,
    GzDoom,
    PrBoomPlus,
}

#[derive(Clone, Debug)]
pub struct GameEngine {
    pub exe_name: String,
    pub game_engine_type: GameEngineType,
}

pub fn get_engine_list() -> Vec<GameEngine> {
    // https://github.com/coelckers/prboom-plus/blob/master/prboom2/doc/README.command-line
    // https://zdoom.org/wiki/Command_line_parameters
    // https://doomwiki.org/wiki/Comparison_of_source_ports

    // vec![
    //     "doom.exe",
    //     "gzdoom.exe",
    //     "dsda-doom.exe",
    //     "prboom-plus.exe",
    //     "glboom-plus.exe",
    // ];

    vec![
        GameEngine {
            exe_name: "doom.exe".to_string(),
            game_engine_type: GameEngineType::Doom,
        },
        GameEngine {
            exe_name: "gzdoom.exe".to_string(),
            game_engine_type: GameEngineType::GzDoom,
        },
        GameEngine {
            exe_name: "dsda-doom.exe".to_string(),
            game_engine_type: GameEngineType::PrBoomPlus,
        },
        // GameEngine {
        //     exe_name: "prboom-plus.exe".to_string(),
        //     game_engine_type: GameEngineType::PrBoom,
        // },
        // GameEngine {
        //     exe_name: "glboom-plus.exe".to_string(),
        //     game_engine_type: GameEngineType::PrBoom,
        // },
    ]
}


#[derive(Clone, Debug, PartialEq, sqlx::Type, Display)]
pub enum InternalWadType {
    Doom,
    DoomShareware,
    Doom2,
    Tnt,
    Plutonia,
    Heretic,
    HereticShareware,
    Hexen,
    HexenDeathkings,
    StrifeTeaser,
    Strife,
    StrifeVoices,
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
            name: "Doom Shareware".to_string(),
            internal_wad_type: InternalWadType::DoomShareware,
        },
        InternalWad {
            file_name: "DOOM.WAD".to_string(),
            name: "Doom".to_string(),
            internal_wad_type: InternalWadType::Doom,
        },
        InternalWad {
            file_name: "DOOM2.WAD".to_string(),
            name: "Doom II".to_string(),
            internal_wad_type: InternalWadType::Doom2,
        },
        InternalWad {
            file_name: "TNT.WAD".to_string(),
            name: "Final Doom - TNT: Evilution".to_string(),
            internal_wad_type: InternalWadType::Tnt,
        },
        InternalWad {
            file_name: "PLUTONIA.WAD".to_string(),
            name: "Final Doom - The Plutonia Experiment".to_string(),
            internal_wad_type: InternalWadType::Plutonia,
        },
        InternalWad {
            file_name: "HERETIC1.WAD".to_string(),
            name: "Heretic Shareware".to_string(),
            internal_wad_type: InternalWadType::HereticShareware,
        },
        InternalWad {
            file_name: "HERETIC.WAD".to_string(),
            name: "Heretic".to_string(),
            internal_wad_type: InternalWadType::Heretic,
        },
        InternalWad {
            file_name: "HEXEN.WAD".to_string(),
            name: "Hexen Demo or Full".to_string(),
            internal_wad_type: InternalWadType::Hexen,
        },
        InternalWad {
            file_name: "HEXDD.WAD".to_string(),
            name: "Hexen: Deathkings of the Dark Citadel".to_string(),
            internal_wad_type: InternalWadType::HexenDeathkings,
        },
        InternalWad {
            file_name: "STRIFE0.WAD".to_string(),
            name: "Strife Teaser".to_string(),
            internal_wad_type: InternalWadType::StrifeTeaser,
        },
        InternalWad {
            file_name: "STRIFE1.WAD".to_string(),
            name: "Strife".to_string(),
            internal_wad_type: InternalWadType::Strife,
        },
        InternalWad {
            file_name: "VOICES.WAD".to_string(),
            name: "Strife Voices".to_string(),
            internal_wad_type: InternalWadType::StrifeVoices,
        }
    ]
}