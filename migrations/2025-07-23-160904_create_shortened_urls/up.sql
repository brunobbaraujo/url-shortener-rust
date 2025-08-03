-- Your SQL goes here
CREATE TABLE "shortened_urls"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"original_url" VARCHAR NOT NULL,
	"short_code" VARCHAR(10) NOT NULL,
	"created_at" TIMESTAMPTZ,
	"updated_at" TIMESTAMPTZ,
	"click_count" INT4,
	"last_accessed" TIMESTAMPTZ
);
