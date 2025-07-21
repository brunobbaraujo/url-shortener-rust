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
    pub handler: fn() -> T,
}

use super::shortener;

impl<T> Route<T> {
    pub fn new(path: &str, method: Method, handler: fn() -> T) -> Self {
        Route {
            path: path.to_string(),
            method,
            handler,
        }
    }
}

pub fn create_routes() -> Vec<Route<impl Future<Output = String>>> {
    vec![shortener::shortener_route()]
}
