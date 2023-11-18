CREATE TABLE IF NOT EXISTS profile_queues (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    date_created DATETIME NOT NULL,
    date_edited DATETIME NOT NULL
);

CREATE TABLE IF NOT EXISTS profile_queue_items (
    id INTEGER PRIMARY KEY NOT NULL,
    profile_queue_id INTEGER NOT NULL,
    profile_id INTEGER NOT NULL,
    order_index INTEGER NOT NULL,
    FOREIGN KEY (profile_queue_id) REFERENCES profile_queues (id),
    FOREIGN KEY (profile_id) REFERENCES profiles (id)
);