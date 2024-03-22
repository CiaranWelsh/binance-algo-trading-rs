use serde::{Deserialize, Serialize};
use crate::binance_client::account::order_status::OrderStatus;
use crate::binance_client::order_types::order_type::OrderType;
use crate::binance_client::order_types::side::Side;
use crate::binance_client::order_types::time_in_force::TimeInForce;
use crate::binance_client::deserialization::*;



#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub id: i64,
    
    pub order_id: i64,
    
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    pub price: f64,
    
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    pub qty: f64,
    
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    pub quote_qty: f64,
    
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    pub commission: f64,
    
    pub commission_asset: String,
    pub time: u64,
    
    pub is_buyer: bool,
    
    pub is_maker: bool,
    
    pub is_best_match: bool,
}
