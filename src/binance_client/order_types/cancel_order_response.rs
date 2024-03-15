use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CancelOrderResponse {
    // Define fields according to Binance API response for a canceled order
    symbol: String,
    origClientOrderId: Option<String>,
    orderId: i64,
    orderListId: i64, // Unless dealing with OCO, this will be -1
    clientOrderId: String,
    // Additional fields can be added as necessary
}