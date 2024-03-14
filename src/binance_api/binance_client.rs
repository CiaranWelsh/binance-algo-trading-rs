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
use crate::binance_api::account::trades::Trade;
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
    pub api_url: String,
    pub websocket_url: String,
    pub stream_url: String,
}

impl BinanceClient {
    pub fn new(api_key: String, api_secret: String, is_live: bool) -> Self {
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


    // pub async fn create_websocket_stream(&self, streams: Vec<BinanceStreamTypes>) -> Result<(), IOError> {
    //     let combined_streams: Vec<String> = streams.into_iter().map(|s| s.to_stream_path()).collect();
    //     let stream_paths = combined_streams.join("/");
    //     let ws_url = format!("{}?streams={}", self.stream_url, stream_paths);
    //
    //     trace!("ws url: {:?}", ws_url);
    //
    //     // Convert ws_url string to Uri
    //     let uri = match ws_url.parse::<Uri>() {
    //         Ok(uri) => uri,
    //         Err(e) => return Err(IOError::new(ErrorKind::Other, format!("Invalid WebSocket URL: {}", e))),
    //     };
    //
    //     // Connect to the WebSocket server
    //     let (mut client, _) = ClientBuilder::from_uri(uri)
    //         .connect()
    //         .await
    //         .map_err(|e| IOError::new(ErrorKind::Other, format!("Failed to connect: {}", e)))?;
    //
    //     info!("WebSocket connected: {:?}" ,client);
    //
    //     // Example sending a message, replace or remove as needed
    //     // client.send(Message::text("Your command here")).await
    //     //     .map_err(|e| IOError::new(ErrorKind::Other, format!("Send Error: {}", e)))?;
    //
    //     while let Some(message) = client.next().await {
    //         match message {
    //             Ok(msg) => {
    //                 if msg.is_ping() {
    //                     trace!("Is ping: {:?}", msg);
    //                     let pong = Message::ping(msg.into_payload());
    //                     // let response_message = Message::pong(msg.as_payload());
    //                     if let Err(e) = client.send(pong){
    //                         error!("Responding to binance's ping with pong failed: {:?}", e);
    //                     }
    //
    //                 } else if msg.is_pong() {
    //                     trace!("Is pong: {:?}", msg);
    //                 } else if msg.is_binary() {
    //                     trace!("Is binary: {:?}", msg.as_payload());
    //                 } else if msg.is_text() {
    //                     match serde_json::from_str::<WebSocketMessage>(msg.as_text().unwrap()) {
    //                         Ok(parsed_message) => {
    //                             // Now you have your deserialized message
    //                             // You can handle it according to your logic
    //                             println!("Parsed kline data: {:?}", parsed_message);
    //                         },
    //                         Err(e) => {
    //                             error!("Failed to parse JSON: {}", e);
    //                         }
    //                     }
    //                 } else if msg.is_close() {
    //                     trace!("Is close: {:?}", msg.as_close());
    //                 } else { panic!("Unexpected message: {:?}", msg) }
    //             }
    //             Err(e) => error!("Error receiving message: {:?}", e),
    //         }
    //     }
    //
    //     Ok(())
    // }
}


#[cfg(test)]
mod tests {
    use std::io::Error;
    use log::LevelFilter;
    use log::LevelFilter::Trace;
    use super::*;
    use tokio;
    use crate::binance_api::auth::{TEST_NET_API_KEY, TEST_NET_API_SECRET};
    use crate::binance_api::logger_conf::init_logger;

    #[tokio::test]
    async fn test_url_initialization() {
        let api_live = BinanceClient::new("".to_string(), "".to_string(), true);
        assert_eq!(api_live.api_url, BINANCE_API_URL);

        let api_test = BinanceClient::new("".to_string(), "".to_string(), false);
        assert_eq!(api_test.api_url, BINANCE_API_TEST_URL);
    }

    #[tokio::test]
    async fn test_set_live_mode() {
        let mut api = BinanceClient::new("".to_string(), "".to_string(), false);
        assert_eq!(api.api_url, BINANCE_API_TEST_URL);

        api.set_live_mode(true);
        assert_eq!(api.api_url, BINANCE_API_URL);

        api.set_live_mode(false);
        assert_eq!(api.api_url, BINANCE_API_TEST_URL);
    }

    #[tokio::test]
    async fn check_orders() {
        init_logger(Trace);
        let mut api = BinanceClient::new(
            TEST_NET_API_KEY.to_string(), TEST_NET_API_SECRET.to_string(), false);
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

        // Initialize the BinanceAPI instance with testnet credentials
        let api = BinanceClient::new(TEST_NET_API_KEY.to_string(), TEST_NET_API_SECRET.to_string(), false);

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
        let api = BinanceClient::new(TEST_NET_API_KEY.to_string(), TEST_NET_API_SECRET.to_string(), false);
        match api.get_listen_key().await {
            Ok(listen_key) => {
                assert!(!listen_key.is_empty(), "Listen key should not be empty");
                println!("Retrieved listen key: {}", listen_key);
            }
            Err(e) => panic!("Failed to retrieve listen key: {}", e),
        }
    }
}



