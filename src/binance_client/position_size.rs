//! # Trading Position Size and Risk Management Example
//!
//! This module provides functions to calculate trading parameters such as position size,
//! stop loss, and take profit, based on various risk management strategies. These functions
//! allow traders to determine optimal trade sizes and risk levels to manage their trades effectively.
//!
//! ## Example Usage
//!
//! Given:
//! - An account size of $1000
//! - A willingness to risk 1% ($10) per trade
//! - An entry price, and stop loss and take profit specified in ticks
//!
//! The following example demonstrates how to calculate the stop loss price, take profit price,
//! and position size for a trade on the ETHUSDT pair, where the tick size is 0.01 (for simplicity),
//! 5 ticks for stop loss, and 50 ticks for take profit.
//!
//! ```rust
//! use crate::binance_api::binance_client::position_size::{round, calculate_ticks, calculate_position_size_in_base_asset};
//!
//! fn main() {
//!     // Define trading parameters
//!     let account_size = 1000.0;
//!     let risk_percentage = 1.0; // Willing to risk 1% of account per trade
//!     let entry_price = 500.0; // Assume an arbitrary entry price
//!     let tick_size = 0.01;
//!     let stop_loss_ticks = 5;
//!     let take_profit_ticks = 50;
//!
//!     // Calculate stop loss and take profit prices
//!     let (stop_loss_price, take_profit_price) = calculate_ticks(
//!         entry_price,
//!         tick_size,
//!         stop_loss_ticks,
//!         take_profit_ticks,
//!         true, // assuming a long position
//!     );
//!
//!     println!("Stop Loss Price: {}", stop_loss_price);
//!     println!("Take Profit Price: {}", take_profit_price);
//!
//!     // Calculate position size in units of the base asset
//!     let position_size = calculate_position_size_in_base_asset(
//!         account_size,
//!         risk_percentage,
//!         entry_price,
//!         stop_loss_price,
//!     );
//!
//!     println!("Position Size: {}", position_size);
//! }
//! ```
//!
//! This example shows how to integrate various functions to calculate key trading parameters
//! based on predefined risk management and trade setup criteria. By adjusting the ticks for stop loss
//! and take profit, traders can tailor their strategies according to different market conditions
//! and personal risk tolerance.
//! 
 //! ## Mathematical Relationships
//!
//! The calculations for position size, stop loss, and take profit are based on the following equations and relationships:
//!
//! ### Risk Amount
//! The risk amount is calculated as a percentage of the account size.
//! ```latex
//! Risk Amount = Account Size * (Risk Percentage / 100)
//! ```
//!
//! ### Stop Loss and Take Profit Prices
//! The stop loss and take profit prices are determined based on the entry price and the specified number of ticks.
//! ```latex
//! Stop Loss Price = Entry Price - (Stop Loss Ticks * Tick Size) for long positions
//! Stop Loss Price = Entry Price + (Stop Loss Ticks * Tick Size) for short positions
//! ```
//!
//! ```latex
//! Take Profit Price = Entry Price + (Take Profit Ticks * Tick Size) for long positions
//! Take Profit Price = Entry Price - (Take Profit Ticks * Tick Size) for short positions
//! ```
//!
//! ### Position Size
//! The position size in units of the base asset is calculated based on the risk amount, the entry price, and the stop loss price.
//! ```latex
//! Position Size = Risk Amount / (Entry Price - Stop Loss Price) / Entry Price
//! ```
//! This equation assumes a linear relationship between the risk amount and the position size, factoring in the price per unit of the base asset.
//!
//! ### Risk-Reward Ratio
//! The risk-reward ratio is the ratio of the potential reward to the potential risk of a trade.
//! ```latex
//! Risk-Reward Ratio = (Take Profit Price - Entry Price) / (Entry Price - Stop Loss Price) for long positions
//! Risk-Reward Ratio = (Entry Price - Take Profit Price) / (Stop Loss Price - Entry Price) for short positions
//! ```
//! This ratio helps traders to evaluate the potential profitability of a trade against its risk.
//!
//! ### Summary
//! These equations highlight the interconnectedness of trading parameters and underscore the importance of a disciplined approach to risk management. By carefully selecting these parameters, traders can optimize their trading strategy to achieve a balance between risk and reward.
//!
//! It is crucial to remember that market conditions, volatility, and other factors can influence the effectiveness of these calculations, and adjustments may be necessary to adapt to changing market dynamics.


/// Rounds a floating-point number to a specified number of decimal places.
///
/// # Arguments
///
/// * `x` - The floating-point number to round.
/// * `decimals` - The number of decimal places to round to.
///
/// # Returns
///
/// The number `x` rounded to `decimals` decimal places.
///
/// # Examples
///
/// ```
/// use binance_api::binance_client::position_size::round;
/// let rounded = round(3.14159, 2);
/// assert_eq!(rounded, 3.14);
/// ```
pub fn round(x: f64, decimals: u32) -> f64 {
    let y = 10i32.pow(decimals) as f64;
    (x * y).round() / y
}

/// Calculates the position size, take profit, and stop loss prices for a trade based on risk management parameters.
///
/// # Arguments
///
/// * `account_size` - The total size of the trading account.
/// * `risk_percentage` - The percentage of the account size willing to risk.
/// * `entry_price` - The entry price for the trade.
/// * `stop_loss_price` - The price at which the trade will be exited to limit losses.
/// * `take_profit_ratio` - The ratio of the take profit distance to the stop loss distance.
/// * `is_long` - A boolean indicating whether the trade is a long (true) or short (false).
///
/// # Returns
///
/// A Result containing a tuple of the position size, take profit price, and stop loss price if successful, or an error message if any input values are invalid.
///
/// # Examples
///
/// ```
/// use binance_api::binance_client::position_size::calculate_position_size;
/// let result = calculate_position_size(1000.0, 1.0, 100.0, 98.0, 2.0, true);
/// assert_eq!(result, Ok((50.0, 104.0, 98.0)));
/// ```
pub fn calculate_position_size(
    account_size: f64,
    risk_percentage: f64,
    entry_price: f64,
    stop_loss_price: f64,
    take_profit_ratio: f64,
    is_long: bool,
) -> Result<(f64, f64, f64), &'static str> {
    // Validation checks
    if account_size <= 0.0
        || risk_percentage <= 0.0
        || risk_percentage > 100.0
        || stop_loss_price <= 0.0
        || take_profit_ratio <= 0.0
    {
        return Err("Invalid input values for calculating position size.");
    }

    let risk_amount = account_size * (risk_percentage / 100.0);
    let stop_loss_distance = (entry_price - stop_loss_price).abs();
    let position_size = risk_amount / stop_loss_distance;

    let take_profit_distance = stop_loss_distance * take_profit_ratio;
    let take_profit_price = if is_long {
        entry_price + take_profit_distance
    } else {
        entry_price - take_profit_distance
    };

    Ok((position_size, take_profit_price, stop_loss_price))
}

/// Calculates the position size, take profit, and stop loss prices for a trade based on a percentage-based stop loss.
///
/// This function converts the stop loss percentage to a stop loss price and then calls `calculate_position_size`.
///
/// # Arguments
///
/// * `account_size` - The total size of the trading account.
/// * `risk_percentage` - The percentage of the account size willing to risk.
/// * `entry_price` - The entry price for the trade.
/// * `stop_loss_percentage` - The stop loss percentage from the entry price.
/// * `take_profit_ratio` - The ratio of the take profit distance to the stop loss distance.
/// * `is_long` - A boolean indicating whether the trade is a long (true) or short (false).
///
/// # Returns
///
/// A Result containing a tuple of the position size, take profit price, and stop loss price if successful, or an error message if any input values are invalid.
///
/// # Examples
///
/// ```
/// use binance_api::binance_client::position_size::calculate_position_size_stop_loss_as_percentage;
/// let result = calculate_position_size_stop_loss_as_percentage(1000.0, 1.0, 100.0, 2.0, 5.0, true);
/// assert!(result.is_ok());
/// ```
pub fn calculate_position_size_stop_loss_as_percentage(
    account_size: f64,
    risk_percentage: f64,
    entry_price: f64,
    stop_loss_percentage: f64,
    take_profit_ratio: f64,
    is_long: bool,
) -> Result<(f64, f64, f64), &'static str> {
    // Conversion of stop loss percentage to stop loss price
    let stop_loss_price = if is_long {
        entry_price * (1.0 - (stop_loss_percentage / 100.0))
    } else {
        entry_price * (1.0 + (stop_loss_percentage / 100.0))
    };

    // Now, call the original function with the calculated stop loss price
    calculate_position_size(
        account_size,
        risk_percentage,
        entry_price,
        stop_loss_price,
        take_profit_ratio,
        is_long,
    )
}

/// Calculates stop loss and take profit prices based on ticks.
///
/// # Arguments
///
/// * `entry_price` - The entry price of the trade.
/// * `tick_size` - The minimum price movement of the trading instrument.
/// * `stop_loss_ticks` - The number of ticks to set the stop loss from the entry price.
/// * `take_profit_ticks` - The number of ticks to set the take profit from the entry price.
/// * `is_long` - A boolean indicating whether the trade is a long (true) or short (false).
///
/// # Returns
///
/// A tuple containing the calculated stop loss price and take profit price.
///
/// # Examples
///
/// ```
/// use binance_api::binance_client::position_size::calculate_ticks;
/// let (stop_loss_price, take_profit_price) = calculate_ticks(1.3000, 0.0001, 50, 150, true);
/// assert_eq!(stop_loss_price, 1.2950);
/// assert_eq!(take_profit_price, 1.3150);
/// ```
pub fn calculate_ticks(
    entry_price: f64,
    tick_size: f64,
    stop_loss_ticks: i32,
    take_profit_ticks: i32,
    is_long: bool,
) -> (f64, f64) {
    let stop_loss_adjustment = tick_size * stop_loss_ticks as f64;
    let take_profit_adjustment = tick_size * take_profit_ticks as f64;

    let stop_loss_price = if is_long {
        entry_price - stop_loss_adjustment
    } else {
        entry_price + stop_loss_adjustment
    };

    let take_profit_price = if is_long {
        entry_price + take_profit_adjustment
    } else {
        entry_price - take_profit_adjustment
    };

    (stop_loss_price, take_profit_price)
}


/// Calculates the position size in units of the base asset based on defined risk parameters.
///
/// # Arguments
///
/// * `account_size` - The total capital available in the trading account.
/// * `risk_percentage` - The percentage of the account size you are willing to risk on this trade.
/// * `entry_price` - The price at which you plan to enter the trade.
/// * `stop_loss_price` - The price at which you plan to exit the trade if it goes against you.
///
/// # Returns
///
/// The position size in units of the base asset that you should buy/sell.
///
/// # Examples
///
/// ```
/// use binance_api::binance_client::position_size::calculate_position_size_in_base_asset;
/// let position_size = calculate_position_size_in_base_asset(10000.0, 1.0, 500.0, 480.0);
/// println!("Position size: {}", position_size);
/// ```
pub fn calculate_position_size_in_base_asset(
    account_size: f64,
    risk_percentage: f64,
    entry_price: f64,
    stop_loss_price: f64,
) -> f64 {
    let risk_amount = account_size * (risk_percentage / 100.0);
    let risk_per_unit = (entry_price - stop_loss_price).abs();
    let position_size = risk_amount / risk_per_unit / entry_price;

    position_size
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_long_position_with_take_profit() {
        let (position_size, take_profit_price, _stop_loss_price) =
            calculate_position_size_stop_loss_as_percentage(1000.0, 1.0, 100.0, 2.0, 5.0, true).unwrap();
        assert!((position_size - 5.0).abs() < 1e-4, "Long position size calculation failed.");
        assert!((take_profit_price - 110.0).abs() < 1e-2, "Long position take profit calculation failed.");
    }

    #[test]
    fn test_short_position_with_take_profit() {
        let (position_size, take_profit_price, _stop_loss_price) =
            calculate_position_size_stop_loss_as_percentage(1000.0, 1.0, 100.0, 2.0, 5.0, false).unwrap();
        assert!((position_size - 5.0).abs() < 1e-4, "Short position size calculation failed.");
        assert!((take_profit_price - 90.0).abs() < 1e-2, "Short position take profit calculation failed.");
    }

    #[test]
    #[should_panic(expected = "Invalid input values for calculating position size.")]
    fn test_invalid_take_profit_ratio() {
        calculate_position_size_stop_loss_as_percentage(1000.0, 1.0, 100.0, 2.0, 0.0, true).unwrap();
    }

    #[test]
    fn test_high_take_profit_ratio() {
        let (position_size, take_profit_price, _stop_loss_price) =
            calculate_position_size_stop_loss_as_percentage(1000.0, 1.0, 100.0, 2.0, 10.0, true).unwrap();
        assert!((position_size - 5.0).abs() < 1e-4, "Position size calculation with high take profit ratio failed.");
        assert!((take_profit_price - 120.0).abs() < 1e-2, "Take profit calculation with high ratio failed.");
    }

    #[test]
    fn test_extremely_low_entry_price() {
        let (position_size, take_profit_price, _stop_loss_price) =
            calculate_position_size_stop_loss_as_percentage(1000.0, 1.0, 0.01, 2.0, 5.0, true).unwrap();
        assert!((position_size - 50000.0).abs() < 1e-4, "Position size calculation failed for low entry price.");
        assert!((take_profit_price - 0.011).abs() < 1e-5, "Take profit calculation failed for low entry price.");
    }

    #[test]
    fn test_small_stop_loss_percentage() {
        let (position_size, take_profit_price, _stop_loss_price) =
            calculate_position_size_stop_loss_as_percentage(1000.0, 1.0, 100.0, 0.01, 5.0, true).unwrap();
        assert!((position_size - 1000.0).abs() < 1e-4, "Position size calculation failed for small stop loss percentage.");
        assert!((take_profit_price - 100.05).abs() < 1e-2, "Take profit calculation failed for small stop loss percentage.");
    }

    #[test]
    fn test_large_stop_loss_percentage() {
        let (position_size, take_profit_price, _stop_loss_price) =
            calculate_position_size_stop_loss_as_percentage(1000.0, 1.0, 100.0, 50.0, 5.0, true).unwrap();
        assert!((position_size - 0.2).abs() < 1e-4, "Position size calculation failed for large stop loss percentage.");
        assert!((take_profit_price - 350.0).abs() < 1e-2, "Take profit calculation failed for large stop loss percentage.");
    }

    #[test]
    #[should_panic(expected = "Invalid input values for calculating position size.")]
    fn test_negative_account_size() {
        calculate_position_size_stop_loss_as_percentage(-1000.0, 0.01, 100.0, 0.02, 5.0, true).unwrap();
    }

    #[test]
    fn test_long_position_with_direct_stop_loss() {
        let (position_size, take_profit_price, calculated_stop_loss_price) =
            calculate_position_size(1000.0, 1.0, 100.0, 98.0, 5.0, true).unwrap();
        assert!((position_size - 5.0).abs() < 1e-4, "Long position size calculation failed.");
        assert!((take_profit_price - 110.0).abs() < 1e-2, "Long position take profit calculation failed.");
        assert!((calculated_stop_loss_price - 98.0).abs() < 1e-2, "Stop loss price mismatch.");
    }

    #[test]
    fn test_short_position_with_direct_stop_loss() {
        let (position_size, take_profit_price, calculated_stop_loss_price) =
            calculate_position_size(1000.0, 1.0, 100.0, 102.0, 5.0, false).unwrap();
        assert!((position_size - 5.0).abs() < 1e-4, "Short position size calculation failed.");
        assert!((take_profit_price - 90.0).abs() < 1e-2, "Short position take profit calculation failed.");
        assert!((calculated_stop_loss_price - 102.0).abs() < 1e-2, "Stop loss price mismatch.");
    }
}