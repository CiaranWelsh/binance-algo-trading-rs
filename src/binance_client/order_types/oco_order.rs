use serde::{Deserialize, Serialize};
use crate::binance_client::order_types::side::Side;
use crate::binance_client::order_types::time_in_force::TimeInForce;

/// Represents an OCO (One Cancels the Other) order on Binance.
///
/// OCO orders consist of a combination of a limit order and a stop-limit order.
/// When either the limit or stop-limit order is executed, the other is automatically canceled.
///
/// # Fields
///
/// - `symbol`: The trading pair symbol.
/// - `side`: The order side (buy or sell).
/// - `quantity`: The order quantity.
/// - `price`: The limit order price.
/// - `stop_price`: The stop order trigger price.
/// - `stop_limit_price`: The price at which the stop-limit order turns into a limit order.
/// - `limit_iceberg_qty`: (Optional) The visible quantity of the limit order iceberg.
/// - `stop_iceberg_qty`: (Optional) The visible quantity of the stop-limit order iceberg.
/// - `stop_limit_time_in_force`: (Optional) Specifies how long the stop-limit order remains in effect.
/// - `new_order_resp_type`: (Optional) Sets the response type received after placing the order.
/// - `list_client_order_id`: (Optional) A unique ID for the entire OCO order list.
/// - `limit_client_order_id`: (Optional) A unique ID for the limit part of the OCO order.
/// - `stop_client_order_id`: (Optional) A unique ID for the stop-limit part of the OCO order.
/// - `recv_window`: (Optional) The number of milliseconds after `timestamp` the request is valid for.
/// - `timestamp`: The order timestamp.
///
/// # Example
///
/// ```
/// use binance_api::binance_client::order_types::oco_order::OcoOrder;
/// use binance_api::binance_client::order_types::side::Side;
/// let oco_order = OcoOrder::new(
///     "BTCUSDT".to_string(),
///     Side::Sell,
///     0.5,
///     9500.0,
///     9400.0,
///     9300.0,
///     1564645656565
/// );
/// ```
///
/// This creates a new OCO order for the `BTCUSDT` symbol, selling 0.5 BTC, with a limit order price of 9500 USDT,
/// a stop order trigger price of 9400 USDT, and a stop-limit price of 9300 USDT.
#[derive(Debug, Serialize, Deserialize)]
pub struct OcoOrder {
    pub symbol: String,
    pub side: Side,
    pub quantity: f64,
    pub price: f64,
    // Limit order price
    #[serde(rename = "stopPrice")]
    pub stop_price: f64,
    // Stop order price
    #[serde(rename = "stopLimitPrice")]
    pub stop_limit_price: f64,
    // Optional: Stop limit price, if different from stop price
    #[serde(rename = "limitIcebergQty")]
    pub limit_iceberg_qty: Option<f64>,
    // Optional: Used to make the limit order an iceberg order
    #[serde(rename = "stopIcebergQty")]
    pub stop_iceberg_qty: Option<f64>,
    // Optional: Used to make the stop limit order an iceberg order
    #[serde(rename = "stopLimitTimeInForce")]
    pub stop_limit_time_in_force: Option<TimeInForce>,
    // Optional: This defines how long the stop limit order will be active
    #[serde(rename = "newOrderRespType")]
    pub new_order_resp_type: Option<String>,
    // Optional: Set the response type received
    #[serde(rename = "listClientOrderId")]
    pub list_client_order_id: Option<String>,
    // Optional: A unique Id for the entire orderList
    #[serde(rename = "limitClientOrderId")]
    pub limit_client_order_id: Option<String>,
    // Optional: A unique Id for the limit order
    #[serde(rename = "stopClientOrderId")]
    pub stop_client_order_id: Option<String>,
    // Optional: A unique Id for the stop order
    pub recv_window: Option<u64>,
    // Added recv_window field
    pub timestamp: u64,
}

impl OcoOrder {
    /// Constructs a new `OcoOrder`.
    ///
    /// # Arguments
    ///
    /// - `symbol`: Trading pair symbol.
    /// - `side`: Buy or sell side of the order.
    /// - `quantity`: Quantity of the asset to order.
    /// - `price`: Price for the limit order.
    /// - `stop_price`: Trigger price for the stop-limit order.
    /// - `stop_limit_price`: Price at which the stop-limit order becomes a limit order.
    /// - `timestamp`: The current timestamp.
    ///
    /// # Returns
    ///
    /// Returns a new instance of `OcoOrder`.
    pub fn new(symbol: String, side: Side, quantity: f64, price: f64, stop_price: f64, stop_limit_price: f64, timestamp: u64) -> Self {
        Self {
            symbol,
            side,
            quantity,
            price,
            stop_price,
            stop_limit_price,
            limit_iceberg_qty: None,
            stop_iceberg_qty: None,
            stop_limit_time_in_force: None,
            new_order_resp_type: None,
            list_client_order_id: None,
            limit_client_order_id: None,
            stop_client_order_id: None,
            recv_window: None, // Initialize recv_window as None by default
            timestamp,
        }
    }
}
