
CREATE TABLE IF NOT EXISTS engines (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    path TEXT NOT NULL,
    version TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS iwads (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    path TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS wads (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    path TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS profiles (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    engine_id INTEGER NULL,
    iwad_id INTEGER NULL,
    wad_id INTEGER NULL,
    FOREIGN KEY (engine_id) REFERENCES engines (id),
    FOREIGN KEY (iwad_id) REFERENCES iwads (id),
    FOREIGN KEY (wad_id) REFERENCES wads (id)
);

CREATE TABLE IF NOT EXISTS settings (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    value TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS search_paths (
    id INTEGER PRIMARY KEY NOT NULL,
    path TEXT NOT NULL,
    path_type TEXT NOT NULL
);
-- Map Editors???
