use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct KlineMessage {
    stream: String,
    data: KlineData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KlineData {
    #[serde(rename = "e")]
    event_type: String,
    #[serde(rename = "E")]
    event_time: u64,
    #[serde(rename = "s")]
    symbol: String,
    k: Kline,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Kline {
    #[serde(rename = "t")]
    start_time: u64,
    #[serde(rename = "T")]
    end_time: u64,
    #[serde(rename = "s")]
    symbol: String,
    #[serde(rename = "i")]
    interval: String,
    #[serde(rename = "f")]
    first_trade_id: u64,
    #[serde(rename = "L")]
    last_trade_id: u64,
    #[serde(rename = "o")]
    open_price: String,
    #[serde(rename = "c")]
    close_price: String,
    #[serde(rename = "h")]
    high_price: String,
    #[serde(rename = "l")]
    low_price: String,
    #[serde(rename = "v")]
    base_asset_volume: String,
    #[serde(rename = "n")]
    number_of_trades: u32,
    #[serde(rename = "x")]
    is_kline_closed: bool,
    #[serde(rename = "q")]
    quote_asset_volume: String,
    #[serde(rename = "V")]
    taker_buy_base_asset_volume: String,
    #[serde(rename = "Q")]
    taker_buy_quote_asset_volume: String,
    #[serde(rename = "B")]
    ignore: String,
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

