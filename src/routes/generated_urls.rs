use super::route::{Method, Route};
use crate::config;
use axum::extract::Path;

pub fn url_route(Path(shortened_url): Path<String>) -> Route<impl Future<Output = String>> {
    fn _url_handler() -> impl Future<Output = String> {
        async { "shortURL".to_string() }
    }

    Route::new("/api/shorten", Method::POST, _url_handler)
}
