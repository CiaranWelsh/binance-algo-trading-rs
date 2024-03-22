use serde::{Deserialize, Serialize};
use crate::binance_client::binance_client::BinanceClient;
use crate::binance_client::order_types::order_type::OrderType;
use crate::binance_client::order_types::side::Side;
use crate::binance_client::order_types::time_in_force::TimeInForce;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopLimitOrder {
    symbol: String,
    side: Side,
    
    r#type: OrderType,
    quantity: f64,
    price: f64,
    
    stop_price: f64,
    timestamp: u64,
    
    time_in_force: TimeInForce,
}

impl StopLimitOrder {
    pub fn new(symbol: &str, side: Side, quantity: f64, price: f64, stop_price: f64, time_in_force: TimeInForce) -> Self {
        StopLimitOrder {
            symbol: symbol.to_string(),
            side,
            r#type: OrderType::StopLossLimit, // Or "TakeProfitLimit" depending on the use case
            quantity,
            price,
            stop_price,
            timestamp: BinanceClient::generate_timestamp().unwrap(),
            time_in_force, // Initialized here
        }
    }
}