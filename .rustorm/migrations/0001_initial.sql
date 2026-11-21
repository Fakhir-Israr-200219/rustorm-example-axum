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
    "email" TEXT NOT NULL UNIQUE,
    "place" TEXT,
    "age" INTEGER NOT NULL,
    "user_bio" TEXT,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "user_profiles" (
        "id" SERIAL NOT NULL,
    "user_id" INTEGER NOT NULL UNIQUE,
    "avatar" TEXT,
    "website" TEXT,
    PRIMARY KEY ("id"),
    FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "posts" (
        "id" SERIAL NOT NULL,
    "title" TEXT NOT NULL,
    "content" TEXT,
    "published" BOOLEAN NOT NULL DEFAULT false,
    "view_count" INTEGER NOT NULL DEFAULT 0,
    "user_id" INTEGER NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("id"),
    FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "comments" (
        "id" SERIAL NOT NULL,
    "body" TEXT NOT NULL,
    "post_id" INTEGER NOT NULL,
    "author_id" INTEGER NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("id"),
    FOREIGN KEY ("post_id") REFERENCES "posts" ("id") ON DELETE CASCADE,
    FOREIGN KEY ("author_id") REFERENCES "users" ("id")
);

CREATE TABLE IF NOT EXISTS "tags" (
        "id" SERIAL NOT NULL,
    "name" TEXT NOT NULL UNIQUE,
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "roles" (
        "id" SERIAL NOT NULL,
    "name" TEXT NOT NULL UNIQUE,
    PRIMARY KEY ("id")
);

CREATE INDEX "users_email_idx" ON "users" ("email");
CREATE INDEX "users_name_age_idx" ON "users" ("name", "age");
CREATE INDEX "posts_user_id_idx" ON "posts" ("user_id");
CREATE INDEX "comments_post_id_idx" ON "comments" ("post_id");
