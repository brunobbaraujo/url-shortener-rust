-- This file should undo anything in `up.sql`
ALTER TABLE "shortened_urls" DROP COLUMN "click_count";
ALTER TABLE "shortened_urls" ADD COLUMN "click_count" INT4;

