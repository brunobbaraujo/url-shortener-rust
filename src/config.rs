use dotenv::dotenv;

pub struct Config {
    pub server: String,
    pub port: u16,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok(); // Load environment variables from .env file
        let server = std::env::var("BACKEND_URL").unwrap_or_else(|_| "localhost".to_string());
        let port = std::env::var("BACKEND_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .unwrap_or(3000);
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://root:root@localhost/postgres".to_string());

        Config {
            server,
            port,
            database_url,
        }
    }
}
