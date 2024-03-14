use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TickerMessage {
    stream: String,
    data: TickerData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TickerData {
    #[serde(rename = "e")]
    event_type: String,  // Event type
    #[serde(rename = "E")]
    event_time: u64,  // Event time
    #[serde(rename = "s")]
    symbol: String,  // Symbol
    #[serde(rename = "p")]
    price_change: String,  // Price change
    #[serde(rename = "P")]
    price_change_percent: String,  // Price change percent
    #[serde(rename = "w")]
    weighted_avg_price: String,  // Weighted average price
    #[serde(rename = "x")]
    previous_close: String,  // Previous close price
    #[serde(rename = "c")]
    last_price: String,  // Last price
    #[serde(rename = "Q")]
    last_qty: String,  // Last quantity
    #[serde(rename = "b")]
    best_bid_price: String,  // Best bid price
    #[serde(rename = "B")]
    best_bid_qty: String,  // Best bid quantity
    #[serde(rename = "a")]
    best_ask_price: String,  // Best ask price
    #[serde(rename = "A")]
    best_ask_qty: String,  // Best ask quantity
    #[serde(rename = "o")]
    open_price: String,  // Open price
    #[serde(rename = "h")]
    high_price: String,  // High price
    #[serde(rename = "l")]
    low_price: String,  // Low price
    #[serde(rename = "v")]
    total_traded_base_asset_volume: String,  // Total traded base asset volume
    #[serde(rename = "q")]
    total_traded_quote_asset_volume: String,  // Total traded quote asset volume
    #[serde(rename = "O")]
    statistics_open_time: u64,  // Statistics open time
    #[serde(rename = "C")]
    statistics_close_time: u64,  // Statistics close time
    #[serde(rename = "F")]
    first_trade_id: u64,  // First trade ID
    #[serde(rename = "L")]
    last_trade_id: u64,  // Last trade ID
    #[serde(rename = "n")]
    total_number_of_trades: u64,  // Total number of trades
}

