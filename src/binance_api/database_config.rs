use std::fs;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub database: DatabaseConfig,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseConfig {
    pub name: String,
    pub user: String,
    pub password: String,
}


impl Config {
    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_contents = fs::read_to_string(file_path)?;
        let config: Self = toml::from_str(&config_contents)?;
        Ok(config)
    }
}


#[cfg(test)]
mod tests {
    use std::env;
    use dotenv::dotenv;
    use super::*;

    #[test]
    fn test_read_database_config() {
        // Make sure the path to the config.toml file is correct.
        // This path is relative to where you run `cargo test` from.
        let config = Config::from_file("/Users/Ciaran/Documents/binance-algo-trading-rs/tests/test_database_config.toml")
            .expect("Failed to load configuration");

        // Perform your assertions here
        // For example, check if the loaded configuration matches expected values:
        assert!(!config.database.name.is_empty(), "Database name should not be empty");
        assert!(!config.database.user.is_empty(), "Database user should not be empty");
        assert!(!config.database.password.is_empty(), "Database password should not be empty");

        // Optionally, print out the config for debugging
        println!("Loaded config: {:#?}", config);
    }


    #[test]
    fn test_load_env_database_config() {
        // Load environment variables from .env file
        dotenv().ok();

        let database_name = env::var("TEST_DATABASE_NAME").expect("DATABASE_NAME must be set");
        let database_user = env::var("TEST_DATABASE_USER").expect("DATABASE_USER must be set");
        let database_password = env::var("TEST_DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set");

        // Perform your assertions here
        assert!(!database_name.is_empty(), "Database name should not be empty");
        assert!(!database_user.is_empty(), "Database user should not be empty");
        assert!(!database_password.is_empty(), "Database password should not be empty");

        // Optionally, print out the loaded values for debugging
        println!("Loaded database name: {}", database_name);
        println!("Loaded database user: {}", database_user);
        println!("Loaded database password: {}", database_password);
    }
}

