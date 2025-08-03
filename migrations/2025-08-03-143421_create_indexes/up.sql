-- Your SQL goes here

CREATE UNIQUE INDEX IF NOT EXISTS idx_short_code ON shortened_urls (short_code);
CREATE UNIQUE INDEX IF NOT EXISTS idx_original_url ON shortened_urls (original_url);
