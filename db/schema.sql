CREATE TABLE IF NOT EXISTS articles (
    id          INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    text        TEXT NOT NULL,
    title       TEXT NOT NULL,
    created     NUMERIC NOT NULL,
    updated     NUMERIC NOT NULL
);
