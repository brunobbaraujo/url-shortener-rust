// @generated automatically by Diesel CLI.

diesel::table! {
    shortened_urls (id) {
        id -> Int4,
        original_url -> Varchar,
        #[max_length = 10]
        short_code -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        click_count -> Int4,
    }
}
