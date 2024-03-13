use std::fmt;
use serde::{Deserialize, Serialize};
use crate::binance_api::order_types::order_type::OrderType;

// Define an enum for the order side
#[derive(Debug, Serialize, Deserialize)]
pub enum Side {
    BUY,
    SELL,
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}