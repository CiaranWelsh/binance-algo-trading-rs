use serde::Deserialize;
use crate::binance_client::deserialization::deserialize_string_to_f64;
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerPrice {
    pub symbol: String,
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    pub price: f64,
}