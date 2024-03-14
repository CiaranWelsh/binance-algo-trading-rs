use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MiniTickerMessage {
    stream: String,
    data: MiniTickerData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MiniTickerData {
    #[serde(rename = "e")]
    event_type: String,  // Event type
    #[serde(rename = "E")]
    event_time: u64,  // Event time
    #[serde(rename = "s")]
    symbol: String,  // Symbol
    #[serde(rename = "c")]
    close_price: String,  // Close price
    #[serde(rename = "o")]
    open_price: String,  // Open price
    #[serde(rename = "l")]
    low_price: String,  // Low price
    #[serde(rename = "h")]
    high_price: String,  // High price
    #[serde(rename = "v")]
    total_traded_base_asset_volume: String,  // Total traded base asset volume
    #[serde(rename = "q")]
    total_traded_quote_asset_volume: String,  // Total traded quote asset volume
}
