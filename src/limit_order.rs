use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LimitOrder {
    symbol: String,
    side: String,
    r#type: String,
    time_in_force: String,
    quantity: f64,
    price: f64,
}

impl LimitOrder {
    pub fn new(symbol: String, side: String, quantity: f64, price: f64) -> Self {
        LimitOrder {
            symbol,
            side,
            r#type: "LIMIT".to_string(),
            time_in_force: "GTC".to_string(), // Good Till Cancel
            quantity,
            price,
        }
    }
}
