use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::db::pool;
use crate::schema::shortened_urls;
// ordinary diesel model setup

#[derive(Queryable, QueryableByName, Selectable, Insertable)]
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
