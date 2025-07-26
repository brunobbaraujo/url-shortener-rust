-- Your SQL goes here
ALTER TABLE "shortened_urls" DROP COLUMN "created_at";
ALTER TABLE "shortened_urls" DROP COLUMN "updated_at";
ALTER TABLE "shortened_urls" ADD COLUMN "created_at" TIMESTAMPTZ NOT NULL;
ALTER TABLE "shortened_urls" ADD COLUMN "updated_at" TIMESTAMPTZ NOT NULL;

