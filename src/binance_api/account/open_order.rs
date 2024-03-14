

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenOrder {
    // This struct can be similar to `Order` with possibly fewer fields depending on the API's response
    symbol: String,
    order_id: i64,
    client_order_id: String,
    price: String,
    orig_qty: String,
    executed_qty: String,
    status: String,
    time_in_force: String,
    r#type: String,
    side: String,
    stop_price: String,
    iceberg_qty: String,
    time: u64,
}
