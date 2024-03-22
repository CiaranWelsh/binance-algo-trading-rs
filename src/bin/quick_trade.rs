use std::io::Error;
use dotenv::Error::EnvVar;
use log::LevelFilter::Trace;
use log::trace;
use regex::Error::Syntax;
use reqwest::Client;
use binance_api::binance_client::account::account_info::AccountInfoClient;
use binance_api::binance_client::binance_client::BinanceClient;
use binance_api::binance_client::load_env::EnvVars;
use binance_api::binance_client::logger_conf::init_logger;
use binance_api::binance_client::order_types::market_order::MarketOrder;
use binance_api::binance_client::order_types::oco_order::{OcoOrder};
use binance_api::binance_client::order_types::side::Side;
use binance_api::binance_client::order_types::time_in_force::TimeInForce;
use binance_api::binance_client::position_size::{calculate_position_size, round};
use binance_api::binance_client::spot_orders::SpotClient;


#[tokio::main]
async fn main() {
    init_logger(Trace);
    let base = "ETH";
    let quote = "USDC";
    let symbol = format!("{}{}", base, quote); 

    let account_size = 1000.0;
    let risk_percentage = 0.00025;


    let vars = EnvVars::new();
    let binance_client = BinanceClient::new(vars.api_key, vars.api_secret, false).await;
    let spot_client = SpotClient::new(&binance_client);

    // let url = binance_client.api_url.clone() + "/v3/exchangeInfo?symbol=" + symbol.as_str();
    // let client = Client::new();
    // let response = client.get(url).send().await.unwrap().text().await.unwrap();

    // Parse the response JSON and extract the information related to order types
    // This is just a placeholder; actual implementation will depend on how you parse the JSON
    // println!("{}", response);



    for i in AccountInfoClient::new(&binance_client).await.unwrap().balances.iter(){
        if i.asset == base || i.asset == quote {
            trace!("Balance before trading: {:?}", i);
        }
    }


    let reward_ratio = 5.0;
    let is_long = true;
    let entry_price = binance_client.get_current_price(symbol.as_str()).await.unwrap();
    let stop_loss_price = entry_price.price - 0.01;

    trace!("current price == entry price = {:?}", entry_price);

    let (mut position_size, mut take_profit_price, mut stop_loss_price) = calculate_position_size(
        account_size,
        risk_percentage,
        entry_price.price,
        stop_loss_price,
        reward_ratio,
        is_long,
    ).unwrap();
    take_profit_price = round(take_profit_price, 2);
    stop_loss_price = round(stop_loss_price, 2);
    let stop_limit_price = stop_loss_price + 0.1;
    position_size = round(position_size, 2);
    // position_size = 0.1;
    trace!("entry_price: {:?}", entry_price.price);
    trace!("take_profit_price: {:?}", take_profit_price);
    trace!("stop_loss_price: {:?}", stop_loss_price);
    trace!("position_size: {:?} {}", position_size, base);

    let mo = MarketOrder::new_with_base_asset(symbol.as_str(), Side::Buy, position_size);

    trace!("Market Order: {:?}", mo);
    let resp = spot_client.create_market_order(mo.clone()).await;

    match resp {
        Ok(response) => {
            trace!("Market order trade placed successfully: {:?}", mo);
            let ts = BinanceClient::generate_timestamp().unwrap();
            let mut oco = OcoOrder::new(
                symbol.to_string(), Side::Sell, position_size, take_profit_price, stop_loss_price, stop_limit_price, ts,
            );
            oco.stop_limit_time_in_force = Some(TimeInForce::GTC);
            let resp2 = spot_client.create_oco_order(oco).await;

            match resp2 {
                Ok(_) => {
                    trace!("OCO Sell order sucessfully placed")
                }
                Err(err) => {
                    panic!("OCO order failed with : {:?}", err)
                }
            }
        }
        Err(err) => { panic!("Market order failed: {:?}", err) }
    }
    for i in AccountInfoClient::new(&binance_client).await.unwrap().balances.iter(){
        if i.asset == base || i.asset == quote {
            trace!("Balance before trading: {:?}", i);
        }
    }
    
    let open_orders = binance_client.fetch_open_orders(symbol.as_str()).await.unwrap();
    for o in open_orders.iter(){
        trace!("order: {:?}", o);
    }
    // let socket = binance_client.create_websocket_stream_with_listen_key().await;
    


}