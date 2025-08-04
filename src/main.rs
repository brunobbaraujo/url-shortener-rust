use axum::{Router, routing::get, routing::post};
use tower_http::cors::CorsLayer;

mod config;
mod db;
mod routes;
pub mod schema;

use config::Config;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let config = Config::from_env();

    let app = Router::new()
        .route("/api/shorten", post(routes::shorten_handler))
        .route("/{code}", get(routes::redirect_handler))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.server, config.port))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
