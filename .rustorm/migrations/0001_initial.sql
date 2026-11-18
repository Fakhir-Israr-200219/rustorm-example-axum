CREATE TABLE "users" (
        "id" SERIAL NOT NULL,
    "name" TEXT NOT NULL,
    "place" TEXT,
    "email" TEXT NOT NULL UNIQUE,
    PRIMARY KEY ("id")
);

CREATE TABLE "posts" (
        "id" SERIAL NOT NULL,
    "title" TEXT NOT NULL,
    "content" TEXT,
    "user_id" INTEGER NOT NULL,
    PRIMARY KEY ("id"),
    FOREIGN KEY ("user_id") REFERENCES "users" ("id")
);

