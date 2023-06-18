CREATE TABLE entries (
    id INTEGER NOT NULL PRIMARY KEY,
    club VARCHAR NOT NULL,
    crew VARCHAR NOT NULL,
    year INTEGER NOT NULL,
    day INTEGER NOT NULL,
    position INTEGER NOT NULL CHECK (position >= 0),
    competition VARCHAR CHECK (competition IN ('early', 'mmays', 'wmays', 'mlents', 'wlents')) NOT NULL,
    UNIQUE(crew,year,day,competition)
)