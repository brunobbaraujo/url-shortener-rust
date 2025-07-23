use super::route::{Method, Route};

pub async fn shortener_handler() -> String {
    "shortURL".to_string()
}

pub async fn shortener_route() -> Route<String> {
    Route::new("/api/shorten", Method::POST, shortener_handler)
}
