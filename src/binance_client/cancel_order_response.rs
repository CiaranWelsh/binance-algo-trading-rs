use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResponse {
    
    pub symbol: String,
    
    pub orig_client_order_id: String,
    
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
    
    pub self_trade_prevention_mode: String,
}

// Assuming Fill structure is the same as in the previous response
// If not, adjust accordingly
#[derive(Debug, Serialize, Deserialize)]
pub struct Fill {
    
    pub price: String,
    
    pub qty: String,
    
    pub commission: String,
    
    pub commission_asset: String,
}
