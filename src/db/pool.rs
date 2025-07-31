use crate::config::Config;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool::Pool;
use tokio::sync::OnceCell;

async fn build_connection_pool() -> Pool<AsyncPgConnection> {
    let config = Config::from_env();
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(config.database_url);
    Pool::builder(manager)
        .max_size(config.pool_size)
        .build()
        .expect("Failed to create database connection pool")
}

pub async fn get_connection_pool() -> &'static Pool<AsyncPgConnection> {
    static POOL: OnceCell<Pool<AsyncPgConnection>> = OnceCell::const_new();
    POOL.get_or_init(build_connection_pool).await
}
