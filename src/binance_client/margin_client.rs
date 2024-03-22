use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::io::{Error as IOError, ErrorKind};
use log::trace;
use serde_json::Value;
use crate::binance_client::binance_client::BinanceClient;
use crate::binance_client::order_types::limit_order::LimitOrder;
use crate::binance_client::order_types::market_order::MarketOrder;
use crate::binance_client::order_types::oco_order::OcoOrder;
use crate::binance_client::order_types::stop_limit_order::StopLimitOrder;


pub struct MarginClient<'a> {
    binance_client: &'a BinanceClient,
}

impl MarginClient<'_> {
    pub fn new(api: &BinanceClient) -> MarginClient {
        MarginClient { binance_client: api }
    }

    pub async fn create_limit_order(&self, order: LimitOrder) -> Result<(), IOError> {
        let endpoint = "/sapi/v1/margin/order"; // Example endpoint for margin orders
        let url = format!("{}{}", self.binance_client.api_url, endpoint);
        let params = serde_qs::to_string(&order).map_err(|err| IOError::new(ErrorKind::InvalidInput, err.to_string()))?;

        let signature = self.binance_client.sign(&params);
        let full_params = format!("{}&signature={}", params, signature);

        let response = self.binance_client.get_client()
            .post(&url)
            .header("X-MBX-APIKEY", self.binance_client.get_api_key())
            .body(full_params)
            .send()
            .await
            .map_err(|err| IOError::new(ErrorKind::Other, format!("HTTP request failed: {}", err)))?;

        match response.status() {
            StatusCode::OK => Ok(()),
            _ => {
                let error_body = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                let error_message = serde_json::from_str::<Value>(&error_body)
                    .ok()
                    .and_then(|v| v["msg"].as_str().map(ToString::to_string))
                    .unwrap_or(error_body);

                Err(IOError::new(ErrorKind::Other, format!("Failed to create margin order: {}", error_message)))
            }
        }
    }
}
