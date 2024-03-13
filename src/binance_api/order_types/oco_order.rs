use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OcoOrder {
    symbol: String,
    side: String,
    quantity: String,
    price: String, // Limit order price
    stop_price: String, // Stop order price
    stop_limit_price: Option<String>, // Optional: Stop limit price, if different from stop price
    list_client_order_id: Option<String>, // Optional: A unique Id for the entire orderList
    limit_client_order_id: Option<String>, // Optional: A unique Id for the limit order
    stop_client_order_id: Option<String>, // Optional: A unique Id for the stop order
    // Other fields as required by Binance for OCO orders
    timestamp: u64,
}

impl OcoOrder {
    pub fn new(symbol: String, side: String, quantity: String, price: String, stop_price: String, timestamp: u64) -> Self {
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
