use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BookTickerMessage {
    stream: String,
    data: BookTickerData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BookTickerData {
    #[serde(rename = "u")]
    update_id: u64, // Order book updateId
    #[serde(rename = "s")]
    symbol: String,  // Symbol
    #[serde(rename = "b")]
    best_bid_price: String,  // Best bid price
    #[serde(rename = "B")]
    best_bid_qty: String,  // Best bid quantity
    #[serde(rename = "a")]
    best_ask_price: String,  // Best ask price
    #[serde(rename = "A")]
    best_ask_qty: String,  // Best ask quantity
}
