use serde::Deserialize;

// Helper struct for deserializing Binance error responses
#[derive(Debug, Deserialize)]
pub struct BinanceError {
    pub code: i32,
    pub msg: String,
}