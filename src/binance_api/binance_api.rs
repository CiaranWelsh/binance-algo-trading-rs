use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::io::{Error as IOError, ErrorKind};
use std::time::{SystemTime, UNIX_EPOCH};
use hmac::{Hmac, KeyInit, Mac};
use log::trace;
use sha2::Sha256;
use crate::binance_api::account::order::Order;
use crate::binance_api::account::deserialization::deserialize_string_to_f64;

const BINANCE_API_URL: &str = "https://api.binance.com/api";
const BINANCE_API_TEST_URL: &str = "https://testnet.binance.vision/api";
const BINANCE_WS_URL: &str = "wss://stream.binance.com:9443/ws";
const BINANCE_WS_TEST_URL: &str = "wss://testnet.binance.vision/ws";
const BINANCE_STREAM_URL: &str = "wss://stream.binance.com:9443/stream";
const BINANCE_STREAM_TEST_URL: &str = "wss://testnet.binance.vision/stream";

#[derive(Debug)]
pub struct BinanceAPI {
    api_key: String,
    api_secret: String,
    is_live: bool,
    client: Client,
    pub api_url: String,
    pub websocket_url: String,
    pub stream_url: String,
}

impl BinanceAPI {
    pub fn new(api_key: String, api_secret: String, is_live: bool) -> Self {
        let (api_url, websocket_url, stream_url) = if is_live {
            (BINANCE_API_URL.to_string(), BINANCE_WS_URL.to_string(), BINANCE_STREAM_URL.to_string())
        } else {
            (BINANCE_API_TEST_URL.to_string(), BINANCE_WS_TEST_URL.to_string(), BINANCE_STREAM_TEST_URL.to_string())
        };

        BinanceAPI {
            api_key,
            api_secret,
            is_live,
            client: Client::new(),
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
    pub async fn fetch_all_orders(&self, symbol: &str) -> Result<Vec<Order>, IOError>{
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

        match response.status() {
            reqwest::StatusCode::OK => {
                // let parsed_result: String= serde_json::from_str(&response.text().await.unwrap().clone()).unwrap();
                // trace!("Parsed: {:?}", response.text().await);
                //
                // Ok(Vec::new())

                let orders = response
                    .json::<Vec<Order>>()
                    .await
                    .map_err(|err| IOError::new(ErrorKind::Other, format!("Failed to deserialize orders: {}", err)))?;
                Ok(orders)
            }
            _ => Err(IOError::new(ErrorKind::Other, "Failed to fetch all orders"))
        }
    }
}


#[cfg(test)]
mod tests {
    use std::io::Error;
    use log::LevelFilter::Trace;
    use super::*;
    use tokio;
    use crate::binance_api::auth::{TEST_NET_API_KEY, TEST_NET_API_SECRET};
    use crate::binance_api::logger_conf::init_logger;

    #[tokio::test]
    async fn test_url_initialization() {
        let api_live = BinanceAPI::new("".to_string(), "".to_string(), true);
        assert_eq!(api_live.api_url, BINANCE_API_URL);

        let api_test = BinanceAPI::new("".to_string(), "".to_string(), false);
        assert_eq!(api_test.api_url, BINANCE_API_TEST_URL);
    }

    #[tokio::test]
    async fn test_set_live_mode() {
        let mut api = BinanceAPI::new("".to_string(), "".to_string(), false);
        assert_eq!(api.api_url, BINANCE_API_TEST_URL);

        api.set_live_mode(true);
        assert_eq!(api.api_url, BINANCE_API_URL);

        api.set_live_mode(false);
        assert_eq!(api.api_url, BINANCE_API_TEST_URL);
    }

    #[tokio::test]
    async fn check_orders(){
        init_logger(Trace);
        let mut api = BinanceAPI::new(
            TEST_NET_API_KEY.to_string(), TEST_NET_API_SECRET.to_string(), false);
        let orders = api.fetch_all_orders("ETHUSDT").await;
        match orders {
            Ok(order_data) => {
                trace!("{:?}", order_data);
            }
            Err(e) => {panic!("error: {}", e.to_string())}
        }
    }
}

