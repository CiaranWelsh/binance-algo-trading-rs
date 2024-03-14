use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use crate::binance_api::binance_api::BinanceAPI;
use crate::binance_api::account::deserialization::deserialize_string_to_f64;


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AssetBalance {
    asset: String,
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    free: f64,
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    locked: f64,
}

impl AssetBalance {
    pub fn new(asset: &str, free: f64, locked: f64) -> Self {
        AssetBalance {
            asset: asset.to_string(),
            free,
            locked,
        }
    }

    pub fn default() -> Self {
        AssetBalance {
            asset: String::new(),
            free: 0.0,
            locked: 0.0,
        }
    }

    // Use BinanceAPI to retrieve the balance of a specified asset
    pub async fn retrieve_balance(api: &BinanceAPI, asset: &str) -> Result<Self, Box<dyn Error>> {
        let timestamp = BinanceAPI::generate_timestamp()?;
        let recv_window = 5000;
        let params = format!("recvWindow={}&timestamp={}", recv_window, timestamp);
        let signature = api.sign(&params);
        let url = format!("{}{}?{}&signature={}", api.api_url, "/v3/account", params, signature);

        let response: Value = api
            .get_client()
            .get(&url)
            .header("X-MBX-APIKEY", api.get_api_key())
            .send()
            .await?
            .json()
            .await?;

        if let Some(balances) = response["balances"].as_array() {
            for balance in balances {
                if balance["asset"] == asset {
                    let free = balance["free"].as_str().unwrap_or("0").parse::<f64>().unwrap_or(0.0);
                    let locked = balance["locked"].as_str().unwrap_or("0").parse::<f64>().unwrap_or(0.0);
                    return Ok(AssetBalance::new(asset, free, locked));
                }
            }
        }

        Err("Asset not found".into())
    }

    // Adjusted to retrieve balances for all assets
    pub async fn retrieve_all_balances(api: &BinanceAPI) -> Result<Vec<Self>, Box<dyn Error>> {
        let timestamp = BinanceAPI::generate_timestamp()?;
        let recv_window = 5000;
        let params = format!("recvWindow={}&timestamp={}", recv_window, timestamp);
        let signature = api.sign(&params);
        let url = format!("{}{}?{}&signature={}", api.api_url, "/v3/account", params, signature);

        let response: Value = api
            .get_client()
            .get(&url)
            .header("X-MBX-APIKEY", api.get_api_key())
            .send()
            .await?
            .json()
            .await?;

        let mut balances = Vec::new();

        if let Some(balances_data) = response["balances"].as_array() {
            for balance in balances_data {
                let asset = balance["asset"].as_str().unwrap_or_default();
                let free = balance["free"].as_str().unwrap_or("0").parse::<f64>().unwrap_or(0.0);
                let locked = balance["locked"].as_str().unwrap_or("0").parse::<f64>().unwrap_or(0.0);

                balances.push(AssetBalance::new(asset, free, locked));
            }
        }

        if balances.is_empty() {
            Err("No assets found".into())
        } else {
            Ok(balances)
        }
    }

}



#[cfg(test)]
mod tests {
    use super::*;
    use tokio;
    use crate::binance_api::auth::{TEST_NET_API_KEY, TEST_NET_API_SECRET};

    // This is an integration test for the Binance testnet.
    // Make sure to replace "your_api_key" and "your_api_secret" with your actual testnet API key and secret.
    #[tokio::test]
    async fn test_retrieve_balance() {

        let api = BinanceAPI::new(TEST_NET_API_KEY.to_string(), TEST_NET_API_SECRET.to_string(), false);

        // Attempt to retrieve the balance for a testnet asset. This asset should exist on your testnet account.
        // If "BTC" doesn't exist or has never been transacted, try with another asset that exists on your testnet account.
        let asset = "BTC";
        let balance_result = AssetBalance::retrieve_balance(&api, asset).await;

        match balance_result {
            Ok(balance) => {
                println!("Balance for {}: {:?}", asset, balance);
            },
            Err(e) => panic!("Failed to retrieve balance: {}", e),
        }
    }

    #[tokio::test]
    async fn check_retrieve_all_balances(){
        let api = BinanceAPI::new(TEST_NET_API_KEY.to_string(), TEST_NET_API_SECRET.to_string(), false);

        // Attempt to retrieve the balance for a testnet asset. This asset should exist on your testnet account.
        // If "BTC" doesn't exist or has never been transacted, try with another asset that exists on your testnet account.
        let asset = "BTC";
        let balance_result = AssetBalance::retrieve_balance(&api, asset).await;

        match balance_result {
            Ok(balance) => {
                println!("Balance for {}: {:?}", asset, balance);
            },
            Err(e) => panic!("Failed to retrieve balance: {}", e),
        }
    }
}
