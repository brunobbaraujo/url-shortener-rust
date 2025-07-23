-- This file should undo anything in `up.sql`
ALTER TABLE "shortened_urls" ADD COLUMN "last_accessed" TIMESTAMPTZ;

