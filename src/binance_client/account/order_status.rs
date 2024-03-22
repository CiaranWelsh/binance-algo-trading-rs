use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    // The order has been accepted by the system but not yet processed.
    New,
    // A portion of the order has been filled, and part of the quantity is still awaiting execution.
    PartiallyFilled,
    // The order has been fully executed, and no quantity remains to be filled.
    Filled,
    // The order has been canceled by the user, and no further execution will occur for this order.
    Canceled,
    // arely seen): The cancel request has been received, but the order has not yet been canceled.
    PendingCancel,
    // The order was not accepted by the system and thus was not placed.
    Rejected,
    // The order was canceled due to its time condition not being met. This can happen with day orders that do not get filled by the end of the trading day.
    Expired,
}
