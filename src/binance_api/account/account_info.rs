use std::error::Error;
use reqwest::Error as ReqwestError;
use serde_json::error::Error as SerdeJsonError;

use std::fmt;
use serde::{Deserialize, Serialize};
use crate::binance_api::account::asset_balance::AssetBalance;
use crate::binance_api::account::commission_rates::CommissionRates;
use crate::binance_api::binance_api::BinanceAPI;



#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfo {
    #[serde(rename = "accountType")]
    account_type: String,
    balances: Vec<AssetBalance>,
    brokered: bool,
    #[serde(rename = "buyerCommission")]
    buyer_commission: u32,
    #[serde(rename = "canDeposit")]
    can_deposit: bool,
    #[serde(rename = "canTrade")]
    can_trade: bool,
    #[serde(rename = "canWithdraw")]
    can_withdraw: bool,
    #[serde(rename = "commissionRates")]
    commission_rates: CommissionRates,
    #[serde(rename = "makerCommission")]
    maker_commission: u32,
    permissions: Vec<String>,
    #[serde(rename = "preventSor")]
    prevent_sor: bool,
    #[serde(rename = "requireSelfTradePrevention")]
    require_self_trade_prevention: bool,
    #[serde(rename = "sellerCommission")]
    seller_commission: u32,
    #[serde(rename = "takerCommission")]
    taker_commission: u32,
    uid: u64,
    #[serde(rename = "updateTime")]
    update_time: u64,
}


impl AccountInfo {
    pub async fn from_binance_api(api: &BinanceAPI) -> Result<Self, Box<dyn Error>> {
        let timestamp = BinanceAPI::generate_timestamp()?;
        let recv_window = 5000;
        let params = format!("recvWindow={}&timestamp={}", recv_window, timestamp);
        let signature = api.sign(&params);
        let url = format!("{}{}?{}&signature={}", api.api_url, "/v3/account", params, signature);

        let response = api
            .get_client()
            .get(&url)
            .header("X-MBX-APIKEY", api.get_api_key())
            .send()
            .await
            .map_err(ReqwestError::from)?;

        if response.status().is_success() {
            let account_info: Self = response
                .json::<AccountInfo>()
                .await
                .map_err(|e| Box::new(e) as Box<dyn Error>)?;
            Ok(account_info)
        } else {
            // Handling HTTP error responses, parsing error message if possible
            let error_message = response.text().await.unwrap_or_else(|_| "Failed to fetch account info".to_string());
            Err(Box::new(ApiError::new(&error_message)))
        }
    }
}

/// Custom error type for API errors
#[derive(Debug)]
struct ApiError {
    details: String,
}

impl ApiError {
    fn new(msg: &str) -> ApiError {
        ApiError { details: msg.to_string() }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ApiError {
    fn description(&self) -> &str {
        &self.details
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    // Import everything from the parent module
    use tokio;
    // Ensure you have the tokio runtime for async tests
    use std::env;
    use log::LevelFilter::Trace;
    use log::trace;
    use crate::binance_api::auth::{TEST_NET_API_KEY, TEST_NET_API_SECRET};
    use crate::binance_api::logger_conf::init_logger; // For accessing environment variables

    #[tokio::test]
    async fn test_fetch_account_info() {
        init_logger(Trace);
        let api = BinanceAPI::new(TEST_NET_API_KEY.to_string(), TEST_NET_API_SECRET.to_string(), false); // false indicates using testnet

        // Attempt to fetch the account information
        match AccountInfo::from_binance_api(&api).await {
            Ok(account_info) => {
                trace!("account info fetched from binance: \n{:?}", account_info);
                // Success: Perform your assertions here
                // For example, verify that the account can trade
                assert!(account_info.can_trade, "Account should be able to trade");
            }
            Err(e) => {
                // If the API call fails, ensure the test fails
                panic!("Failed to fetch account info: {}", e);
            }
        }
    }
}
