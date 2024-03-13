use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StopLimitOrder {
    symbol: String,
    side: String,
    #[serde(rename = "type")]
    r#type: String,
    #[serde(rename = "timeInForce")]
    time_in_force: String,
    quantity: String,
    price: String,
    #[serde(rename = "stopPrice")]
    stop_price: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    recv_window: Option<u64>,
    timestamp: u64,
}

impl StopLimitOrder {
    // Convenience method for creating a new StopLimitOrder
    pub fn new(
        symbol: String,
        side: String,
        time_in_force: String,
        quantity: String,
        price: String,
        stop_price: String,
        recv_window: Option<u64>,
        timestamp: u64,
    ) -> Self {
        Self {
            symbol,
            side,
            r#type: "STOP_LOSS_LIMIT".to_string(), // Set type to STOP_LOSS_LIMIT
            time_in_force,
            quantity,
            price,
            stop_price,
            recv_window,
            timestamp,
        }
    }

    // Add any additional convenience methods or setters you might need
    // For example, a method to set the recvWindow if it wasn't set during construction
    pub fn set_recv_window(&mut self, recv_window: u64) {
        self.recv_window = Some(recv_window);
    }
}

// Example usage:
// let order = StopLimitOrder::new(
//     "BTCUSDT".to_string(),
//     "SELL".to_string(),
//     "GTC".to_string(),
//     "0.01".to_string(),
//     "9000".to_string(),
//     "9500".to_string(),
//     None, // or Some(recv_window_value)
//     current_timestamp,
// );
// order.set_recv_window(5000); // Optional
