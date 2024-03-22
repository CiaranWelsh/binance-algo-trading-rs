use serde::{Deserialize, Serialize};
use crate::binance_client::order_types::order_type::OrderType;
use crate::binance_client::order_types::side::Side;
use crate::binance_client::order_types::time_in_force::TimeInForce;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitOrder {
    symbol: String,
    side: Side,
    
    r#type: OrderType,
    
    time_in_force: String,
    quantity: f64,
    price: f64,
    timestamp: u64,
}

impl LimitOrder {
    pub fn new(symbol: &str, side: Side, quantity: f64, price: f64, timestamp: u64) -> Self {
        LimitOrder {
            symbol: symbol.to_string(),
            side,
            r#type: OrderType::Limit,
            time_in_force: TimeInForce::GTC.to_string(),
            quantity,
            price,
            timestamp,
        }
    }
}
