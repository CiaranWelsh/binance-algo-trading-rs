use serde::{Deserialize, Serialize};
use crate::binance_client::order_types::side::Side;
use crate::binance_client::binance_client::BinanceClient;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarketOrder {
    symbol: String,
    side: Side,
    
    r#type: String,
    quantity: Option<f64>,
    // Optional, used for sell orders
    
    quote_order_qty: Option<f64>,
    // Optional, used for buy orders
    timestamp: u64,
}

impl MarketOrder {
    pub fn new_with_base_asset(symbol: &str, side: Side, quantity: f64) -> Self {
        MarketOrder {
            symbol: symbol.to_string(),
            side,
            r#type: "MARKET".to_string(),
            quantity: Some(quantity),
            quote_order_qty: None,
            timestamp: BinanceClient::generate_timestamp().unwrap(),
        }
    }

    // Creates a market order where the quoteOrderQty specifies the quote asset amount
    pub fn new_with_quote_asset(symbol: &str, side: Side, quote_order_qty: f64) -> Self {
        MarketOrder {
            symbol: symbol.to_string(),
            side,
            r#type: "MARKET".to_string(),
            quantity: None,
            quote_order_qty: Some(quote_order_qty),
            timestamp: BinanceClient::generate_timestamp().unwrap(),
        }
    }
}
