-- Your SQL goes here
ALTER TABLE "shortened_urls" DROP COLUMN "id";
ALTER TABLE "shortened_urls" ADD COLUMN "id" serial;

ALTER TABLE "shortened_urls" ADD PRIMARY KEY (id);
