CREATE TABLE IF NOT EXISTS "post_tags" (
    "tag_id" INTEGER NOT NULL,
    "post_id" INTEGER NOT NULL,
    PRIMARY KEY ("tag_id", "post_id"),
    FOREIGN KEY ("tag_id") REFERENCES "tags" ("id") ON DELETE CASCADE,
    FOREIGN KEY ("post_id") REFERENCES "posts" ("id") ON DELETE CASCADE
);

CREATE INDEX "post_tags_tag_id_idx" ON "post_tags" ("tag_id");
CREATE INDEX "post_tags_post_id_idx" ON "post_tags" ("post_id");