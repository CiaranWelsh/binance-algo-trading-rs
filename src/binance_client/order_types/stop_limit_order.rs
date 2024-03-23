use serde::{Deserialize, Serialize};
use crate::binance_client::binance_client::BinanceClient;
use crate::binance_client::order_types::order_type::OrderType;
use crate::binance_client::order_types::side::Side;
use crate::binance_client::order_types::time_in_force::TimeInForce;


/// The StopLimitOrder struct represents a stop-limit order on the Binance exchange, defined within 
/// a Rust trading bot or application that interfaces with the Binance API. Stop-limit orders are 
/// advanced order types that combine the features of stop orders and limit orders. These orders 
/// can be used to specify a price limit at which the order should be executed after a given stop 
/// price has been reached, providing traders with greater control over their entry and exit 
/// points in the markets.
/// 
/// Fields
/// 
/// symbol: The trading pair symbol (e.g., "BTCUSDT").
/// side: The order side, either Side::Buy or Side::Sell, indicating whether the order is to buy or sell.
/// type: The order type, automatically set to OrderType::StopLossLimit for stop-limit orders.
/// quantity: The amount of the asset to buy or sell.
/// price: The limit price at which the order should be executed once the stop price is reached.
/// stop_price: The price at which the stop-limit order becomes active and places a limit order at the specified limit price.
/// timestamp: A timestamp for the order, typically generated at the time of order creation.
/// time_in_force: Specifies how long the order will remain active. Common values include GTC (Good Till Cancelled).
/// 
/// Buy Stop-Limit Order
/// A buy stop-limit order is placed above the current market price and consists of a stop price 
/// and a limit price higher than the stop price. This order type is used to protect against 
/// breakout scenarios or to enter the market at a predefined price level.
/// 
/// Example:
/// 
/// To place a buy stop-limit order for BTCUSDT at a stop price of $20,000 and a limit price of $20,100:
/// ```rust
/// use binance_api::binance_client::order_types::side::Side;
/// use binance_api::binance_client::order_types::stop_limit_order::StopLimitOrder;
/// use binance_api::binance_client::order_types::time_in_force::TimeInForce;
/// let buy_stop_limit_order = StopLimitOrder::new(
///     "BTCUSDT",
///     Side::Buy,
///     0.1 , // limit price
///     20000.0 , // quantity in BTC
///     20100.0 , // stop price
///     TimeInForce::GTC,
/// );
///```
/// Sell Stop-Limit Order
/// A sell stop-limit order is placed below the current market price and consists of a stop price 
/// and a limit price lower than the stop price. This order type is commonly used to limit potential
/// losses on a position or to exit the market in a controlled manner after a certain price level 
/// is breached.
/// 
/// Example:
/// 
/// To place a sell stop-limit order for BTCUSDT at a stop 
/// price of $18,000 and a limit price of $17,900:
/// ```rust
/// use binance_api::binance_client::order_types::side::Side;
/// use binance_api::binance_client::order_types::stop_limit_order::StopLimitOrder;
/// use binance_api::binance_client::order_types::time_in_force::TimeInForce;
/// let sell_stop_limit_order = StopLimitOrder::new(
///     "BTCUSDT",
///     Side::Sell,
///     0.1 , // limit price
///     18000.0 , // quantity in BTC
///     17900.0 , // stop price
///     TimeInForce::GTC,
/// );
///```
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopLimitOrder {
    symbol: String,
    side: Side,
    r#type: OrderType,
    quantity: f64,
    price: f64,
    stop_price: f64,
    timestamp: u64,
    time_in_force: TimeInForce,
}

impl StopLimitOrder {
    pub fn new(symbol: &str, side: Side, quantity: f64, stop_price: f64, price: f64, time_in_force: TimeInForce) -> Self {
        StopLimitOrder {
            symbol: symbol.to_string(),
            side,
            r#type: OrderType::StopLossLimit, // Or "TakeProfitLimit" depending on the use case
            quantity,
            price,
            stop_price,
            timestamp: BinanceClient::generate_timestamp().unwrap(),
            time_in_force, // Initialized here
        }
    }
}