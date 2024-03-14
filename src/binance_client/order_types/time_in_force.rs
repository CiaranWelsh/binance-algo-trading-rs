use std::fmt;
use serde::{Deserialize, Serialize};
use crate::binance_client::order_types::order_type::OrderType;

/// Defines the time frame in which an order will remain valid before it is executed or expires.
///
/// These options provide traders with additional control over the timing of their trades
/// and can be critical for strategy implementation, especially in fast-moving markets.
#[derive(Debug, Serialize, Deserialize)]
pub enum TimeInForce {
    /// Good Till Cancel (GTC) orders remain active until they are executed or manually canceled by the trader.
    /// GTC orders do not expire unless filled or canceled, providing a way to place long-term orders.
    GTC,

    /// Immediate Or Cancel (IOC) orders must be executed immediately. Any portion of the order that cannot be
    /// filled immediately will be canceled, reducing the risk of partial fills. It's used when execution speed is essential.
    IOC,

    /// Fill Or Kill (FOK) orders are similar to IOC orders, but they require the entire order to be filled immediately.
    /// If the full order cannot be filled, it will be entirely canceled. FOK orders are used to avoid partial fills
    /// at the expense of not executing the order at all if the full quantity is not available.
    FOK,

    /// Good Till Crossing (GTX), also known as Post Only, orders will only execute if they do not immediately match
    /// and fill. They are used primarily by market makers looking to add liquidity to the order book, ensuring the order
    /// provides liquidity instead of taking it.
    GTX,
}


impl fmt::Display for TimeInForce {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}