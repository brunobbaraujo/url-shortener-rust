use super::route::{Method, Route};
use crate::config;
use crate::db;
use axum::extract::Path;
use axum::response::Redirect;
use std::future::Future;

pub fn url_route() -> Route<impl Future<Output = Redirect>> {
    Route::new("/:code", Method::GET, handle_shortened_url)
}

async fn handle_shortened_url(Path(code): Path<String>) -> Redirect {
    // Get the URL record from the database
    let records = db::get_shortened_url_by_code(&code).await;

    if let Some(record) = records.first() {
        // Update click count and last accessed time could be done here
        // For now, just redirect to the original URL
        Redirect::to(&record.original_url)
    } else {
        // If no matching record is found, redirect to homepage or error page
        Redirect::to("/")
    }
}
