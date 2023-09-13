CREATE TABLE IF NOT EXISTS game_settings (
    id INTEGER PRIMARY KEY NOT NULL,
    comp_level TEXT NULL,
    fast_monsters INTEGER NULL,
    no_monsters INTEGER NULL,
    respawn_monsters INTEGER NULL,
    map TEXT NULL,
    skill TEXT NULL,
    turbo INTEGER NULL,
    timer INTEGER NULL,
    resolution TEXT NULL,
    full_screen INTEGER NULL
);