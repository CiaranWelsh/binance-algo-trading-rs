use std::collections::HashMap;
use reqwest::{Client, Response, StatusCode};
use serde::{Deserialize, Serialize};
use std::io::{Error as IOError, ErrorKind};
use log::trace;
use serde_json::{json, Value};
use crate::binance_client::binance_client::BinanceClient;
use crate::binance_client::order_response::OrderResponse;
use crate::binance_client::order_types::cancel_order_response::CancelOrderResponse;
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
    
    pub async fn create_limit_order(&self, order: LimitOrder) -> Result<OrderResponse, IOError> {
        let endpoint = "/v3/order";
        let url = format!("{}{}", self.binance_client.api_url, endpoint);
        let params = serde_qs::to_string(&order).map_err(|err| IOError::new(ErrorKind::InvalidInput, err.to_string()))?;

        let signature = self.binance_client.sign(&params);
        let full_params = format!("{}&signature={}", params, signature);
        let response = self.send_request(url, full_params).await;
        Self::parse_order_response(response?).await
    }


    pub async fn create_stop_limit_order(&self, order: StopLimitOrder) -> Result<OrderResponse, IOError> {
        let endpoint = "/v3/order";
        let url = format!("{}{}", self.binance_client.api_url, endpoint);
        let params = serde_qs::to_string(&order)
            .map_err(|err| IOError::new(ErrorKind::InvalidInput, format!("Failed to serialize order: {}", err)))?;


        let signature = self.binance_client.sign(&params);
        let full_params = format!("{}&signature={}", params, signature);
        trace!("full params: {:?}", full_params);
        let response = self.send_request(url, full_params).await;
        Self::parse_order_response(response?).await
    }


    pub async fn create_oco_order(&self, order: OcoOrder) -> Result<OrderResponse, IOError> {
        let endpoint = "/v3/order/oco";
        let url = format!("{}{}", self.binance_client.api_url, endpoint);
        let params = serde_qs::to_string(&order).map_err(|err| IOError::new(ErrorKind::InvalidInput, format!("Failed to serialize order: {}", err)))?;

        let signature = self.binance_client.sign(&params);
        let full_params = format!("{}&signature={}", params, signature);

        let response = self.send_request(url, full_params).await;
        Self::parse_order_response(response?).await
    }


    pub async fn create_market_order(&self, order: MarketOrder) -> Result<OrderResponse, IOError> {
        let endpoint = "/v3/order";
        let url = format!("{}{}", self.binance_client.api_url, endpoint);
        let params = serde_qs::to_string(&order).map_err(|err| IOError::new(ErrorKind::InvalidInput, format!("Failed to serialize order: {}", err)))?;

        let signature = self.binance_client.sign(&params);
        let full_params = format!("{}&signature={}", params, signature);
        let response = self.send_request(url, full_params).await;
        Self::parse_order_response(response?).await
    }

    // Function to cancel an order given its ID and symbol
    pub async fn cancel_order(&self, symbol: &str, order_id: i64) -> Result<CancelOrderResponse, IOError> {
        let endpoint = "/v3/order";

        let timestamp = BinanceClient::generate_timestamp().unwrap();
        let query_string = format!(
            "orderId={}&symbol={}&timestamp={}",
            order_id, symbol, timestamp
        );

        let signature = self.binance_client.sign(&query_string);

        let url = format!(
            "{}{}?{}&signature={}",
            self.binance_client.api_url, endpoint, query_string, signature
        );

        let response = self.binance_client.get_client()
            .delete(&url)
            .header("X-MBX-APIKEY", self.binance_client.get_api_key())
            .send()
            .await
            .map_err(|err| IOError::new(ErrorKind::Other, format!("HTTP request failed: {}", err)))?;

        match response.status() {
            StatusCode::OK => {
                let body = response.text().await.map_err(|_| IOError::new(ErrorKind::Other, "Failed to read response body"))?;
                trace!("Cancel order response: {:?}", body);
                let cancel_order_response : CancelOrderResponse = serde_json::from_str(&body)?;
                Ok(cancel_order_response)
            }
            _ => {
                Err(Self::parse_error(response, "Failed to cancel order").await)
            }
        }
    }


    async fn parse_order_response(response: Response) -> Result<OrderResponse, IOError> {
        match response.status() {
            StatusCode::OK => {
                let body = response.text().await.map_err(|_| IOError::new(ErrorKind::Other, "Failed to read response body"))?;
                trace!("body: {:?}", body);

                let order_response: OrderResponse = serde_json::from_str(body.as_str())?;
                trace!("order response: {:?}", order_response);
                Ok(order_response)
            }
            _ => {
                Err(Self::parse_error(response, "Failed to place order").await)
            }
        }
    }

    async fn send_request(&self, url: String, params: String) -> Result<Response, IOError> {
        let resp = self.binance_client.get_client()
            .post(&url)
            .header("X-MBX-APIKEY", self.binance_client.get_api_key())
            .body(params)
            .send()
            .await
            .map_err(|err| IOError::new(ErrorKind::Other, format!("HTTP request failed: {}", err)))?;
        Ok(resp)
    }

    async fn parse_error(response: Response, error_message: &str) -> IOError  {
        let error_body = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        let response_error_message = serde_json::from_str::<Value>(&error_body)
            .ok()
            .and_then(|v| v["msg"].as_str().map(ToString::to_string))
            .unwrap_or(error_body);

        IOError::new(ErrorKind::Other, format!("{}: {}", error_message,  response_error_message))
    }

}
