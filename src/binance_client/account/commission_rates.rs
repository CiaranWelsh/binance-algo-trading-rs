use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommissionRates {
    buyer: String,
    maker: String,
    seller: String,
    taker: String,
}