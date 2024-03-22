use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderResponse {
    
    pub symbol: String,
    
    pub order_id: i64,
    
    pub order_list_id: i64,
    
    pub client_order_id: String,
    
    pub transact_time: u64,
    
    pub price: String,
    
    pub orig_qty: String,
    
    pub executed_qty: String,
    
    pub cummulative_quote_qty: String,
    
    pub status: String,
    
    pub time_in_force: String,
    
    pub r#type: String,
    
    pub side: String,
    
    pub working_time: u64,
    
    pub fills: Vec<Fill>,
    
    pub self_trade_prevention_mode: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fill {
    // Define fill structure based on the expected fields in the fills array
    
    pub price: String,
    
    pub qty: String,
    
    pub commission: String,
    
    pub commission_asset: String,
}
