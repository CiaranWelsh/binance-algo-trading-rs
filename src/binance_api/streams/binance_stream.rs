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