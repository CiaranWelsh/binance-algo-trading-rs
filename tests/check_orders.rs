#[cfg(test)]
mod integration_tests {
    use super::*;
    use std::{env, thread};
    use std::time::Duration;
    use async_std::channel::unbounded;
    use log::LevelFilter::Trace;
    use log::trace;
    use serde::Serialize;
    use binance_api::binance_client::account::order::Order;
    use binance_api::binance_client::account::order_status::OrderStatus;
    use binance_api::binance_client::binance_client::BinanceClient;
    use binance_api::binance_client::logger_conf::init_logger;
    use binance_api::binance_client::order_types::limit_order::LimitOrder;
    use binance_api::binance_client::order_types::market_order::MarketOrder;
    use binance_api::binance_client::order_types::oco_order::OcoOrder;
    use binance_api::binance_client::order_types::side::Side;
    use binance_api::binance_client::order_types::stop_limit_order::StopLimitOrder;
    use binance_api::binance_client::order_types::time_in_force::TimeInForce;
    use binance_api::binance_client::spot_orders::SpotClient;
    use binance_api::binance_client::load_env::EnvVars;
    use binance_api::binance_client::order_response::OrderResponse;
    use binance_api::binance_client::order_types::order_type::OrderType;
    use binance_api::binance_client::position_size::round;

    #[tokio::test]
    async fn test_create_limit_order() {
        init_logger(Trace);
        // Load API keys from environment variables

        let symbol = "ETHUSDT";
        // Initialize BinanceAPI with testnet configuration
        let vars = EnvVars::new();
        let binance_client = BinanceClient::new(vars.api_key, vars.api_secret, false).await;

        // Initialize SpotOrders
        let spot_orders = SpotClient::new(&binance_client);
        

        // Define a limit order (replace with testnet compatible values)
        // symbol: String, side: String, quantity: f64, price: f64
        let limit_order = LimitOrder::new(
            symbol,
            Side::Buy,
            0.01,
            2500.0,
            BinanceClient::generate_timestamp().unwrap(),
        );


        // Attempt to create a limit order
        let order_response: OrderResponse = spot_orders.create_limit_order(limit_order).await
            .expect("Failed to place order");

        spot_orders.cancel_order(symbol, order_response.order_id).await
            .expect("Failed to cancel order");
    }


    #[tokio::test]
    async fn test_create_sell_limit_order() {
        init_logger(Trace);

        // Initialize BinanceAPI with testnet configuration
        let vars = EnvVars::new();
        let symbol = "ETHUSDT";
        let binance_client = BinanceClient::new(vars.api_key, vars.api_secret, false).await;
        binance_client.cancel_all_open_orders(symbol).await.unwrap();

        // Initialize SpotOrders
        let spot_orders = SpotClient::new(&binance_client);

        // Generate a timestamp
        let timestamp = BinanceClient::generate_timestamp().unwrap();

        let current_eth_price = binance_client.get_current_price(symbol).await
            .expect("No eth price");

        let stop_price = round(current_eth_price.price * 1.2, 2);

        trace!("stop price: {:?}", stop_price);

        // Define a sell limit order
        // Ensure you have sufficient balance of the asset you're trying to sell on the testnet
        let sell_limit_order = LimitOrder::new(
            symbol,
            Side::Sell,
            0.01,
            stop_price,
            timestamp,
        );

        // // Attempt to create a sell limit order
        let result = spot_orders.create_limit_order(sell_limit_order).await
            .expect("Failed to create sell limit order");
        trace!("order respo: {:?}", result);

        // thread::sleep(Duration::from_secs(5));
        let cancel_order_result = spot_orders.cancel_order(symbol, result.order_id).await;

        cancel_order_result.expect("Failed to cancel order");

    }


    #[tokio::test]
    async fn test_create_buy_market_order_using_base_asset() {
        init_logger(Trace);

        // Initialize BinanceAPI with testnet configuration
        let vars = EnvVars::new();
        let symbol = "ETHUSDT";

        let binance_client = BinanceClient::new(vars.api_key, vars.api_secret, false).await;

        // Initialize SpotOrders
        let spot_orders = SpotClient::new(&binance_client);

        // base asset is eth.
        let buy_market_order = MarketOrder::new_with_base_asset(
            symbol,
            Side::Buy,
            0.01,
        );

        // Attempt to create a buy market order
        let result = spot_orders.create_market_order(buy_market_order).await
            .unwrap();

        trace!("order response: {:?}", result);


    }

    #[tokio::test]
    async fn test_create_buy_market_order_using_quote_asset() {
        init_logger(Trace);

        // Initialize BinanceAPI with testnet configuration
        let vars = EnvVars::new();
        let symbol = "ETHUSDT";
        let binance_client = BinanceClient::new(vars.api_key, vars.api_secret, false).await;

        // Initialize SpotOrders
        let spot_orders = SpotClient::new(&binance_client);

        // Define a buy market order
        let buy_market_order = MarketOrder::new_with_quote_asset(
            symbol,
            Side::Buy,
            10.0, // usdt
        );

        // Attempt to create a buy market order
        let result = spot_orders.create_market_order(buy_market_order).await;
    }

    #[tokio::test]
    async fn test_create_sell_market_order_using_base_asset() {
        init_logger(Trace);

        let vars = EnvVars::new();
        let symbol = "ETHUSDT";
        let binance_client = BinanceClient::new(vars.api_key, vars.api_secret, false).await;
        
        let spot_orders = SpotClient::new(&binance_client);

        let sell_market_order = MarketOrder::new_with_base_asset(
            symbol,
            Side::Sell,
            0.1, // Quantity of ETH to sell
        );

        let result = spot_orders.create_market_order(sell_market_order).await;
    }

    #[tokio::test]
    async fn test_create_sell_market_order_using_quote_asset() {
        init_logger(Trace);

        let vars = EnvVars::new();
        let symbol = "ETHUSDT";
        let binance_client = BinanceClient::new(vars.api_key, vars.api_secret, false).await;
        let spot_orders = SpotClient::new(&binance_client);
        

        // This is a conceptual example; actual implementation requires calculating the ETH amount equivalent to 100 USDT beforehand
        let sell_market_order = MarketOrder::new_with_quote_asset(
            symbol,
            Side::Sell,
            100.0, // Conceptual value in USDT to receive from selling ETH
        );

        let result = spot_orders.create_market_order(sell_market_order).await;
        trace!("{:?}", result);
        assert!(result.is_ok(), "Conceptual test; real implementation would differ.");
    }


    #[tokio::test]
    async fn test_create_buy_stop_limit_order() {
        init_logger(Trace);

        let vars = EnvVars::new();
        let symbol = "ETHUSDT";
        let binance_client = BinanceClient::new(vars.api_key, vars.api_secret, false).await;
        let spot_orders = SpotClient::new(&binance_client);

        binance_client.cancel_all_open_orders(symbol).await.expect("Failed to cancel open orders");

        let current_price = binance_client.get_current_price(symbol)
            .await.expect("No current price");


        let quantity = 0.01;
        let stop_price = round(current_price.price*1.2, 2); // Above current market price for buy stop-limit
        let limit_price = round(current_price.price*1.25, 2); // The price at which you actually wish to buy
        let stop_limit_order = StopLimitOrder::new(
            symbol, Side::Buy, quantity, limit_price, stop_price, TimeInForce::GTC,
        );

        trace!("Stop limit order: {:?}", serde_json::to_string(&stop_limit_order));

        let result = spot_orders.create_stop_limit_order(stop_limit_order).await
            .expect("Failed to place order");
        spot_orders.cancel_order(symbol, result.order_id).await.unwrap();
    }

    #[tokio::test]
    async fn test_create_sell_stop_limit_order() {
        init_logger(Trace);

        let vars = EnvVars::new();
        let symbol = "ETHUSDT";
        let binance_client = BinanceClient::new(vars.api_key, vars.api_secret, false).await;
        let spot_orders = SpotClient::new(&binance_client);

        let current_price = binance_client.get_current_price(symbol)
            .await.expect("No current price");

        let quantity = 0.01;
        let stop_price = round(current_price.price*0.8, 2); // Above current market price for buy stop-limit
        let limit_price = round(current_price.price*0.75, 2); // The price at which you actually wish to buy
        let stop_limit_order = StopLimitOrder::new(
            symbol, Side::Buy, quantity, limit_price, stop_price, TimeInForce::GTC,
        );

        let result = spot_orders.create_stop_limit_order(stop_limit_order).await;
        spot_orders.cancel_order(symbol, result.unwrap().order_id).await.unwrap();
    }
}
