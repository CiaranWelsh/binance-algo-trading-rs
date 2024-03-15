pub fn round(x: f64, decimals: u32) -> f64 {
    let y = 10i32.pow(decimals) as f64;
    ((x * y).round() / y)
}

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