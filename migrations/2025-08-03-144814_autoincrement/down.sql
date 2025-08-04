-- This file should undo anything in `up.sql`
ALTER TABLE "shortened_urls" DROP COLUMN "id";
ALTER TABLE "shortened_urls" ADD COLUMN "id" INT4 NOT NULL;

ALTER TABLE "shortened_urls" ADD PRIMARY KEY (id);
