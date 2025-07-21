use axum::{Router, routing::get};
mod config;

use config::Config;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let config = Config::from_env();
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.server, config.port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
