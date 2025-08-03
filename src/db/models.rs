use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::db::pool;
use crate::schema::shortened_urls;
// ordinary diesel model setup

#[derive(Queryable, QueryableByName, Selectable, Insertable, Debug)]
#[diesel(table_name = crate::schema::shortened_urls)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ShortenedUrl {
    pub id: i32,
    pub original_url: String,
    pub short_code: String,
    pub click_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn get_original_url_by_codes(codes: Vec<&str>) -> Vec<ShortenedUrl> {
    let conn_pool = pool::get_connection_pool();
    let conn = &mut conn_pool.await.get().await.unwrap();

    let data = shortened_urls::table
        .filter(shortened_urls::short_code.eq_any(codes))
        .select(ShortenedUrl::as_select())
        .load(conn)
        .await
        .unwrap_or_else(|_| vec![]);

    // Increment click count if the URL exists
    if !data.is_empty() {
        diesel::insert_into(shortened_urls::table)
            .values(&data)
            .on_conflict(shortened_urls::short_code)
            .do_update()
            .set(shortened_urls::click_count.eq(diesel::dsl::sql("shortened_urls.click_count + 1")))
            .execute(conn)
            .await
            .unwrap();
    }

    data
}

pub async fn get_shortened_code_by_url(url: &str) -> Option<String> {
    let conn_pool = pool::get_connection_pool();
    let conn = &mut conn_pool.await.get().await.unwrap();

    let data = shortened_urls::table
        .filter(shortened_urls::original_url.eq(url))
        .select(shortened_urls::short_code)
        .first::<String>(conn)
        .await
        .ok();

    data
}

pub async fn insert_shortened_url(
    original_url: &str,
    short_code: &str,
) -> Result<ShortenedUrl, diesel::result::Error> {
    if short_code.len() != 10 {
        return Err(diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::SerializationFailure,
            Box::new(String::from(
                "Short code must be exactly 10 characters long",
            )),
        ));
    }

    let conn_pool = pool::get_connection_pool();
    let conn = &mut conn_pool.await.get().await.unwrap();

    let new_url = ShortenedUrl {
        id: 0, // ID will be auto-incremented
        original_url: original_url.to_string(),
        short_code: short_code.to_string(),
        click_count: 0,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    diesel::insert_into(shortened_urls::table)
        .values(&new_url)
        .execute(conn)
        .await
        .expect("Can't insert code on table");

    Ok(new_url)
}
