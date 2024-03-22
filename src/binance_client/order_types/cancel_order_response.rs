use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResponse {
    // Define fields according to Binance API response for a canceled order
    symbol: String,
    orig_client_order_id: Option<String>,
    order_id: i64,
    order_list_id: i64, // Unless dealing with OCO, this will be -1
    client_order_id: String,
}