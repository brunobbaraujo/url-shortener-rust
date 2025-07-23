-- Your SQL goes here
ALTER TABLE "shortened_urls" DROP COLUMN "click_count";
ALTER TABLE "shortened_urls" ADD COLUMN "click_count" INT4 NOT NULL;

