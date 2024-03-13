use std::fmt;
use serde::{Deserialize, Serialize};

/// Represents the type of order to be placed on the Binance exchange.
///
/// Each variant corresponds to a specific order type supported by Binance,
/// defining how the order will be executed by the matching engine.
#[derive(Debug, Serialize, Deserialize)]
pub enum OrderType {
    /// A limit order is an order to buy or sell at a specified price or better.
    /// This order type ensures that the order will only be executed at the
    /// specified price or a more favorable one.
    LIMIT,

    /// A market order is an order to buy or sell immediately at the best available
    /// current price. It prioritizes speed over price and may be executed at a price
    /// significantly different from the last traded price, especially in volatile markets.
    MARKET,

    /// A stop loss order places a market sell order if the asset's price dips to a certain level.
    /// It is used to limit an investor's loss on a position.
    STOP_LOSS,

    /// Similar to a stop loss order, but it specifies the price of the limit sell order
    /// placed when the stop price is reached. It combines the features of stop and limit orders.
    STOP_LOSS_LIMIT,

    /// A take profit order places a market buy order if the asset's price rises to a certain level.
    /// It is used to secure profits.
    TAKE_PROFIT,

    /// Similar to a take profit order, but it specifies the price of the limit buy order
    /// placed when the stop price is reached. It allows traders to set a precise profit target.
    TAKE_PROFIT_LIMIT,

    /// A limit maker order is a limit order that won't be executed if it would immediately
    /// match and fill. It's used to ensure the order will be added to the order book,
    /// adding liquidity instead of taking it.
    LIMIT_MAKER,

    /// An OCO, or One Cancels the Other order, is a pair of orders stipulating that if one
    /// order executes, then the other order is automatically canceled. An OCO order combines
    /// a stop order with a limit order on the same asset.
    OCO,
}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}