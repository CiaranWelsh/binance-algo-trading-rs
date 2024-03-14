use dotenv::Error::EnvVar;
use binance_api::binance_client::binance_client::BinanceClient;
use binance_api::binance_client::load_env::EnvVars;
use binance_api::binance_client::position_size::calculate_position_size;
use binance_api::binance_client::spot_orders::SpotClient;

#[tokio::main]
async fn main() {
    let vars = EnvVars::new();
    let binance_client = BinanceClient::new(vars.api_key, vars.api_secret, false).await;
    let spot_client = SpotClient::new(&binance_client);

    let symbol = "ETHUSDT".to_string();
    let account_size = 1000;
    let risk_percentage = 1;

    let reward_ratio = 5;
    let is_long = true;
    let entry_price = binance_client.

    let position_size = calculate_position_size(account_size, risk_percentage, entry_price, reward_ratio, is_long);

}