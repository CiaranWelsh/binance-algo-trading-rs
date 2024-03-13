use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CommissionRates {
    buyer: String,
    maker: String,
    seller: String,
    taker: String,
}