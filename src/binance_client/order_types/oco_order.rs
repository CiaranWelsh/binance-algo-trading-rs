use serde::{Deserialize, Serialize};
use crate::binance_client::order_types::side::Side;
use crate::binance_client::order_types::time_in_force::TimeInForce;

#[derive(Debug, Serialize, Deserialize)]
pub struct OcoOrder {
    pub symbol: String,
    pub side: Side,
    pub quantity: f64,
    pub price: f64,
    // Limit order price
    #[serde(rename = "stopPrice")]
    pub stop_price: f64,
    // Stop order price
    #[serde(rename = "stopLimitPrice")]
    pub stop_limit_price: f64,
    // Optional: Stop limit price, if different from stop price
    #[serde(rename = "limitIcebergQty")]
    pub limit_iceberg_qty: Option<f64>,
    // Optional: Used to make the limit order an iceberg order
    #[serde(rename = "stopIcebergQty")]
    pub stop_iceberg_qty: Option<f64>,
    // Optional: Used to make the stop limit order an iceberg order
    #[serde(rename = "stopLimitTimeInForce")]
    pub stop_limit_time_in_force: Option<TimeInForce>,
    // Optional: This defines how long the stop limit order will be active
    #[serde(rename = "newOrderRespType")]
    pub new_order_resp_type: Option<String>,
    // Optional: Set the response type received
    #[serde(rename = "listClientOrderId")]
    pub list_client_order_id: Option<String>,
    // Optional: A unique Id for the entire orderList
    #[serde(rename = "limitClientOrderId")]
    pub limit_client_order_id: Option<String>,
    // Optional: A unique Id for the limit order
    #[serde(rename = "stopClientOrderId")]
    pub stop_client_order_id: Option<String>,
    // Optional: A unique Id for the stop order
    pub recv_window: Option<u64>, // Added recv_window field
    pub timestamp: u64,
}

impl OcoOrder {
    pub fn new(symbol: String, side: Side, quantity: f64, price: f64, stop_price: f64, stop_limit_price: f64, timestamp: u64) -> Self {
        Self {
            symbol,
            side,
            quantity,
            price,
            stop_price,
            stop_limit_price,
            limit_iceberg_qty: None,
            stop_iceberg_qty: None,
            stop_limit_time_in_force: None,
            new_order_resp_type: None,
            list_client_order_id: None,
            limit_client_order_id: None,
            stop_client_order_id: None,
            recv_window: None, // Initialize recv_window as None by default
            timestamp,
        }
    }
}
