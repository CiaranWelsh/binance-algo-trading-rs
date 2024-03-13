use serde::{Deserialize, Serialize};
use crate::binance_api::binance_api::BinanceAPI;
use crate::binance_api::order_types::side::Side;
use crate::binance_api::order_types::time_in_force::TimeInForce;

#[derive(Debug, Serialize, Deserialize)]
pub struct StopLimitOrder {
    symbol: String,
    side: Side,
    #[serde(rename = "type")]
    r#type: String,
    quantity: f64,
    price: f64,
    #[serde(rename = "stopPrice")]
    stop_price: f64,
    timestamp: u64,
    #[serde(rename = "timeInForce")]
    time_in_force: TimeInForce,
}

impl StopLimitOrder {
    pub fn new(symbol: String, side: Side, quantity: f64, price: f64, stop_price: f64, time_in_force: TimeInForce) -> Self {
        StopLimitOrder {
            symbol,
            side,
            r#type: "STOP_LOSS_LIMIT".to_string(), // Or "TAKE_PROFIT_LIMIT" depending on the use case
            quantity,
            price,
            stop_price,
            timestamp: BinanceAPI::generate_timestamp().unwrap(),
            time_in_force, // Initialized here
        }
    }
}