use std::error::Error;
use reqwest::Error as ReqwestError;
use serde_json::error::Error as SerdeJsonError;

use std::fmt;
use serde::{Deserialize, Serialize};
use crate::binance_client::account::asset_balance::AssetBalance;
use crate::binance_client::account::commission_rates::CommissionRates;
use crate::binance_client::binance_client::BinanceClient;



#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfoClient {
    pub account_type: String,
    pub balances: Vec<AssetBalance>,
    pub brokered: bool,
    
    pub buyer_commission: u32,
    
    pub can_deposit: bool,
    
    pub can_trade: bool,
    
    pub can_withdraw: bool,
    
    pub commission_rates: CommissionRates,
    
    pub maker_commission: u32,
    pub permissions: Vec<String>,
    
    pub prevent_sor: bool,
    
    pub require_self_trade_prevention: bool,
    
    pub seller_commission: u32,
    
    pub taker_commission: u32,
    pub uid: u64,
    
    pub update_time: u64,
}


impl AccountInfoClient {
    pub async fn new(api: &BinanceClient) -> Result<Self, Box<dyn Error>> {
        let timestamp = BinanceClient::generate_timestamp()?;
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
                .json::<AccountInfoClient>()
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
    use crate::binance_client::load_env::EnvVars;
    use crate::binance_client::logger_conf::init_logger; // For accessing environment variables

    #[tokio::test]
    async fn test_fetch_account_info() {
        init_logger(Trace);
        let vars = EnvVars::new();
        let mut api = BinanceClient::new(
            vars.api_key.to_string(), vars.api_secret.to_string(), false)
            .await;

        // Attempt to fetch the account information
        match AccountInfoClient::new(&api).await {
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
