use serde::{Deserialize, Serialize};
use crate::binance_client::account::order_status::OrderStatus;
use crate::binance_client::order_types::order_type::OrderType;
use crate::binance_client::order_types::side::Side;
use crate::binance_client::order_types::time_in_force::TimeInForce;
use crate::deserialization::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    symbol: String,
    #[serde(rename = "orderId")]
    order_id: i64,
    #[serde(rename = "orderListId")]
    order_list_id: i64,
    // Unless OCO, the value will be -1
    #[serde(rename = "clientOrderId")]
    client_order_id: String,
    #[serde(rename = "price", deserialize_with = "deserialize_string_to_f64")]
    price: f64,
    #[serde(rename = "origQty", deserialize_with = "deserialize_string_to_f64")]
    orig_qty: f64,
    #[serde(rename = "executedQty", deserialize_with = "deserialize_string_to_f64")]
    executed_qty: f64,
    #[serde(rename = "cummulativeQuoteQty", deserialize_with = "deserialize_string_to_f64")]
    cummulative_quote_qty: f64,
    status: OrderStatus,
    #[serde(rename = "timeInForce")]
    time_in_force: TimeInForce,
    #[serde(rename = "type")]
    order_type: OrderType,
    side: Side,
    #[serde(rename = "stopPrice", deserialize_with = "deserialize_string_to_f64")]
    stop_price: f64,
    #[serde(rename = "icebergQty", deserialize_with = "deserialize_string_to_f64")]
    iceberg_qty: f64,
    time: u64,
    #[serde(rename = "updateTime")]
    update_time: u64,
    #[serde(rename = "isWorking")]
    is_working: bool,
    #[serde(rename = "origQuoteOrderQty", deserialize_with = "deserialize_string_to_f64")]
    orig_quote_order_qty: f64,
    // Optional fields, use deserialize_with for optional numeric types if necessary
    #[serde(rename = "preventedMatchId", skip_serializing_if = "Option::is_none")]
    prevented_match_id: Option<i64>,
    #[serde(rename = "preventedQuantity", skip_serializing_if = "Option::is_none")]
    prevented_quantity: Option<f64>,
    #[serde(rename = "strategyId", skip_serializing_if = "Option::is_none")]
    strategy_id: Option<i64>,
    #[serde(rename = "strategyType", skip_serializing_if = "Option::is_none")]
    strategy_type: Option<i64>,
    #[serde(rename = "trailingDelta", skip_serializing_if = "Option::is_none")]
    trailing_delta: Option<u64>,
    #[serde(rename = "trailingTime", skip_serializing_if = "Option::is_none")]
    trailing_time: Option<i64>,
    #[serde(rename = "usedSor", skip_serializing_if = "Option::is_none")]
    used_sor: Option<bool>,
    #[serde(rename = "workingFloor", skip_serializing_if = "Option::is_none")]
    working_floor: Option<String>,
}
