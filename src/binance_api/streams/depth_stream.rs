use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DepthMessage {
    stream: String,
    data: DepthData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DepthData {
    #[serde(rename = "e")]
    event_type: String,
    #[serde(rename = "E")]
    event_time: u64,
    #[serde(rename = "s")]
    symbol: String,
    #[serde(rename = "U")]
    first_update_id: u64,
    #[serde(rename = "u")]
    final_update_id: u64,
    #[serde(rename = "b")]
    bids: Vec<Bid>,
    #[serde(rename = "a")]
    asks: Vec<Ask>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bid {
    #[serde(rename = "p")]
    price: String,
    #[serde(rename = "q")]
    quantity: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ask {
    #[serde(rename = "p")]
    price: String,
    #[serde(rename = "q")]
    quantity: String,
}
