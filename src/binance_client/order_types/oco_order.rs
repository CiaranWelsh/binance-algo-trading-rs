use serde::{Deserialize, Serialize};
use crate::binance_client::order_types::side::Side;

#[derive(Debug, Serialize, Deserialize)]
pub struct OcoOrder {
    symbol: String,
    side: Side,
    quantity: f64,
    price: f64,
    // Limit order price
    stop_price: f64,
    // Stop order price
    #[serde(rename = "stopLimitPrice")]
    stop_limit_price: Option<f64>,
    // Optional: Stop limit price, if different from stop price
    #[serde(rename = "listClientOrderId")]
    list_client_order_id: Option<String>,
    // Optional: A unique Id for the entire orderList
    #[serde(rename = "limitClientOrderId")]
    limit_client_order_id: Option<String>,
    // Optional: A unique Id for the limit order
    #[serde(rename = "stopClientOrderId")]
    stop_client_order_id: Option<String>,
    // Optional: A unique Id for the stop order
    // Other fields as required by Binance for OCO orders

    timestamp: u64,
}


impl OcoOrder {
    pub fn new(symbol: String, side: Side, quantity: f64, price: f64, stop_price: f64, timestamp: u64) -> Self {
        Self {
            symbol,
            side,
            quantity,
            price,
            stop_price,
            stop_limit_price: None,
            list_client_order_id: None,
            limit_client_order_id: None,
            stop_client_order_id: None,
            timestamp,
        }
    }
}
