CREATE TABLE IF NOT EXISTS engines (
    id INTEGER PRIMARY KEY NOT NULL,
    app_name TEXT NOT NULL,
    path TEXT NOT NULL UNIQUE,
    version TEXT NOT NULL,
    game_engine_type TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS iwads (
    id INTEGER PRIMARY KEY NOT NULL,
    path TEXT NOT NULL UNIQUE,
    internal_wad_type TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS maps (
    id INTEGER PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    author TEXT NOT NULL,
    path TEXT NOT NULL UNIQUE
);
CREATE TABLE IF NOT EXISTS editors (
    id INTEGER PRIMARY KEY NOT NULL,
    app_name TEXT NOT NULL,
    path TEXT NOT NULL UNIQUE,
    load_file_argument TEXT NULL,
    additional_arguments TEXT NULL,
    version TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS profiles (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    engine_id INTEGER NOT NULL,
    iwad_id INTEGER NOT NULL,
    map_id INTEGER NULL,
    map_id2 INTEGER NULL,
    map_id3 INTEGER NULL,
    map_id4 INTEGER NULL,
    map_id5 INTEGER NULL,
    additional_arguments TEXT NULL,
    date_created DATETIME NOT NULL,
    date_edited DATETIME NOT NULL,
    date_last_run DATETIME NULL,
    FOREIGN KEY (engine_id) REFERENCES engines (id),
    FOREIGN KEY (iwad_id) REFERENCES maps (id),
    FOREIGN KEY (map_id) REFERENCES maps (id),
    FOREIGN KEY (map_id2) REFERENCES maps (id),
    FOREIGN KEY (map_id3) REFERENCES maps (id),
    FOREIGN KEY (map_id5) REFERENCES maps (id)
);
CREATE TABLE IF NOT EXISTS app_settings (
    id INTEGER PRIMARY KEY NOT NULL,
    default_profile_id INTEGER NULL,
    last_profile_id INTEGER NULL,
    default_engine_id INTEGER NULL,
    default_iwad_id INTEGER NULL,
    default_editor_id INTEGER NULL,
    engine_search_folder TEXT NULL,
    iwad_search_folder TEXT NULL,
    map_search_folder TEXT NULL,
    editor_search_folder TEXT NULL,
    menu_mode TEXT NOT NULL,
    FOREIGN KEY (default_profile_id) REFERENCES profiles (id),
    FOREIGN KEY (last_profile_id) REFERENCES profiles (id),
    FOREIGN KEY (default_editor_id) REFERENCES editors (id),
    FOREIGN KEY (default_engine_id) REFERENCES engines (id),
    FOREIGN KEY (default_iwad_id) REFERENCES iwads (id)
);
CREATE TABLE IF NOT EXISTS play_settings (
    id INTEGER PRIMARY KEY NOT NULL,
    comp_level TEXT NULL,
    config_file TEXT NULL,
    fast_monsters BOOLEAN NOT NULL,
    no_monsters BOOLEAN NOT NULL,
    respawn_monsters BOOLEAN NOT NULL,
    warp TEXT NULL,
    skill INTEGER NULL,
    turbo INTEGER NULL,
    timer INTEGER NULL,
    width INTEGER NULL,
    height INTEGER NULL,
    full_screen BOOLEAN NOT NULL,
    windowed BOOLEAN NOT NULL,
    additional_arguments TEXT NULL
);
CREATE TABLE IF NOT EXISTS track_menu (
    id INTEGER PRIMARY KEY NOT NULL,
    option_name TEXT NOT NULL UNIQUE,
    usage INTEGER NOT NULL
);