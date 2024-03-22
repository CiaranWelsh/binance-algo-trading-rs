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

pub struct SpotClient<'a> {
    binance_client: &'a BinanceClient,
}

impl SpotClient<'_> {
    pub fn new(api: &BinanceClient) -> SpotClient {
        SpotClient { binance_client: api }
    }


    pub async fn create_limit_order(&self, order: LimitOrder) -> Result<(), IOError> {
        let endpoint = "/v3/order";
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
                // Attempt to decode the response body to get the error message
                let error_body = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                // Optionally, parse the JSON to extract a specific field
                let error_message = serde_json::from_str::<Value>(&error_body)
                    .ok()
                    .and_then(|v| v["msg"].as_str().map(ToString::to_string))
                    .unwrap_or(error_body);

                Err(IOError::new(ErrorKind::Other, format!("Failed to create limit order: {}", error_message)))
            }
        }
    }


    pub async fn create_stop_limit_order(&self, order: StopLimitOrder) -> Result<(), IOError> {
        let endpoint = "/v3/order";
        let url = format!("{}{}", self.binance_client.api_url, endpoint);
        let params = serde_qs::to_string(&order).map_err(|err| IOError::new(ErrorKind::InvalidInput, format!("Failed to serialize order: {}", err)))?;


        let signature = self.binance_client.sign(&params);
        let full_params = format!("{}&signature={}", params, signature);
        trace!("full input params: {:?}", full_params);

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

                Err(IOError::new(ErrorKind::Other, format!("Failed to create stop-limit order: {}", error_message)))
            }
        }
    }


    pub async fn create_oco_order(&self, order: OcoOrder) -> Result<(), IOError> {
        let endpoint = "/v3/order/oco";
        let url = format!("{}{}", self.binance_client.api_url, endpoint);
        let params = serde_qs::to_string(&order).map_err(|err| IOError::new(ErrorKind::InvalidInput, format!("Failed to serialize order: {}", err)))?;

        let signature = self.binance_client.sign(&params);
        let full_params = format!("{}&signature={}", params, signature);

        
        trace!("Full params: {:?}", full_params);
        let response = self.binance_client.get_client()
            .post(&url)
            .header("X-MBX-APIKEY", self.binance_client.get_api_key())
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(full_params)
            .send()
            .await
            .map_err(|err| IOError::new(ErrorKind::Other, format!("HTTP request failed: {}", err)))?;
        

        match response.status() {
            StatusCode::OK => Ok(()),
            _ => {
                let error_body = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                trace!("Error body : {:?}", error_body);
                let error_message = serde_json::from_str::<Value>(&error_body)
                    .ok()
                    .and_then(|v| v["msg"].as_str().map(ToString::to_string))
                    .unwrap_or(error_body);

                Err(IOError::new(ErrorKind::Other, format!("Failed to create OCO order: {}", error_message)))
            }
        }
    }


    pub async fn create_market_order(&self, order: MarketOrder) -> Result<(), IOError> {
        let endpoint = "/v3/order";
        let url = format!("{}{}", self.binance_client.api_url, endpoint);
        let params = serde_qs::to_string(&order).map_err(|err| IOError::new(ErrorKind::InvalidInput, format!("Failed to serialize order: {}", err)))?;

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
            StatusCode::OK => {
                trace!("response: {:?}", response);
                Ok(())
            },
            _ => {
                let error_body = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                let error_message = serde_json::from_str::<Value>(&error_body)
                    .ok()
                    .and_then(|v| v["msg"].as_str().map(ToString::to_string))
                    .unwrap_or(error_body);

                Err(IOError::new(ErrorKind::Other, format!("Failed to create market order: {}", error_message)))
            }
        }
    }

    // // Cancel an order by orderId
    // pub async fn cancel_order(&self, symbol: &str, order_id: i64) -> Result<CancelOrderResponse, IOError> {
    //     let endpoint = "/v3/order";
    //     let timestamp = BinanceClient::generate_timestamp()?;
    //     let mut params = HashMap::new();
    //     params.insert("symbol", symbol);
    //     params.insert("orderId", &order_id.to_string());
    //     params.insert("timestamp", &timestamp.to_string());
    //
    //     // Convert params to a query string and sign it
    //     let query_string = self.query_string_from_params(&params);
    //     let signature = self.sign(&query_string);
    //
    //     let params = serde_qs::to_string(&order).map_err(|err| IOError::new(ErrorKind::InvalidInput, format!("Failed to serialize order: {}", err)))?;
    //
    //     let signature = self.binance_client.sign(&params);
    //     let full_params = format!("{}&signature={}", params, signature);
    //
    //     let url = format!("{}{}?{}&signature={}", self.binance_client.api_url, endpoint, query_string, signature);
    //
    //     let response = self.binance_client.get_client()
    //         .delete(&url)
    //         .header("X-MBX-APIKEY", self.binance_client.get_api_key().clone())
    //         .send()
    //         .await
    //         .map_err(|err| IOError::new(ErrorKind::Other, format!("HTTP request failed: {}", err)))?;
    //
    //     match response.status() {
    //         reqwest::StatusCode::OK => {
    //             response.json::<CancelOrderResponse>().await.map_err(|err| IOError::new(ErrorKind::Other, format!("Failed to deserialize response: {}", err)))
    //         },
    //         _ => {
    //             // Handle non-OK responses, possibly using response.text() to get more details
    //             let error_msg = response.text().await.unwrap_or_else(|_| "Failed to read error message".to_string());
    //             Err(IOError::new(ErrorKind::Other, format!("Failed to cancel order: {}", error_msg)))
    //         }
    //     }
    // }
}
