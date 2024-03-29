#[derive(Debug)]
pub enum BinanceStreamTypes {
    Depth(String),
    Trade(String),
    Kline(String, String), // Symbol and interval
    Ticker(String),
    MiniTicker(String),
    BookTicker(String),
    AllMarketMiniTickers
}

impl BinanceStreamTypes {
    pub(crate) fn to_stream_path(&self) -> String {
        match self {
            BinanceStreamTypes::Depth(symbol) => format!("{}@depth", symbol),
            BinanceStreamTypes::Trade(symbol) => format!("{}@trade", symbol),
            BinanceStreamTypes::Kline(symbol, interval) => format!("{}@kline_{}", symbol, interval),
            BinanceStreamTypes::Ticker(symbol) => format!("{}@ticker", symbol),
            BinanceStreamTypes::MiniTicker(symbol) => format!("{}@miniTicker", symbol),
            BinanceStreamTypes::BookTicker(symbol) => format!("{}@bookTicker", symbol),
            BinanceStreamTypes::AllMarketMiniTickers => "!miniTicker@arr".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::thread::sleep;
    use std::time::Duration;
    use log::LevelFilter;
    use crate::binance_client::binance_client::BinanceClient;
    use crate::binance_client::load_env::EnvVars;
    use crate::binance_client::logger_conf::init_logger;
    use crate::binance_client::streams::binance_stream::BinanceStreamTypes;
    use crate::binance_client::streams::binance_websocket::BinanceWebSocket;

    #[tokio::test]
    async fn depth_stream_test() {
        // Initialize logger for detailed output, if needed.
        init_logger(LevelFilter::Trace);

        let is_live = false; // false indicates using testnet.
        let vars = EnvVars::new();
        let mut binance_client = BinanceClient::new(
            vars.api_key.to_string(), vars.api_secret.to_string(), false)
            .await;
        let websocket_api = BinanceWebSocket::new(&binance_client);

        // Define the stream you want to subscribe to - Depth for a test symbol.
        let symbol = "btcusdt"; // Test symbol, make sure it's available in the testnet.
        let streams = vec![BinanceStreamTypes::Depth(symbol.to_string())];

        // Call the method to create and listen to the websocket stream.
        let _result = websocket_api.connect_and_listen(streams).await.unwrap_or_else(|e| {
            panic!("Failed to connect or process messages: {:?}", e);
        });

    }
}