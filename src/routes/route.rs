use std::future::Future;

#[derive(Debug, Clone)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
}

#[derive(Debug, Clone)]
pub struct Route<T> {
    pub path: String,
    pub method: Method,
    pub handler: fn() -> Box<dyn Future<Output = T> + Unpin>,
}

use super::shortener;

impl<T> Route<T> {
    pub fn new(
        path: &str,
        method: Method,
        handler: fn() -> Box<dyn Future<Output = T> + Unpin>,
    ) -> Self {
        Route {
            path: path.to_string(),
            method,
            handler,
        }
    }
}

pub async fn create_routes() -> Vec<Route<String>> {
    vec![shortener::shortener_route().await]
}
