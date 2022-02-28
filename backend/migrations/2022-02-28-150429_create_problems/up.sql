-- Your SQL goes here
CREATE TABLE "problems" (
    "id"    INTEGER NOT NULL UNIQUE,
    "title"    TEXT NOT NULL,
    "accepted_cnt"    INTEGER DEFAULT 0,
    "submit_cnt"    INTEGER DEFAULT 0,
    "description"    TEXT NOT NULL,
    "input_desc"    TEXT NOT NULL,
    "output_desc"    TEXT NOT NULL,
    "difficulty"    TEXT NOT NULL,
    "time_limit"    INTEGER DEFAULT 1,
    "memory_limit"    INTEGER DEFAULT 256,
    PRIMARY KEY("id" AUTOINCREMENT)
)