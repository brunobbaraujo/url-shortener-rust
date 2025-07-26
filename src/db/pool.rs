use crate::config::Config;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool::Pool;
use std::sync::LazyLock;
use tokio::sync::OnceCell;

use diesel_async::AsyncPgConnection;

/// Database connection pool for the application.
/// Uses the DATABASE_URL from the Config struct, which is loaded from environment variables.
pub type DbPool = Pool<diesel_async::AsyncPgConnection>;

/// Static connection pool that's lazily initialized on first use.
pub static POOL: LazyLock<DbPool> = LazyLock::new(|| {
    let config = Config::from_env();
    let database_url = config.database_url;
    let conn_config =
        AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);

    Pool::builder(conn_config)
        .max_size(config.pool_size) // Adjust based on your application needs
        .build()
        .expect("Failed to create database connection pool")
});

async fn build_connection_pool() -> Pool<AsyncPgConnection> {
    let connection_url = Config::from_env().database_url;
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(connection_url);
    Pool::builder(manager).build().unwrap()
}

pub async fn get_connection_pool() -> &'static Pool<AsyncPgConnection> {
    static POOL: OnceCell<Pool<AsyncPgConnection>> = OnceCell::const_new();
    POOL.get_or_init(build_connection_pool).await
}
