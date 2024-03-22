use serde::{Deserialize, Serialize};
use crate::binance_client::deserialization::{
    deserialize_string_to_f64,
    deserialize_string_to_i64,
    deserialize_optional_string_to_i64,
    deserialize_optional_string_to_f64
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInfo {
    timezone: String,
    #[serde(rename = "serverTime")]
    server_time: u64,
    #[serde(rename = "rateLimits")]
    rate_limits: Vec<RateLimit>,
    symbols: Vec<SymbolInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RateLimit {
    #[serde(rename = "rateLimitType")]
    rate_limit_type: String,
    interval: String,
    #[serde(rename = "intervalNum")]
    interval_num: i64,
    #[serde(rename = "limit")]
    limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolInfo {
    #[serde(rename = "symbol")]
    symbol: String,
    #[serde(rename = "status")]
    status: String,
    #[serde(rename = "baseAsset")]
    base_asset: String,
    #[serde(rename = "quoteAsset")]
    quote_asset: String,
    #[serde(rename = "filters")]
    filters: Vec<Filter>,
    #[serde(rename = "permissions")]
    permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "filterType")]
pub enum Filter {
    #[serde(rename = "PRICE_FILTER")]
    #[serde(rename_all = "camelCase")]
    PriceFilter {
        #[serde(rename = "minPrice", deserialize_with = "deserialize_string_to_f64")]
        min_price: f64,
        #[serde(rename = "maxPrice", deserialize_with = "deserialize_string_to_f64")]
        max_price: f64,
        #[serde(rename = "tickSize", deserialize_with = "deserialize_string_to_f64")]
        tick_size: f64,
    },
    #[serde(rename = "LOT_SIZE")]
    #[serde(rename_all = "camelCase")]
    LotSize {
        #[serde(rename = "minQty", deserialize_with = "deserialize_string_to_f64")]
        min_qty: f64,
        #[serde(rename = "maxQty", deserialize_with = "deserialize_string_to_f64")]
        max_qty: f64,
        #[serde(rename = "stepSize", deserialize_with = "deserialize_string_to_f64")]
        step_size: f64,
    },
    #[serde(rename = "NOTIONAL")]
    #[serde(rename_all = "camelCase")]
    Notional {
        #[serde(rename = "minNotional", deserialize_with = "deserialize_string_to_f64")]
        min_notional: f64,
        #[serde(rename = "applyMinToMarket")]
        apply_min_to_market: bool,
        #[serde(rename = "maxNotional", deserialize_with = "deserialize_optional_string_to_f64", default, skip_serializing_if = "Option::is_none")]
        max_notional: Option<f64>,
        #[serde(rename = "applyMaxToMarket", default, skip_serializing_if = "Option::is_none")]
        apply_max_to_market: Option<bool>,
        #[serde(rename = "avgPriceMins", skip_serializing_if = "Option::is_none")]
        avg_price_mins: Option<i64>,
    },
    #[serde(rename = "MIN_NOTIONAL")]
    #[serde(rename_all = "camelCase")]
    MinNotional {
        #[serde(rename = "minNotional", deserialize_with = "deserialize_string_to_f64")]
        min_notional: f64,
        #[serde(rename = "applyToMarket")]
        apply_to_market: bool,
        #[serde(rename = "avgPriceMins", deserialize_with = "deserialize_optional_string_to_i64")]
        avg_price_mins: Option<i64>,
    },
    #[serde(rename = "ICEBERG_PARTS")]
    #[serde(rename_all = "camelCase")]
    IcebergParts {
        #[serde(rename = "limit")]
        limit: i64,
    },
    #[serde(rename = "MARKET_LOT_SIZE")]
    #[serde(rename_all = "camelCase")]
    MarketLotSize {
        #[serde(rename = "minQty", deserialize_with = "deserialize_string_to_f64")]
        min_qty: f64,
        #[serde(rename = "maxQty", deserialize_with = "deserialize_string_to_f64")]
        max_qty: f64,
        #[serde(rename = "stepSize", deserialize_with = "deserialize_string_to_f64")]
        step_size: f64,
    },
    #[serde(rename = "MAX_NUM_ORDERS")]
    #[serde(rename_all = "camelCase")]
    MaxNumOrders {
        #[serde(rename = "maxNumOrders")]
        max_num_orders: i64,
    },
    #[serde(rename = "MAX_NUM_ALGO_ORDERS")]
    #[serde(rename_all = "camelCase")]
    MaxNumAlgoOrders {
        #[serde(rename = "maxNumAlgoOrders")]
        max_num_algo_orders: i64,
    },
    #[serde(rename = "MAX_NUM_ICEBERG_ORDERS")]
    #[serde(rename_all = "camelCase")]
    MaxNumIcebergOrders {
        #[serde(rename = "maxnNumIcebergOrders")]
        max_num_iceberg_orders: i64,
    },
    #[serde(rename = "MAX_POSITION")]
    #[serde(rename_all = "camelCase")]
    MaxPosition {
        #[serde(rename = "maxPosition", deserialize_with = "deserialize_string_to_f64")]
        max_position: f64,
    },
    #[serde(rename = "PERCENT_PRICE")]
    #[serde(rename_all = "camelCase")]
    PercentPrice {
        #[serde(rename = "multiplierUp", deserialize_with = "deserialize_string_to_f64")]
        multiplier_up: f64,
        #[serde(rename = "multiplierDown", deserialize_with = "deserialize_string_to_f64")]
        multiplier_down: f64,
        #[serde(rename = "avgPriceMins",)]
        avg_price_mins: i64,
    },
    #[serde(rename = "TRAILING_DELTA")]
    #[serde(rename_all = "camelCase")]
    TrailingDelta {
        #[serde(rename = "minTrailingAboveDelta")]
        min_trailing_above_delta: i64,
        #[serde(rename = "maxTrailingAboveDelta")]
        max_trailing_above_delta: i64,
        #[serde(rename = "minTrailingBelowDelta")]
        min_trailing_below_delta: i64,
        #[serde(rename = "maxTrailingBelowDelta")]
        max_trailing_below_delta: i64,
    },

    #[serde(rename = "LEVERAGE_FILTER")]
    #[serde(rename_all = "camelCase")]
    LeverageFilter {
        #[serde(rename = "minLeverage", deserialize_with = "deserialize_string_to_f64")]
        min_leverage: f64,
        #[serde(rename = "maxLeverage", deserialize_with = "deserialize_string_to_f64")]
        max_leverage: f64,
        #[serde(rename = "leverageStep", deserialize_with = "deserialize_string_to_f64")]
        leverage_step: f64,
    },
    #[serde(rename = "MARGIN_FILTER")]
    #[serde(rename_all = "camelCase")]
    MarginFilter {
        #[serde(rename = "minMargin", deserialize_with = "deserialize_string_to_f64")]
        min_margin: f64,
        #[serde(rename = "maxMargin", deserialize_with = "deserialize_string_to_f64")]
        max_margin: f64,
        #[serde(rename = "marginStep", deserialize_with = "deserialize_string_to_f64")]
        margin_step: f64,
    },

    #[serde(rename = "PERCENT_PRICE_BY_SIDE")]
    #[serde(rename_all = "camelCase")]
    PercentPriceBySide {
        #[serde(rename = "bidMultiplierUp", deserialize_with = "deserialize_string_to_f64")]
        bid_multiplier_up: f64,
        #[serde(rename = "bidMultiplierDown", deserialize_with = "deserialize_string_to_f64")]
        bid_multiplier_down: f64,
        #[serde(rename = "askMultiplierUp", deserialize_with = "deserialize_string_to_f64")]
        ask_multiplier_up: f64,
        #[serde(rename = "askMultiplierDown", deserialize_with = "deserialize_string_to_f64")]
        ask_multiplier_down: f64,
        #[serde(rename = "avgPriceMins")]
        avg_price_mins: i64,
    },
}

