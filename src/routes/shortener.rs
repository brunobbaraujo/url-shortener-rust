use super::route::{Method, Route};

pub fn shortener_handler() -> impl Future<Output = String> {
    async {
        // For now, we return a placeholder response
        "shortURL".to_string()
    }
}

pub fn shortener_route() -> Route<impl Future<Output = String>> {
    Route::new("/api/shorten", Method::POST, shortener_handler)
}
