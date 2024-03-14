use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct KlineMessage {
    pub stream: String,
    pub data: KlineData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KlineData {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    pub k: Kline,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Kline {
    #[serde(rename = "t")]
    pub start_time: u64,
    #[serde(rename = "T")]
    pub end_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "i")]
    pub interval: String,
    #[serde(rename = "f")]
    pub first_trade_id: u64,
    #[serde(rename = "L")]
    pub last_trade_id: u64,
    #[serde(rename = "o")]
    pub open_price: f64,
    #[serde(rename = "c")]
    pub close_price: f64,
    #[serde(rename = "h")]
    pub high_price: f64,
    #[serde(rename = "l")]
    pub low_price: f64,
    #[serde(rename = "v")]
    pub base_asset_volume: f64,
    #[serde(rename = "n")]
    pub number_of_trades: u32,
    #[serde(rename = "x")]
    pub is_kline_closed: bool,
    #[serde(rename = "q")]
    pub quote_asset_volume: f64,
    #[serde(rename = "V")]
    pub taker_buy_base_asset_volume: f64,
    #[serde(rename = "Q")]
    pub taker_buy_quote_asset_volume: f64,
    #[serde(rename = "B")]
    pub ignore: String,
}



#[cfg(test)]
mod tests {
    use serde_json::Error;
    use super::*;

    #[test]
    fn test_kline_message_deserialization() {
        let json_data = r#"{
            "stream":"ethusdt@kline_1m",
            "data":{
                "e":"kline",
                "E":1710420011529,
                "s":"ETHUSDT",
                "k":{
                    "t":1710420000000,
                    "T":1710420059999,
                    "s":"ETHUSDT",
                    "i":"1m",
                    "f":45859,
                    "L":45874,
                    "o":"3938.10000000",
                    "c":"3937.65000000",
                    "h":"3938.26000000",
                    "l":"3937.65000000",
                    "v":"0.74280000",
                    "n":16,
                    "x":false,
                    "q":"2925.13303100",
                    "V":"0.29710000",
                    "Q":"1170.03364600",
                    "B":"0"
                }
            }
        }"#;
        let parsed: Result<KlineMessage, serde_json::Error> = serde_json::from_str(json_data);

        match parsed {
            Ok(kline_message) => {
                assert_eq!(kline_message.stream, "ethusdt@kline_1m");
                assert_eq!(kline_message.data.event_type, "kline");
                assert_eq!(kline_message.data.symbol, "ETHUSDT");
                assert_eq!(kline_message.data.k.interval, "1m");
                assert!(!kline_message.data.k.is_kline_closed);
            }
            Err(e) => {
                panic!("error: {:?}", e);
            }
        }


    }
}

