-- Your SQL goes here
CREATE TABLE "sessions" (
    hash_id TEXT PRIMARY KEY NOT NULL,
    created_ts INTEGER NOT NULL,
    updated_ts INTEGER NOT NULL
);

INSERT INTO "sessions"
    (hash_id, created_ts, updated_ts)
VALUES
    ("abc234", 1663555768, 1663555768),
    ("cdf678", 1663555778, 1663555778),
    ("ahyd21", 1663535768, 1663535768),
    ("_325fg", 1663525768, 1663525768);
