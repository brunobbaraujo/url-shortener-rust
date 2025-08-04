use crate::db;
use axum::extract::Path;
use axum::response::Redirect;

pub async fn redirect_handler(Path(code): Path<String>) -> Redirect {
    // Get the URL record from the database
    let records = db::get_original_url_by_codes(vec![&code]).await;

    if let Some(record) = records.first() {
        // Update click count and last accessed time could be done here
        // For now, just redirect to the original URL
        Redirect::to(&record.original_url)
    } else {
        // If no matching record is found, redirect to homepage or error page
        Redirect::to("/")
    }
}
