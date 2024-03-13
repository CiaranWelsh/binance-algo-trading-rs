use serde::{Deserialize, Serialize};
use crate::binance_api::order_types::order_type::OrderType;
use crate::binance_api::order_types::side::Side;
use crate::binance_api::order_types::time_in_force::TimeInForce;

#[derive(Debug, Serialize, Deserialize)]
pub struct LimitOrder {
    symbol: String,
    side: Side,
    #[serde(rename = "type")]
    r#type: OrderType,
    #[serde(rename = "timeInForce")]
    time_in_force: String,
    quantity: f64,
    price: f64,
    timestamp: u64,
}

impl LimitOrder {
    pub fn new(symbol: String, side: Side, quantity: f64, price: f64, timestamp: u64) -> Self {
        LimitOrder {
            symbol,
            side,
            r#type: OrderType::Limit,
            time_in_force: TimeInForce::GTC.to_string(),
            quantity,
            price,
            timestamp,
        }
    }
}
