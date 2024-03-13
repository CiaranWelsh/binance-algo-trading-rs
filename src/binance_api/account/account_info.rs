use serde::{Deserialize, Serialize};
use crate::binance_api::account::asset_balance::AssetBalance;
/*
Account Information (/api/v3/account): Provides detailed information about the account, including balances for each asset.
Query Order (/api/v3/order): Retrieves information about an individual order.
Current Open Orders (/api/v3/openOrders): Retrieves all open orders on a symbol.
All Orders (/api/v3/allOrders): Retrieves all orders on a symbol, including open, canceled, and filled orders.

 */
#[derive(Debug, Serialize, Deserialize)]
struct AccountInfo {
    maker_commission: u32,
    taker_commission: u32,
    buyer_commission: u32,
    seller_commission: u32,
    can_trade: bool,
    can_withdraw: bool,
    can_deposit: bool,
    update_time: u64,
    account_type: String,
    balances: Vec<AssetBalance>,
    permissions: Vec<String>,
}




