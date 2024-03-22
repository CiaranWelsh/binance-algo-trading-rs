use serde::{Deserialize, Serialize};
use crate::binance_client::account::order_status::OrderStatus;
use crate::binance_client::order_types::order_type::OrderType;
use crate::binance_client::order_types::side::Side;
use crate::binance_client::order_types::time_in_force::TimeInForce;
use crate::binance_client::deserialization::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub symbol: String,
    
    pub order_id: i64,
    
    pub order_list_id: i64,
    // Unless OCO, the value will be -1
    
    pub client_order_id: String,
    
    pub price: f64,
    
    pub orig_qty: f64,
    
    pub executed_qty: f64,
    
    cummulative_quote_qty: f64,
    pub status: OrderStatus,
    
    pub time_in_force: TimeInForce,
    
    order_type: OrderType,
    pub side: Side,
    
    pub stop_price: f64,
    
    iceberg_qty: f64,
    pub time: u64,
    
    pub update_time: u64,
    
    pub is_working: bool,
    
    pub orig_quote_order_qty: f64,
    // Optional fields, use deserialize_with for optional numeric types if necessary
    
    pub prevented_match_id: Option<i64>,
    
    pub prevented_quantity: Option<f64>,
    
    pub strategy_id: Option<i64>,
    
    pub strategy_type: Option<i64>,
    
    pub trailing_delta: Option<u64>,
    
    pub trailing_time: Option<i64>,
    
    pub used_sor: Option<bool>,
    
    pub working_floor: Option<String>,
}
