use serde::{Deserialize, Serialize};
use crate::binance_client::order_types::time_in_force::TimeInForce;
use crate::binance_client::deserialization::deserialize_string_to_f64;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenOrder {
    // This struct can be similar to `Order` with possibly fewer fields depending on the API's response
    symbol: String,
    
    order_id: i64,
    
    client_order_id: String,
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    price: f64,
    
    orig_qty: f64,
    
    executed_qty: f64,
    status: String,
    
    time_in_force: TimeInForce,
    
    r#type: String,
    side: String,
    
    stop_price: f64,
    
    iceberg_qty: f64,
    time: u64,
}
