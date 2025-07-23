use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::db::pool;
use crate::schema::shortened_urls;
// ordinary diesel model setup

#[derive(Queryable, QueryableByName, Selectable)]
#[diesel(table_name = crate::schema::shortened_urls)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ShortenedUrl {
    pub id: i32,
    pub original_url: String,
    pub short_code: String,
    pub click_count: Option<i32>,
    pub last_accessed: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn get_shortened_url_by_code(code: &str) -> Vec<ShortenedUrl> {
    if code.len() != 10 {
        return vec![]; // Return empty if code is not valid
    }

    let conn_pool = pool::get_connection_pool();
    let conn = &mut conn_pool.await.get().await.unwrap();

    let data = shortened_urls::table
        .filter(shortened_urls::short_code.eq(code))
        .select(ShortenedUrl::as_select())
        .limit(1)
        // execute the query via the provided
        // async `diesel_async::RunQueryDsl`
        .load(conn)
        .await
        .unwrap_or_else(|_| vec![]);
    data
}
