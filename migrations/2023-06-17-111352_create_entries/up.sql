CREATE TABLE entries (
    id INTEGER NOT NULL PRIMARY KEY,
    club VARCHAR NOT NULL,
    crew VARCHAR NOT NULL,
    year INTEGER NOT NULL,
    day INTEGER NOT NULL,
    position INTEGER NOT NULL,
    competition INTEGER CHECK (competition IN (0, 1, 2, 3, 4)) NOT NULL
)