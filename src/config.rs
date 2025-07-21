use dotenv::dotenv;
use std;

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

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_config_from_env() {
        unsafe {
            std::env::set_var("BACKEND_URL", "testserver");
            std::env::set_var("BACKEND_PORT", "8180");
            std::env::set_var("DATABASE_URL", "postgres://test:test@localhost/test");
        }
        let config = Config::from_env();
        assert_eq!(config.server, "testserver");
        assert_eq!(config.port, 8180);
        assert_eq!(config.database_url, "postgres://test:test@localhost/test");
    }
}
