use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenOrder {
    // Example fields based on the Binance API documentation
    symbol: String,

    orderId: i64,
    price: String,
    origQty: String,
    executedQty: String,
    status: String,
    timeInForce: String,
    r#type: String,
    side: String,
    // Add other fields as per the Binance API response
}
