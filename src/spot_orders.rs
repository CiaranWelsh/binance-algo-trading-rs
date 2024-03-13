use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::io::{Error as IOError, ErrorKind};
use crate::binance_api::BinanceAPI;
use crate::limit_order::LimitOrder;
use crate::oco_order::OcoOrder;
use crate::stop_limit_order::StopLimitOrder;

pub struct SpotOrders<'a> {
    api: &'a BinanceAPI,
}

impl SpotOrders<'_> {
    pub fn new(api: &BinanceAPI) -> SpotOrders {
        SpotOrders { api }
    }

    pub async fn create_limit_order(&self, order: LimitOrder) -> Result<(), IOError> {
        let endpoint = "/v3/order";
        let url = format!("{}{}", self.api.api_url, endpoint);
        let params = serde_qs::to_string(&order).map_err(|err| IOError::new(ErrorKind::InvalidInput, err.to_string()))?;

        let signature = self.api.sign(&params);
        let full_params = format!("{}&signature={}", params, signature);

        let response = self.api.get_client()
            .post(&url)
            .header("X-MBX-APIKEY", self.api.get_api_key())
            .body(full_params)
            .send()
            .await
            .map_err(|err| IOError::new(ErrorKind::Other, err.to_string()))?;

        match response.status().is_success() {
            true => Ok(()),
            false => Err(IOError::new(ErrorKind::Other, "Failed to create limit order")),
        }
    }

    pub async fn create_stop_limit_order(&self, order: StopLimitOrder) -> Result<(), IOError> {
        let endpoint = "/v3/order";
        let url = format!("{}{}", self.api.api_url, endpoint);
        // Serialize `StopLimitOrder` instead of `LimitOrder`
        let params = serde_qs::to_string(&order).map_err(|err| IOError::new(ErrorKind::InvalidInput, err.to_string()))?;

        let signature = self.api.sign(&params);
        let full_params = format!("{}&signature={}", params, signature);

        let response = self.api.get_client()
            .post(&url)
            .header("X-MBX-APIKEY", self.api.get_api_key())
            .body(full_params)
            .send()
            .await
            .map_err(|err| IOError::new(ErrorKind::Other, err.to_string()))?;

        match response.status().is_success() {
            true => Ok(()),
            false => Err(IOError::new(ErrorKind::Other, "Failed to create stop-limit order")),
        }
    }


    pub async fn create_oco_order(&self, order: OcoOrder) -> Result<(), IOError> {
        let endpoint = "/api/v3/order/oco"; // Endpoint for OCO orders
        let url = format!("{}{}", self.api.api_url, endpoint);
        let params = serde_qs::to_string(&order)
            .map_err(|err| IOError::new(ErrorKind::InvalidInput, format!("Failed to serialize order: {}", err)))?;

        let signature = self.api.sign(&params);
        let full_params = format!("{}&signature={}", params, signature);

        let response = self.api.get_client()
            .post(&url)
            .header("X-MBX-APIKEY", self.api.get_api_key())
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(full_params)
            .send()
            .await
            .map_err(|err| IOError::new(ErrorKind::Other, format!("HTTP request failed: {}", err)))?;

        match response.status().is_success() {
            true => Ok(()),
            false => {
                let error_text = response.text().await.unwrap_or_else(|_| "Failed to parse error response".into());
                Err(IOError::new(ErrorKind::Other, format!("Failed to create OCO order: {}", error_text)))
            }
        }
    }
}
