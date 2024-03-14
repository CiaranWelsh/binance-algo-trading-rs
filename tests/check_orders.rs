#[cfg(test)]
mod integration_tests {
    use super::*;
    use std::env;
    use log::LevelFilter::Trace;
    use log::trace;
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

    #[tokio::test]
    async fn test_create_limit_order() {
        init_logger(Trace);
        // Load API keys from environment variables

        // Initialize BinanceAPI with testnet configuration
        let vars = EnvVars::new();
        let binance_client = BinanceClient::new(vars.api_key, vars.api_secret, false).await;

        // Initialize SpotOrders
        let spot_orders = SpotClient::new(&binance_client);
        binance_client.cancel_all_open_orders("ETHUSDT").await.expect("Nopey cancel orders");


        // Define a limit order (replace with testnet compatible values)
        // symbol: String, side: String, quantity: f64, price: f64
        let limit_order = LimitOrder::new(
            "ETHUSDT".to_string(),
            Side::Buy,
            0.01,
            2500.0,
            BinanceClient::generate_timestamp().unwrap(),
        );


        // Attempt to create a limit order
        let result = spot_orders.create_limit_order(limit_order).await;

        trace!("{:?}", result);

        // Assert that the order creation was successful
        assert!(result.is_ok(), "Failed to create limit order: {:?}", result.err());
    }


    #[tokio::test]
    async fn test_create_sell_limit_order() {
        init_logger(Trace);

        // Initialize BinanceAPI with testnet configuration
        let vars = EnvVars::new();
        let binance_client = BinanceClient::new(vars.api_key, vars.api_secret, false).await;
        binance_client.cancel_all_open_orders("ETHUSDT").await.expect("Nopey cancel orders");

        // Initialize SpotOrders
        let spot_orders = SpotClient::new(&binance_client);

        // Generate a timestamp
        let timestamp = BinanceClient::generate_timestamp().unwrap();

        // Define a sell limit order
        // Ensure you have sufficient balance of the asset you're trying to sell on the testnet
        let sell_limit_order = LimitOrder::new(
            "ETHUSDT".to_string(), // Make sure to use a symbol you have in your test account
            Side::Sell,
            0.01, // Quantity to sell
            3000.0, // Sell price, set this according to current market conditions for the test to pass
            timestamp,
        );

        // Attempt to create a sell limit order
        let result = spot_orders.create_limit_order(sell_limit_order).await;

        trace!("{:?}", result);

        // Assert that the sell limit order creation was successful
        assert!(result.is_ok(), "Failed to create sell limit order: {:?}", result.err());
    }


    #[tokio::test]
    async fn test_create_buy_market_order_using_base_asset() {
        init_logger(Trace);

        // Initialize BinanceAPI with testnet configuration
        let vars = EnvVars::new();
        let binance_client = BinanceClient::new(vars.api_key, vars.api_secret, false).await;
        binance_client.cancel_all_open_orders("ETHUSDT").await.expect("Nopey cancel orders");

        // Initialize SpotOrders
        let spot_orders = SpotClient::new(&binance_client);

        // base asset is eth.
        let buy_market_order = MarketOrder::new_with_base_asset(
            "ETHUSDT".to_string(),
            Side::Buy,
            0.1,
        );

        // Attempt to create a buy market order
        let result = spot_orders.create_market_order(buy_market_order).await;

        trace!("{:?}", result);

        // Assert that the buy market order creation was successful
        assert!(result.is_ok(), "Failed to create buy market order: {:?}", result.err());
    }

    #[tokio::test]
    async fn test_create_buy_market_order_using_quote_asset() {
        init_logger(Trace);

        // Initialize BinanceAPI with testnet configuration
        let vars = EnvVars::new();
        let binance_client = BinanceClient::new(vars.api_key, vars.api_secret, false).await;
        binance_client.cancel_all_open_orders("ETHUSDT").await.expect("Nopey cancel orders");

        // Initialize SpotOrders
        let spot_orders = SpotClient::new(&binance_client);

        // Define a buy market order
        let buy_market_order = MarketOrder::new_with_quote_asset(
            "ETHUSDT".to_string(),
            Side::Buy,
            100.0, // usdt
        );

        // Attempt to create a buy market order
        let result = spot_orders.create_market_order(buy_market_order).await;

        trace!("{:?}", result);

        // Assert that the buy market order creation was successful
        assert!(result.is_ok(), "Failed to create buy market order: {:?}", result.err());
    }

    #[tokio::test]
    async fn test_create_sell_market_order_using_base_asset() {
        init_logger(Trace);

        let vars = EnvVars::new();
        let binance_client = BinanceClient::new(vars.api_key, vars.api_secret, false).await;
        binance_client.cancel_all_open_orders("ETHUSDT").await.expect("Nopey cancel orders");
        let spot_orders = SpotClient::new(&binance_client);

        let sell_market_order = MarketOrder::new_with_base_asset(
            "ETHUSDT".to_string(),
            Side::Sell,
            0.1, // Quantity of ETH to sell
        );

        let result = spot_orders.create_market_order(sell_market_order).await;
        trace!("{:?}", result);
        assert!(result.is_ok(), "Failed to create sell market order: {:?}", result.err());
    }

    #[tokio::test]
    async fn test_create_sell_market_order_using_quote_asset() {
        init_logger(Trace);

        let vars = EnvVars::new();
        let binance_client = BinanceClient::new(vars.api_key, vars.api_secret, false).await;
        let spot_orders = SpotClient::new(&binance_client);
        binance_client.cancel_all_open_orders("ETHUSDT").await.expect("Nopey cancel orders");

        // This is a conceptual example; actual implementation requires calculating the ETH amount equivalent to 100 USDT beforehand
        let sell_market_order = MarketOrder::new_with_quote_asset(
            "ETHUSDT".to_string(),
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
        let binance_client = BinanceClient::new(vars.api_key, vars.api_secret, false).await;
        let spot_orders = SpotClient::new(&binance_client);

        // Ensure these values are set correctly according to current market conditions
        let symbol = "ETHUSDT".to_string();
        binance_client.cancel_all_open_orders(symbol.as_str()).await.expect("Nopey cancel orders");
        let quantity = 0.01;
        let stop_price = 10000.0; // Above current market price for buy stop-limit
        let limit_price = 9500.0; // The price at which you actually wish to buy
        let stop_limit_order = StopLimitOrder::new(
            symbol, Side::Buy, quantity, limit_price, stop_price, TimeInForce::GTC,
        );

        let result = spot_orders.create_stop_limit_order(stop_limit_order).await;
        trace!("{:?}", result);
        assert!(result.is_ok(), "Failed to create buy stop-limit order: {:?}", result.err());
    }

    #[tokio::test]
    async fn test_create_sell_stop_limit_order() {
        init_logger(Trace);

        let vars = EnvVars::new();
        let binance_client = BinanceClient::new(vars.api_key, vars.api_secret, false).await;
        let spot_orders = SpotClient::new(&binance_client);

        // Ensure these values are set correctly according to current market conditions
        let symbol = "ETHUSDT".to_string();
        binance_client.cancel_all_open_orders(symbol.as_str()).await.expect("Nopey cancel orders");
        let quantity = 0.01;
        let stop_price = 1500.0; // Above current market price for buy stop-limit
        let limit_price = 1550.0; // The price at which you actually wish to buy
        let stop_limit_order = StopLimitOrder::new(
            symbol, Side::Sell, quantity, limit_price, stop_price, TimeInForce::GTC,
        );

        let result = spot_orders.create_stop_limit_order(stop_limit_order).await;
        trace!("{:?}", result);
        assert!(result.is_ok(), "Failed to create buy stop-limit order: {:?}", result.err());
    }
}
