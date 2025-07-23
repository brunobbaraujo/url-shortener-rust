use axum::{Router, routing::get, routing::post};

mod config;
mod db;
mod routes;
pub mod schema;

use crate::routes::route;

use config::Config;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let config = Config::from_env();
    let routes = route::create_routes();
    let mut app = Router::new();

    let mut routes_iter = routes.into_iter();
    while let Some(route) = routes_iter.next() {
        app = match route.method {
            route::Method::GET => app.route(&route.path, get(route.handler)),
            route::Method::POST => app.route(&route.path, post(route.handler)),
            route::Method::PUT => app.route(&route.path, axum::routing::put(route.handler)),
            route::Method::DELETE => app.route(&route.path, axum::routing::delete(route.handler)),
        };
    }

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.server, config.port))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
