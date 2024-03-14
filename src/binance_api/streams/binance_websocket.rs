use async_tungstenite::tungstenite::http::Uri;
use log::{error, info, trace};
use reqwest::Client;
use std::io::{Error as IOError, ErrorKind};
use async_std::stream::StreamExt;
use futures_util::future::err;
use futures_util::SinkExt;
use serde_json::{Error, Value};
use tokio_websockets::{ClientBuilder, Message};
use regex::Regex;
use crate::binance_api::binance_client::BinanceClient;
use crate::binance_api::streams::binance_stream::BinanceStreamTypes;
use crate::binance_api::streams::depth_stream::DepthMessage;
use crate::binance_api::streams::kline_data::{Kline, KlineMessage};

#[derive(Debug)]
pub struct BinanceWebSocket<'a> {
    binance_client: &'a BinanceClient,
}

impl<'a> BinanceWebSocket<'a> {
    pub fn new(binance_client: &'a BinanceClient) -> Self {
        Self {
            binance_client,
        }
    }

    // The method to connect to WebSocket and listen for messages
    pub async fn connect_and_listen(&self, streams: Vec<BinanceStreamTypes>) -> Result<(), IOError> {
        let combined_streams: Vec<String> = streams.into_iter().map(|s| s.to_stream_path()).collect();
        let stream_paths = combined_streams.join("/");
        let ws_url = format!("{}?streams={}", self.binance_client.stream_url, stream_paths);

        trace!("ws url: {:?}", ws_url);

        // Convert ws_url string to Uri
        let uri = match ws_url.parse::<Uri>() {
            Ok(uri) => uri,
            Err(e) => return Err(IOError::new(ErrorKind::Other, format!("Invalid WebSocket URL: {}", e))),
        };

        // Connect to the WebSocket server
        let (mut client, _) = ClientBuilder::from_uri(uri)
            .connect()
            .await
            .map_err(|e| IOError::new(ErrorKind::Other, format!("Failed to connect: {}", e)))?;

        info!("WebSocket connected: {:?}" ,client);


        while let Some(message) = client.next().await {
            match message {
                Ok(msg) => {
                    if msg.is_ping() {
                        trace!("Is ping: {:?}", msg);
                        let pong = Message::ping(msg.into_payload());
                        // let response_message = Message::pong(msg.as_payload());
                        if let Err(e) = client.send(pong).await {
                            error!("Responding to binance's ping with pong failed: {:?}", e);
                        }
                    } else if msg.is_pong() {
                        trace!("Is pong: {:?}", msg);
                    } else if msg.is_binary() {
                        trace!("Is binary: {:?}", msg.as_payload());
                    } else if msg.is_text() {
                        let message_as_string: Value = serde_json::from_str(msg.as_text().unwrap()).expect("Failed to parse message to JSON");

                        if let Some(stream_name) = message_as_string["stream"].as_str() {
                            let re = Regex::new(r"^(?P<symbol>[a-z0-9]+)@(?P<stream_type>[a-z]+)(?:_(?P<detail>[a-z0-9]+))?$").unwrap();

                            if let Some(caps) = re.captures(stream_name) {
                                let symbol = caps.name("symbol").map_or("", |m| m.as_str());
                                let stream_type = caps.name("stream_type").map_or("", |m| m.as_str());
                                let detail = caps.name("detail").map_or("", |m| m.as_str()); // This can be empty for streams without additional details

                                trace!("symbol: {:?}, stream type: {:?}, detail: {:?}", symbol, stream_type, detail);

                                match stream_type {
                                    "depth" => {
                                        // Handle depth message
                                        if let Ok(depth_message) = serde_json::from_value::<DepthMessage>(message_as_string.clone()) {
                                            println!("Received depth message for {}: {:?}", symbol, depth_message);
                                        } else {
                                            error!("Failed to parse depth message.\n {:?}", message_as_string);
                                        }
                                    }
                                    "kline" => {
                                        assert!(!detail.is_empty(), "No time on kline?");
                                        // Now you know the interval or detail of the kline stream
                                        let result = serde_json::from_value::<KlineMessage>(message_as_string.clone());
                                        match result {
                                            Ok(kline_message) => {
                                                println!("Received kline message for {}: interval: {}, data: {:?}", symbol, detail, kline_message);
                                            }
                                            Err(error) => {
                                                panic!("error: {:?}", error);
                                            }
                                        }
                                    }
                                    // Add more cases for other stream types as needed
                                    _ => error!("Unknown stream type: {}", stream_type),
                                }
                            } else {
                                error!("Stream name does not match expected pattern: {:?}", stream_name);
                            }
                        }
                    } else if msg.is_close() {
                        trace!("Is close: {:?}", msg.as_close());
                    } else { panic!("Unexpected message: {:?}", msg) }
                }
                Err(e) => error!("Error receiving message: {:?}", e),
            }
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use std::hint::black_box;
    use log::LevelFilter::Trace;
    use log::trace;
    use crate::binance_api::auth::{TEST_NET_API_KEY, TEST_NET_API_SECRET};
    use crate::binance_api::binance_client::BinanceClient;
    use crate::binance_api::logger_conf::init_logger;
    use crate::binance_api::streams::binance_stream::BinanceStreamTypes;
    use crate::binance_api::streams::binance_websocket::BinanceWebSocket;

    #[tokio::test]
    async fn kline_websocket_stream_test() {
        // Assuming `init_logger` and `TEST_NET_API_KEY`, `TEST_NET_API_SECRET` are available
        init_logger(Trace);

        let binance_client = BinanceClient::new(TEST_NET_API_KEY.to_string(), TEST_NET_API_SECRET.to_string(), false)
            .await;
        let websocket_api = BinanceWebSocket::new(&binance_client);

        // Define the streams you want to subscribe to
        let streams = vec![
            BinanceStreamTypes::Kline("btcusdt".to_string(), "1m".to_string()),
            BinanceStreamTypes::Kline("ethusdt".to_string(), "1m".to_string()),
            // BinanceStream::Depth(symbol.clone()),
            // BinanceStream::Trade(symbol.clone()),
        ];

        // Call the method to create and listen to the websocket stream
        let result = websocket_api.connect_and_listen(streams).await;

        trace!("result: {:?}",result);

        // match result {
        //     Ok(data) => {
        //         info!("Successfully connected and processed messages from WebSocket streams.");
        //         info!("Data {:?}", data);
        //     },
        //     Err(e) => panic!("Failed to connect or process messages: {}", e),
        // }
    }

    #[tokio::test]
    async fn depth_websocket_stream_test() {
        // Assuming `init_logger` and `TEST_NET_API_KEY`, `TEST_NET_API_SECRET` are available
        init_logger(Trace);

        let binance_client = BinanceClient::new(TEST_NET_API_KEY.to_string(), TEST_NET_API_SECRET.to_string(), false)
            .await;
        let websocket_api = BinanceWebSocket::new(&binance_client);

        // Define the streams you want to subscribe to
        let streams = vec![
            BinanceStreamTypes::Depth("ethusdt".to_string()),
        ];

        // Call the method to create and listen to the websocket stream
        let result = websocket_api.connect_and_listen(streams).await;

        trace!("result: {:?}",result);

        // match result {
        //     Ok(data) => {
        //         info!("Successfully connected and processed messages from WebSocket streams.");
        //         info!("Data {:?}", data);
        //     },
        //     Err(e) => panic!("Failed to connect or process messages: {}", e),
        // }
    }
}

