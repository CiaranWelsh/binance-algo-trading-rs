use serde::{Deserialize, Serialize};
use crate::binance_client::order_types::time_in_force::TimeInForce;
use crate::binance_client::deserialization::deserialize_string_to_f64;

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenOrder {
    // This struct can be similar to `Order` with possibly fewer fields depending on the API's response
    symbol: String,
    #[serde(rename = "orderId")]
    order_id: i64,
    #[serde(rename = "clientOrderId")]
    client_order_id: String,
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    price: f64,
    #[serde(rename = "origQty", deserialize_with = "deserialize_string_to_f64")]
    orig_qty: f64,
    #[serde(rename = "executedQty", deserialize_with = "deserialize_string_to_f64")]
    executed_qty: f64,
    status: String,
    #[serde(rename = "timeInForce")]
    time_in_force: TimeInForce,
    #[serde(rename = "type")]
    r#type: String,
    side: String,
    #[serde(rename = "stopPrice", deserialize_with = "deserialize_string_to_f64")]
    stop_price: f64,
    #[serde(rename = "icebergQty", deserialize_with = "deserialize_string_to_f64")]
    iceberg_qty: f64,
    time: u64,
}
