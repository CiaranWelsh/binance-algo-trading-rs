use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::io::{Error as IOError, ErrorKind};
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use async_tungstenite::tungstenite::http::Uri;
use async_tungstenite::tungstenite::WebSocket;
use hmac::{Hmac, KeyInit, Mac};
use log::{error, info, trace};
use sha2::Sha256;
use futures::StreamExt;
use futures::FutureExt;
use futures_util::SinkExt;
use tokio::net::TcpStream;
use tokio::sync::TryAcquireError::Closed;
use tokio_websockets::{ClientBuilder, Error as WsError, Error, MaybeTlsStream, Message, WebSocketStream};
use tokio_websockets::upgrade::Response;


use crate::binance_api::account::order::Order;
use crate::binance_api::account::deserialization::deserialize_string_to_f64;
use crate::binance_api::account::open_order::OpenOrder;
use crate::binance_api::account::trades::Trade;
use crate::binance_api::database_client::DatabaseClient;
use crate::binance_api::streams::binance_stream::BinanceStreamTypes;
use crate::binance_api::streams::kline_data::KlineMessage;

const BINANCE_API_URL: &str = "https://api.binance.com/api";
const BINANCE_API_TEST_URL: &str = "https://testnet.binance.vision/api";

const BINANCE_WS_URL: &str = "wss://stream.binance.com:9443/ws";
const BINANCE_WS_TEST_URL: &str = "wss://testnet.binance.vision/ws";

const BINANCE_STREAM_URL: &str = "wss://stream.binance.com:9443/stream";
const BINANCE_STREAM_TEST_URL: &str = "wss://testnet.binance.vision/stream";

#[derive(Debug)]
pub struct BinanceClient {
    api_key: String,
    api_secret: String,
    is_live: bool,
    client: Client,
    db_client: Option<DatabaseClient>,
    pub api_url: String,
    pub websocket_url: String,
    pub stream_url: String,
    // user: String,
    // pwd: String,
    // dbname: String,
}

impl BinanceClient {
    pub async fn new(api_key: String, api_secret: String, is_live: bool) -> Self {
        let (api_url, websocket_url, stream_url) = if is_live {
            (BINANCE_API_URL.to_string(), BINANCE_WS_URL.to_string(), BINANCE_STREAM_URL.to_string())
        } else {
            (BINANCE_API_TEST_URL.to_string(), BINANCE_WS_TEST_URL.to_string(), BINANCE_STREAM_TEST_URL.to_string())
        };

        BinanceClient {
            api_key,
            api_secret,
            is_live,
            client: Client::new(),
            db_client: None,
            api_url,
            websocket_url,
            stream_url,
        }
    }

    pub fn set_live_mode(&mut self, is_live: bool) {
        self.is_live = is_live;
        self.api_url = if is_live { BINANCE_API_URL } else { BINANCE_API_TEST_URL }.to_string();
        self.websocket_url = if is_live { BINANCE_WS_URL } else { BINANCE_WS_TEST_URL }.to_string();
        self.stream_url = if is_live { BINANCE_STREAM_URL } else { BINANCE_STREAM_TEST_URL }.to_string();
    }


    // Method to optionally initialize the database client
    pub async fn init_db_client(&mut self, user: &str, pwd: &str, dbname: &str) -> Result<(), IOError> {
        // let user = user.unwrap_or("default_user");
        // let pwd = pwd.unwrap_or("default_password");
        // let dbname = dbname.unwrap_or("BinanceData");

        match DatabaseClient::connect_or_create_if_not_exist(dbname, user, pwd).await {
            Ok(db_client) => {
                self.db_client = Some(db_client);
                Ok(())
            }
            Err(e) => {
                Err(IOError::new(ErrorKind::Other, format!("Failed to initialize database client: {:?}", e)))
            }
        }
    }

    pub async fn ping(&self) -> Result<(), IOError> {
        let url = format!("{}/v3/ping", self.api_url);

        let res = self.client.get(&url).send().await.map_err(|err| {
            IOError::new(
                ErrorKind::Other,
                format!("Ping request failed: {}", err),
            )
        })?;

        if res.status().is_success() {
            println!("Ping successful");
            Ok(())
        } else {
            Err(IOError::new(
                ErrorKind::Other,
                "Ping failed with non-success status",
            ))
        }
    }

    pub fn get_client(&self) -> &Client {
        &self.client
    }
    pub fn generate_timestamp() -> Result<u64, IOError> {
        let start = SystemTime::now();
        let since_epoch = start
            .duration_since(UNIX_EPOCH)
            .map_err(|e| IOError::new(ErrorKind::Other, format!("Time error: {}", e)))?;
        Ok(since_epoch.as_millis() as u64)
    }

    pub fn sign(&self, message: &str) -> String {
        // Create alias for HMAC-SHA256
        type HmacSha256 = Hmac<Sha256>;

        let mut mac = HmacSha256::new_from_slice(self.api_secret.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(message.as_bytes());
        let result = mac.finalize();
        let code_bytes = result.into_bytes();

        // Convert HMAC result to hexadecimal string
        hex::encode(code_bytes)
    }

    pub fn get_api_key(&self) -> &str {
        &self.api_key
    }

    // Fetch all orders for a specific symbol
    pub async fn fetch_all_orders(&self, symbol: &str) -> Result<Vec<Order>, IOError> {
        let endpoint = "/v3/allOrders";
        let timestamp = Self::generate_timestamp().unwrap();
        let params = format!("symbol={}&timestamp={}", symbol, timestamp);
        let signature = self.sign(&params);
        let url = format!("{}{}?{}&signature={}", self.api_url, endpoint, params, signature);

        let response = self.client
            .get(&url)
            .header("X-MBX-APIKEY", self.api_key.clone())
            .send()
            .await
            .map_err(|err| IOError::new(ErrorKind::Other, format!("HTTP request failed: {}", err)))?;

        if response.status() == reqwest::StatusCode::OK {
            let orders = response
                .json::<Vec<Order>>()
                .await
                .map_err(|err| IOError::new(ErrorKind::Other, format!("Failed to deserialize orders: {}", err)))?;
            Ok(orders)
        } else {
            // Attempt to capture and log the error message from Binance
            let error_msg = response.text().await.unwrap_or_else(|_| "Failed to read error message".to_string());
            Err(IOError::new(ErrorKind::Other, format!("Failed to fetch all orders: {}", error_msg)))
        }
    }

    // Fetch all orders for a specific symbol
    pub async fn fetch_open_orders(&self, symbol: &str) -> Result<Vec<OpenOrder>, IOError> {
        let endpoint = "/v3/openOrders";
        let timestamp = Self::generate_timestamp().unwrap();
        let params = format!("symbol={}&timestamp={}", symbol, timestamp);
        let signature = self.sign(&params);
        let url = format!("{}{}?{}&signature={}", self.api_url, endpoint, params, signature);

        let response = self.client
            .get(&url)
            .header("X-MBX-APIKEY", self.api_key.clone())
            .send()
            .await
            .map_err(|err| IOError::new(ErrorKind::Other, format!("HTTP request failed: {}", err)))?;

        if response.status() == reqwest::StatusCode::OK {
            let orders = response
                .json::<Vec<OpenOrder>>()
                .await
                .map_err(|err| IOError::new(ErrorKind::Other, format!("Failed to deserialize orders: {}", err)))?;
            Ok(orders)
        } else {
            // Attempt to capture and log the error message from Binance
            let error_msg = response.text().await.unwrap_or_else(|_| "Failed to read error message".to_string());
            Err(IOError::new(ErrorKind::Other, format!("Failed to fetch all orders: {}", error_msg)))
        }
    }

    // Fetch all trades for a specific symbol
    pub async fn fetch_all_trades(&self, symbol: &str) -> Result<Vec<Trade>, IOError> {
        let endpoint = "/v3/myTrades";
        let timestamp = Self::generate_timestamp()?;
        let params = format!("symbol={}&timestamp={}", symbol, timestamp);
        let signature = self.sign(&params);
        let url = format!("{}{}?{}&signature={}", self.api_url, endpoint, params, signature);

        let response = self.client
            .get(&url)
            .header("X-MBX-APIKEY", self.api_key.clone())
            .send()
            .await
            .map_err(|err| IOError::new(ErrorKind::Other, format!("HTTP request failed: {}", err)))?;

        match response.status() {
            reqwest::StatusCode::OK => {
                let trades = response
                    .json::<Vec<Trade>>()
                    .await
                    .map_err(|err| IOError::new(ErrorKind::Other, format!("Failed to deserialize trades: {}", err)))?;
                Ok(trades)
            }
            _ => Err(IOError::new(ErrorKind::Other, "Failed to fetch all trades"))
        }
    }

    pub async fn get_listen_key(&self) -> Result<String, IOError> {
        let url = format!("{}/v3/userDataStream", self.api_url);
        let res = self.client.post(&url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await
            .map_err(|err| IOError::new(ErrorKind::Other, format!("Failed to get listen key: {}", err)))?;

        if res.status().is_success() {
            let data: serde_json::Value = res.json().await.map_err(|err| IOError::new(ErrorKind::Other, format!("Failed to parse listen key response: {}", err)))?;
            Ok(data["listenKey"].as_str().unwrap_or_default().to_string())
        } else {
            Err(IOError::new(ErrorKind::Other, "Failed to get listen key with non-success status"))
        }
    }

    pub async fn create_websocket_stream_with_listen_key(&self) -> Result<(), IOError> {
        match self.get_listen_key().await {
            Ok(listen_key) => {
                let ws_url = format!("{}/{}", self.websocket_url, listen_key);
                println!("Connecting to WebSocket at: {}", ws_url);
                // Connect to WebSocket using listen_key in ws_url...
                // This is a placeholder for your actual WebSocket connection logic.
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}


#[cfg(test)]
mod tests {
    use std::env;
    use std::io::Error;
    use dotenv::{dotenv, vars};
    use dotenv::Error::EnvVar;
    use env_logger::{Env, init};
    use log::LevelFilter;
    use log::LevelFilter::Trace;
    use super::*;
    use tokio;
    use url::quirks::username;
    use crate::binance_api::account::account_info::AccountInfoClient;
    use crate::binance_api::load_env::{EnvVars};
    use crate::binance_api::logger_conf::init_logger;

    #[tokio::test]
    async fn test_url_initialization() {
        let api_live = BinanceClient::new("".to_string(), "".to_string(), true);
        assert_eq!(api_live.await.api_url, BINANCE_API_URL);

        let api_test = BinanceClient::new("".to_string(), "".to_string(), false);
        assert_eq!(api_test.await.api_url, BINANCE_API_TEST_URL);
    }

    #[tokio::test]
    async fn test_set_live_mode() {
        let mut api = BinanceClient::new("".to_string(), "".to_string(), false)
            .await;
        assert_eq!(api.api_url, BINANCE_API_TEST_URL);

        api.set_live_mode(true);
        assert_eq!(api.api_url, BINANCE_API_URL);

        api.set_live_mode(false);
        assert_eq!(api.api_url, BINANCE_API_TEST_URL);
    }

    #[tokio::test]
    async fn check_orders() {
        init_logger(Trace);
        let vars = EnvVars::new();
        let mut api = BinanceClient::new(
            vars.api_key.to_string(), vars.api_secret.to_string(), false)
            .await;
        let orders = api.fetch_all_orders("ETHUSDT").await;
        match orders {
            Ok(order_data) => {
                trace!("{:?}", order_data);
            }
            Err(e) => { panic!("error: {}", e.to_string()) }
        }
    }

    #[tokio::test]
    async fn test_fetch_all_trades() {
        init_logger(LevelFilter::Trace); // Initialize logger if needed
        let vars = EnvVars::new();
        let mut api = BinanceClient::new(
            vars.api_key.to_string(), vars.api_secret.to_string(), false)
            .await;

        // Fetch all trades for a specific symbol
        let result = api.fetch_all_trades("BTCUSDT").await;

        match result {
            Ok(trades) => {
                trace!("No trades found, make sure the test account has trades for the symbol");
                trace!("Fetched trades: {:?}", trades);
                // Perform further assertions as necessary, e.g., checking if a specific trade exists
            }
            Err(e) => panic!("Failed to fetch trades: {}", e),
        }
    }

    #[tokio::test]
    async fn test_get_listen_key() {
        let vars = EnvVars::new();
        let mut api = BinanceClient::new(
            vars.api_key.to_string(), vars.api_secret.to_string(), false)
            .await;
        match api.get_listen_key().await {
            Ok(listen_key) => {
                assert!(!listen_key.is_empty(), "Listen key should not be empty");
                println!("Retrieved listen key: {}", listen_key);
            }
            Err(e) => panic!("Failed to retrieve listen key: {}", e),
        }
    }


    #[tokio::test]
    async fn test_database_lifecycle() {
        init_logger(Trace);
        let vars = EnvVars::new();

        trace!("vars: {:?}", vars);

        async fn drop_if_database_exists(vars: &EnvVars) {
            DatabaseClient::drop_database_if_exists(vars.name.as_str(), vars.user.as_str(), vars.pwd.as_str())
                .await.expect("Failed to delete the database")
        }
        async fn db_exist(vars: &EnvVars) -> bool {
            DatabaseClient::database_exists(vars.name.as_str(), vars.user.as_str(), vars.pwd.as_str()).await.unwrap()
        }

        // Ensure the database does not exist
        assert!(!db_exist(&vars).await, "Database should not exist at the start of the test");

        // Code to create the database
        // Assuming you have a method or process to do this
        // For example, using `init_db_client` if it creates the database when it doesn't exist
        let mut client = BinanceClient::new(vars.api_key.to_string(), vars.api_secret.to_string(), false).await;
        client.init_db_client(
            vars.name.as_str(),
            vars.user.as_str(),
            vars.pwd.as_str(),
        ).await.expect("Failed to initialize or create the database");

        // Verify the database now exists
        assert!(db_exist(&vars).await, "Database should exist after creation");

        client.db_client.unwrap().close().await;

        // Delete the database
        drop_if_database_exists(&vars).await;
        // Ensure the database no longer exists
        assert!(db_exist(&vars).await, "Database should be deleted by the end of the test");
    }


    #[tokio::test]
    async fn check_fetch_all_orders() {
        init_logger(Trace);
        let vars = EnvVars::new();
        let client = BinanceClient::new(vars.api_key, vars.api_secret, false).await;

        match AccountInfoClient::new(&client).await {
            Ok(account_info) => {
                trace!("account info fetched from binance: \n{:?}", account_info);
                let balances = account_info.balances;
                // for balance in balances.iter() {
                //     trace!("balance: {:?}", balance);
                // }
                let orders_opt = client.fetch_all_orders("ETHUSDT").await;
                match orders_opt {
                    Ok(orders) => {
                        for o in orders {
                            trace!("Order: {:?}", o);
                        }
                    }
                    Err(e) => { panic!("error: {:?}", e) }
                }
            }
            Err(e) => {
                // If the API call fails, ensure the test fails
                panic!("Failed to fetch account info: {}", e);
            }
        }

        // let orders = client.await.();
        // trace!("{:?}", client.fetch_all_orders())
    }
}




