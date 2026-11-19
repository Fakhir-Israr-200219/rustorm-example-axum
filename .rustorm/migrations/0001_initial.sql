-- RustORM Migration Tracking
CREATE TABLE IF NOT EXISTS _rustorm_migrations (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    checksum TEXT NOT NULL,
    applied_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS "users" (
        "id" SERIAL NOT NULL,
    "name" TEXT NOT NULL,
    "place" TEXT,
    "hujra" TEXT,
    "lips" TEXT,
    "email" TEXT NOT NULL UNIQUE,
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "posts" (
        "id" SERIAL NOT NULL,
    "title" TEXT NOT NULL,
    "content" TEXT,
    "user_id" INTEGER NOT NULL,
    PRIMARY KEY ("id"),
    FOREIGN KEY ("user_id") REFERENCES "users" ("id")
);

