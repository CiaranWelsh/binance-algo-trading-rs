use reqwest::Client;
use serde::{Deserialize, Serialize};
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
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::{Error as IOError, ErrorKind, Write};

use crate::binance_client::account::order::Order;
use crate::binance_client::deserialization::deserialize_string_to_f64;
use crate::binance_client::account::open_order::OpenOrder;
use crate::binance_client::account::trades::Trade;
use crate::binance_client::binance_error::BinanceError;
use crate::binance_client::database_client::DatabaseClient;
use crate::binance_client::exchange_info::ExchangeInfo;
use crate::binance_client::streams::binance_stream::BinanceStreamTypes;
use crate::binance_client::streams::kline_data::KlineMessage;
use crate::binance_client::ticker_price::TickerPrice;

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

    pub async fn fetch_exchange_info(&self) -> Result<ExchangeInfo, Box<dyn Error>> {
        let url = format!("{}/v3/exchangeInfo", self.api_url);

        // Make the HTTP GET request to the Binance API
        let response = reqwest::get(&url).await.map_err(|e| Box::new(e) as Box<dyn Error>)?;
        let response2 = reqwest::get(&url).await.map_err(|e| Box::new(e) as Box<dyn Error>)?;
        
        
        
        // let text = response2.text().await;
        // let p = "/Users/Ciaran/Documents/binance-algo-trading-rs/src/binance_client/string.txt";
        // let mut f = File::create(p).unwrap();
        // f.write(&text.unwrap().into_bytes()).unwrap();

        // Check if the request was successful
        if response.status().is_success() {
            // Parse the JSON response into the ExchangeInfo struct
            let x = response.json().await;
            // trace!("{:?}", x);
            let exchange_info: ExchangeInfo = x.map_err(|e| Box::new(e) as Box<dyn Error>)?;
            Ok(exchange_info)
        } else {
            // If the request was not successful, create an error
            let error_msg = response.text().await.unwrap_or_else(|_| "Failed to read error message".to_string());
            Err(Box::new(IOError::new(ErrorKind::Other, format!("Failed to fetch data: {}", error_msg))))
        }
    }


    // Generic function to fetch and deserialize data from Binance API
    async fn fetch_from_api<T: DeserializeOwned>(&self, endpoint: &str, params: &str) -> Result<Vec<T>, IOError> {
        let signature = self.sign(params);
        let url = format!("{}{}?{}&signature={}", self.api_url, endpoint, params, signature);

        let response = self.client
            .get(&url)
            .header("X-MBX-APIKEY", self.api_key.clone())
            .send()
            .await
            .map_err(|err| IOError::new(ErrorKind::Other, format!("HTTP request failed: {}", err)))?;


        if response.status().is_success() {
            response
                .json::<Vec<T>>()
                .await
                .map_err(|err| IOError::new(ErrorKind::Other, format!("Binance client: Failed to deserialize response: {}", err)))
        } else {
            // Attempt to capture and log the error message from Binance
            let error_msg = response.text().await.unwrap_or_else(|_| "Failed to read error message".to_string());
            Err(IOError::new(ErrorKind::Other, format!("Failed to fetch data: {}", error_msg)))
        }
    }

    pub async fn fetch_all_orders(&self, symbol: &str) -> Result<Vec<Order>, IOError> {
        let params = format!("symbol={}&timestamp={}", symbol, Self::generate_timestamp().unwrap());
        self.fetch_from_api::<Order>("/v3/allOrders", &params).await
    }

    pub async fn fetch_open_orders(&self, symbol: &str) -> Result<Vec<OpenOrder>, IOError> {
        let params = format!("symbol={}&timestamp={}", symbol, Self::generate_timestamp().unwrap());
        self.fetch_from_api::<OpenOrder>("/v3/openOrders", &params).await
    }

    pub async fn fetch_my_trades(&self, symbol: &str) -> Result<Vec<Trade>, IOError> {
        let params = format!("symbol={}&timestamp={}", symbol, Self::generate_timestamp().unwrap());
        self.fetch_from_api::<Trade>("/v3/myTrades", &params).await
    }

    pub async fn cancel_all_open_orders(&self, symbol: &str) -> Result<Vec<Value>, IOError> {
        /*
        
        // let orders: Vec<Order> = binance_client.fetch_all_orders(symbol).await
        //     .expect("Failed to fetch orders");
        // 
        // for order in orders {
        //     trace!("Order: {:?}", order);
        //     if order.status != OrderStatus::Filled {
        //         // tokio::time::sleep(tokio::time::Duration::from_secs(1)).await; // Wait a bit before attempting to cancel
        //         match spot_orders.cancel_order(symbol, order.order_id).await {
        //             Ok(_) => trace!("Order cancelled successfully."),
        //             Err(e) => eprintln!("Failed to cancel order: {:?}", e),
        //         }
        //     } else {
        //         trace!("Order already filled, cannot cancel.");
        //     }
        // }
         */
        let endpoint = "/v3/openOrders";
        let timestamp = Self::generate_timestamp()?;
        let params = format!("symbol={}&timestamp={}", symbol, timestamp);
        let signature = self.sign(&params);
        let url = format!("{}{}?{}&signature={}", self.api_url, endpoint, params, signature);

        let response = self.client
            .delete(&url)
            .header("X-MBX-APIKEY", self.api_key.clone())
            .send()
            .await
            .map_err(|err| IOError::new(ErrorKind::Other, format!("HTTP request failed: {}", err)))?;

        match response.status() {
            reqwest::StatusCode::OK => {
                let cancelled_orders = response.json::<Vec<Value>>().await
                    .map_err(|err| IOError::new(ErrorKind::Other, format!("Failed to deserialize canceled orders response: {}", err)))?;
                Ok(cancelled_orders)
            }
            reqwest::StatusCode::BAD_REQUEST => {
                let error_body = response.text().await.unwrap_or_else(|_| "Failed to read error message".to_string());
                if let Ok(error) = serde_json::from_str::<BinanceError>(&error_body) {
                    match error.code {
                        -2011 => Ok(vec![]), // Handle "Unknown order sent." as no orders to cancel
                        _ => Err(IOError::new(ErrorKind::Other, format!("Failed to cancel open orders: {} - {}", error.code, error.msg))),
                    }
                } else {
                    Err(IOError::new(ErrorKind::Other, format!("Failed to cancel open orders: Failed to parse error message")))
                }
            }
            _ => {
                // For all other unexpected status codes
                let error_msg = response.text().await.unwrap_or_else(|_| "Failed to read error message".to_string());
                Err(IOError::new(ErrorKind::Other, format!("Failed to cancel open orders: {}", error_msg)))
            }
        }
    }

    
    
    // Function to get the current price of a symbol
    pub async fn get_current_price(&self, symbol: &str) -> Result<TickerPrice, IOError> {
        let request_url = format!("{}/v3/ticker/price?symbol={}", self.api_url, symbol);

        let response = self.client
            .get(&request_url)
            .send()
            .await
            .map_err(|e| IOError::new(ErrorKind::Other, e.to_string()))?
            .json::<TickerPrice>()
            .await
            .map_err(|e| IOError::new(ErrorKind::Other, e.to_string()))?;

        Ok(response)
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


    // Method to listen for user data stream updates
    pub async fn listen_user_data_stream(&self) -> Result<(), IOError> {
        let listen_key = self.get_listen_key().await?;
        let ws_url = format!("{}/{}", self.websocket_url, listen_key);

        let (ws_stream, _) = tokio_tungstenite::connect_async(ws_url)
            .await
            .map_err(|e| IOError::new(ErrorKind::Other, format!("WebSocket connection failed: {}", e)))?;

        println!("Connected to WebSocket user data stream");

        let (write, mut read) = ws_stream.split();

        // Listen for messages
        while let Some(message) = read.next().await {
            match message {
                Ok(msg) => {
                    if let Ok(text) = msg.into_text() {
                        println!("Received message: {}", text);
                        // Here you would handle the message, for example by checking if it indicates a filled trade
                    }
                }
                Err(e) => {
                    println!("Error receiving message: {}", e);
                    break;
                }
            }
        }

        Ok(())
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
    use tokio_postgres::types::Format::Binary;
    use url::quirks::username;
    use crate::binance_client::account::account_info::AccountInfoClient;
    use crate::binance_client::load_env::{EnvVars};
    use crate::binance_client::logger_conf::init_logger;
    use crate::binance_client::order_types::limit_order::LimitOrder;
    use crate::binance_client::order_types::side::Side;
    use crate::binance_client::spot_orders::SpotClient;

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
    async fn test_exhangee() {
        init_logger(Trace);
        let vars = EnvVars::new();
        let mut api = BinanceClient::new(
            vars.api_key.to_string(), vars.api_secret.to_string(), false)
            .await;
        
        let data = api.fetch_exchange_info().await;
        match data {
            Ok(d) => {
                // trace!("{:?}", d.clone());
            }
            Err(err) => {
                panic!("err: {:?}", err);
            }
        }
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
        let result = api.fetch_my_trades("ETHUSDT").await;

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
                // trace!("account info fetched from binance: \n{:?}", account_info);
                // let balances = account_info.balances;
                // for balance in balances.iter() {
                //     trace!("balance: {:?}", balance);
                // }
                let orders_opt = client.fetch_all_orders("ETHUSDT").await;
                trace!("Orders: {:?}", orders_opt);
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

    #[tokio::test]
    async fn test_fetch_open_orders() {
        init_logger(Trace);
        let vars = EnvVars::new();
        let client = BinanceClient::new(vars.api_key, vars.api_secret, false).await;

        let result = client.fetch_open_orders("ETHUSDT").await;

        match result {
            Ok(orders) => {
                // assert!(!orders.is_empty(), "Should fetch at least one open order");
                // trace!("Fetched open orders: {:?}", orders);
            }
            Err(e) => panic!("Failed to fetch open orders: {}", e),
        }
    }

    #[tokio::test]
    async fn test_cancel_all_open_orders() {
        init_logger(Trace);
        let vars = EnvVars::new();
        let client = BinanceClient::new(vars.api_key, vars.api_secret, false).await;
        let spot = SpotClient::new(&client);

        let ts = BinanceClient::generate_timestamp().unwrap();
        spot.create_limit_order(
            LimitOrder::new("ETHUSDC", Side::Buy, 0.01, 2500.0, ts)
        ).await.unwrap();

        trace!("{:?}", client.fetch_open_orders("ETHUSDT").await.unwrap());

        // Ensure there's at least one open order for the test to be meaningful
        // This part is skipped here but ensure to have an open order for "ETHUSDT" or change the symbol accordingly


        // Attempt to cancel all open orders
        let result = client.cancel_all_open_orders("ETHUSDT").await;

        match result {
            Ok(cancelled_orders) => {
                // If there are no open orders, the array may be empty
                // assert!(
                //     !cancelled_orders.is_empty(),
                //     "Open orders should have been cancelled but the cancelled orders list is empty"
                // );
                trace!("Cancelled orders: {:?}", cancelled_orders);
            }
            Err(e) => panic!("Failed to cancel open orders: {}", e),
        }

        // Optionally, you can verify that there are no more open orders for the symbol
        let open_orders_result = client.fetch_open_orders("ETHUSDT").await;
        assert!(
            open_orders_result.unwrap().is_empty(),
            "There should be no open orders after cancellation"
        );
    }

    // 
    // #[tokio::test]
    // async fn test_listen_user_data_stream() {
    //     let vars = EnvVars::new(); // Ensure this fetches testnet API keys and URLs
    //     let client = BinanceClient::new(vars.api_key, vars.api_secret, false).await;
    //     let spot_client = SpotClient::new(&client);
    // 
    //     // This should be an unlikely price to ensure the order won't be filled immediately
    //     let test_order_price = 0.01;
    //     let symbol = "BNBUSDT";
    //     let quantity = 1.0;
    // 
    //     // Place a limit order
    //     let order_response = client.place_limit_order(symbol, Side::Buy, quantity, test_order_price).await;
    //     assert!(order_response.is_ok(), "Failed to place test limit order");
    // 
    //     // Listen to the user data stream for a short period to catch the order update
    //     let listen_result = client.listen_user_data_stream().await;
    //     assert!(listen_result.is_ok(), "Listening to user data stream failed");
    // 
    //     // Optionally, clean up by canceling the test order to avoid leaving open orders on the testnet account
    //     // Implementation of cancel_order() is assumed to exist
    //     let cancel_response = spot_client.cancel_order(symbol, order_response.unwrap().order_id).await;
    //     assert!(cancel_response.is_ok(), "Failed to cancel test limit order");
    // 
    //     // This test is inherently limited by its reliance on the behavior of the Binance testnet and the presence of real-time updates.
    //     // It does not assert on receiving the specific WebSocket message due to the asynchronous and unpredictable nature of such messages.
    //     // In a real application, consider more sophisticated methods to verify WebSocket communications, such as mocking or event recording.
    // }
}




