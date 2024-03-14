use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct klineMessage {
    stream: String,
    data: KlineData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KlineData {
    #[serde(rename = "e")]
    event_type: String,
    #[serde(rename = "E")]
    event_time: u64,
    #[serde(rename = "s")]
    symbol: String,
    k: Kline,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Kline {
    #[serde(rename = "t")]
    start_time: u64,
    #[serde(rename = "T")]
    end_time: u64,
    #[serde(rename = "s")]
    symbol: String,
    #[serde(rename = "i")]
    interval: String,
    #[serde(rename = "f")]
    first_trade_id: u64,
    #[serde(rename = "L")]
    last_trade_id: u64,
    #[serde(rename = "o")]
    open_price: String,
    #[serde(rename = "c")]
    close_price: String,
    #[serde(rename = "h")]
    high_price: String,
    #[serde(rename = "l")]
    low_price: String,
    #[serde(rename = "v")]
    base_asset_volume: String,
    #[serde(rename = "n")]
    number_of_trades: u32,
    #[serde(rename = "x")]
    is_kline_closed: bool,
    #[serde(rename = "q")]
    quote_asset_volume: String,
    #[serde(rename = "V")]
    taker_buy_base_asset_volume: String,
    #[serde(rename = "Q")]
    taker_buy_quote_asset_volume: String,
    #[serde(rename = "B")]
    ignore: String,
}