use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Order {
    symbol: String,
    order_id: i64,
    order_list_id: i64, // Unless OCO, the value will be -1
    client_order_id: String,
    price: String,
    orig_qty: String,
    executed_qty: String,
    cummulative_quote_qty: String,
    status: String,
    time_in_force: String,
    r#type: String,
    side: String,
    stop_price: String,
    iceberg_qty: String,
    time: u64,
    update_time: u64,
    is_working: bool,
    orig_quote_order_qty: String,
}
