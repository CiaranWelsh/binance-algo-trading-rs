use std::collections::HashMap;
use std::{env, hash};
use dotenv::dotenv;
use uuid::Uuid;

#[derive(Debug)]
pub struct EnvVars {
    pub name: String,
    pub user: String,
    pub pwd: String,
    pub api_key: String,
    pub api_secret: String,
}

impl EnvVars {
    pub fn new() -> Self {
        dotenv().ok();
        let unique_suffix = Uuid::new_v4();
        let db_name = format!("{}_{}", env::var("TEST_DATABASE_NAME").expect("DATABASE_NAME must be set"), unique_suffix);

        Self {
            name: db_name,
            user: env::var("TEST_DATABASE_USER").expect("TEST_DATABASE_USER must be set"),
            pwd: env::var("TEST_DATABASE_PASSWORD").expect("TEST_DATABASE_PASSWORD must be set"),
            api_key: env::var("TEST_NET_API_KEY").expect("TEST_NET_API_KEY must be set"),
            api_secret: env::var("TEST_NET_API_SECRET").expect("TEST_NET_API_SECRET must be set"),
        }
    }
}