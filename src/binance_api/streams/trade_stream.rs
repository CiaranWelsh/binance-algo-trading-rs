use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeMessage {
    stream: String,
    data: TradeData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeData {
    #[serde(rename = "e")]
    event_type: String, // Event type
    #[serde(rename = "E")]
    event_time: u64, // Event time
    #[serde(rename = "s")]
    symbol: String, // Symbol
    #[serde(rename = "t")]
    trade_id: u64, // Trade ID
    #[serde(rename = "p")]
    price: String, // Price
    #[serde(rename = "q")]
    quantity: String, // Quantity
    #[serde(rename = "b")]
    buyer_order_id: u64, // Buyer's order ID
    #[serde(rename = "a")]
    seller_order_id: u64, // Seller's order ID
    #[serde(rename = "T")]
    trade_time: u64, // Trade time
    #[serde(rename = "m")]
    is_market_maker: bool, // Is the buyer the market maker?
    #[serde(rename = "M")]
    ignore: bool, // Placeholder (ignore)
}

