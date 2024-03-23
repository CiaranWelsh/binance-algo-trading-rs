use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr, Map};

#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OrderResponse {
    pub symbol: String,
    pub order_id: i64,
    #[serde(rename = "orderListId", default)]
    pub order_list_id: i64,
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,
    pub transact_time: u64,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    pub price: Option<f64>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    pub orig_qty: Option<f64>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    pub executed_qty: Option<f64>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    pub cummulative_quote_qty: Option<f64>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub time_in_force: Option<String>,
    #[serde(rename = "type", default)]
    pub order_type: Option<String>,
    #[serde(default)]
    pub side: Option<String>,
    #[serde(default)]
    pub working_time: Option<u64>,
    #[serde(default)]
    pub self_trade_prevention_mode: Option<String>,
    #[serde(default)]
    pub fills: Option<Vec<Fill>>,
    // Conditional Fields
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    pub stop_price: Option<f64>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    pub iceberg_qty: Option<f64>,
    #[serde(default)]
    pub prevented_match_id: Option<i64>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    pub prevented_quantity: Option<f64>,
    #[serde(default)]
    pub strategy_id: Option<i64>,
    #[serde(default)]
    pub strategy_type: Option<i64>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    pub trailing_delta: Option<f64>,
    #[serde(default)]
    pub trailing_time: Option<i64>,
    #[serde(default)]
    pub used_sor: Option<bool>,
    #[serde(default)]
    pub working_floor: Option<String>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Fill {
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub qty: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub commission: f64,
    pub commission_asset: String,
    #[serde(rename = "tradeId")]
    pub trade_id: i64,
}

