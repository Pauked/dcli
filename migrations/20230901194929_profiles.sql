
CREATE TABLE IF NOT EXISTS engines (
    id INTEGER PRIMARY KEY NOT NULL,
    path TEXT NOT NULL,
    version TEXT NOT NULL,
    game_engine_type TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS iwads (
    id INTEGER PRIMARY KEY NOT NULL,
    path TEXT NOT NULL,
    internal_wad_type TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS pwads (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    path TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS profiles (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    engine_id INTEGER NULL,
    iwad_id INTEGER NULL,
    pwad_id INTEGER NULL,
    FOREIGN KEY (engine_id) REFERENCES engines (id),
    FOREIGN KEY (iwad_id) REFERENCES iwads (id),
    FOREIGN KEY (pwad_id) REFERENCES pwads (id)
);

CREATE TABLE IF NOT EXISTS settings (
    id INTEGER PRIMARY KEY NOT NULL,
    active_profile_id INTEGER NULL,
    exe_search_folder TEXT NULL,
    iwad_search_folder TEXT NULL,
    pwad_search_folder TEXT NULL,
    FOREIGN KEY (active_profile_id) REFERENCES profiles (id)
);

-- CREATE TABLE IF NOT EXISTS search_paths (
--     id INTEGER PRIMARY KEY NOT NULL,
--     path TEXT NOT NULL,
--     path_type TEXT NOT NULL
-- );
-- Map Editors???
