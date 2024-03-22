# Filters
Filters define trading rules on a symbol or an exchange.
Filters come in two forms: `symbol filters` and `exchange filters`.

## Symbol filters
### PRICE_FILTER
The `PRICE_FILTER` defines the `price` rules for a symbol. There are 3 parts:

* `min_price` defines the minimum `price`/`stopPrice` allowed; disabled on `min_price` == 0.
* `max_price` defines the maximum `price`/`stopPrice` allowed; disabled on `max_price` == 0.
* `tick_size` defines the intervals that a `price`/`stopPrice` can be increased/decreased by; disabled on `tick_size` == 0.

Any of the above variables can be set to 0, which disables that rule in the `price filter`. In order to pass the `price filter`, the following must be true for `price`/`stopPrice` of the enabled rules:

* `price` >= `min_price`
* `price` <= `max_price`
* `price` % `tick_size` == 0

**/exchangeInfo format:**
```javascript
{
  "filterType": "PRICE_FILTER",
  "min_price": "0.00000100",
  "max_price": "100000.00000000",
  "tick_size": "0.00000100"
}
```

### PERCENT_PRICE
The `PERCENT_PRICE` filter defines the valid range for the price based on the average of the previous trades.
`avg_price_mins` is the number of minutes the average price is calculated over. 0 means the last price is used.

In order to pass the `percent price`, the following must be true for `price`:
* `price` <= `weightedAveragePrice` * `multiplier_up`
* `price` >= `weightedAveragePrice` * `multiplier_down`

**/exchangeInfo format:**
```javascript
{
  "filterType": "PERCENT_PRICE",
  "multiplier_up": "1.3000",
  "multiplier_down": "0.7000",
  "avg_price_mins": 5
}
```

### PERCENT_PRICE_BY_SIDE
The `PERCENT_PRICE_BY_SIDE` filter defines the valid range for the price based on the average of the previous trades.<br/>
`avg_price_mins` is the number of minutes the average price is calculated over. 0 means the last price is used. <br/>
There is a different range depending on whether the order is placed on the `BUY` side or the `SELL` side.

Buy orders will succeed on this filter if:
* `Order price` <= `weightedAveragePrice` * `bidMultiplierUp`
* `Order price` >= `weightedAveragePrice` * `bidMultiplierDown`

Sell orders will succeed on this filter if:
* `Order Price` <= `weightedAveragePrice` * `askMultiplierUp`
* `Order Price` >= `weightedAveragePrice` * `askMultiplierDown`

**/exchangeInfo format:**
```javascript
  {
    "filterType": "PERCENT_PRICE_BY_SIDE",
    "bidMultiplierUp": "1.2",
    "bidMultiplierDown": "0.2",
    "askMultiplierUp": "5",
    "askMultiplierDown": "0.8",
    "avg_price_mins": 1
  }
```


### LOT_SIZE
The `LOT_SIZE` filter defines the `quantity` (aka "lots" in auction terms) rules for a symbol. There are 3 parts:

* `min_qty` defines the minimum `quantity`/`icebergQty` allowed.
* `max_qty` defines the maximum `quantity`/`icebergQty` allowed.
* `step_size` defines the intervals that a `quantity`/`icebergQty` can be increased/decreased by.

In order to pass the `lot size`, the following must be true for `quantity`/`icebergQty`:

* `quantity` >= `min_qty`
* `quantity` <= `max_qty`
* `quantity` % `step_size` == 0

**/exchangeInfo format:**
```javascript
{
  "filterType": "LOT_SIZE",
  "min_qty": "0.00100000",
  "max_qty": "100000.00000000",
  "step_size": "0.00100000"
}
```

### MIN_NOTIONAL
The `MIN_NOTIONAL` filter defines the minimum notional value allowed for an order on a symbol.
An order's notional value is the `price` * `quantity`.
`apply_to_market` determines whether or not the `MIN_NOTIONAL` filter will also be applied to `MARKET` orders.
Since `MARKET` orders have no price, the average price is used over the last `avg_price_mins` minutes.
`avg_price_mins` is the number of minutes the average price is calculated over. 0 means the last price is used.


**/exchangeInfo format:**
```javascript
{
  "filterType": "MIN_NOTIONAL",
  "min_notional": "0.00100000",
  "apply_to_market": true,
  "avg_price_mins": 5
}
```

### NOTIONAL
The `NOTIONAL` filter defines the acceptable notional range allowed for an order on a symbol. <br/><br/>
`applyMinToMarket` determines whether the `min_notional` will be applied to `MARKET` orders. <br/>
`applyMaxToMarket` determines whether the `maxNotional` will be applied to `MARKET` orders.

In order to pass this filter, the notional (`price * quantity`) has to pass the following conditions:

* `price * quantity` <= `maxNotional`
* `price * quantity` >= `min_notional`

For `MARKET` orders, the average price used over the last `avg_price_mins` minutes will be used for calculation. <br/>
If the `avg_price_mins` is 0, then the last price will be used.

**/exchangeInfo format:**
```javascript
{
   "filterType": "NOTIONAL",
   "min_notional": "10.00000000",
   "applyMinToMarket": false,
   "maxNotional": "10000.00000000",
   "applyMaxToMarket": false,
   "avg_price_mins": 5
}
```

### ICEBERG_PARTS
The `ICEBERG_PARTS` filter defines the maximum parts an iceberg order can have. The number of `ICEBERG_PARTS` is defined as `CEIL(qty / icebergQty)`.

**/exchangeInfo format:**
```javascript
{
  "filterType": "ICEBERG_PARTS",
  "limit": 10
}
```

### MARKET_LOT_SIZE
The `MARKET_LOT_SIZE` filter defines the `quantity` (aka "lots" in auction terms) rules for `MARKET` orders on a symbol. There are 3 parts:

* `min_qty` defines the minimum `quantity` allowed.
* `max_qty` defines the maximum `quantity` allowed.
* `step_size` defines the intervals that a `quantity` can be increased/decreased by.

In order to pass the `market lot size`, the following must be true for `quantity`:

* `quantity` >= `min_qty`
* `quantity` <= `max_qty`
* `quantity` % `step_size` == 0

**/exchangeInfo format:**
```javascript
{
  "filterType": "MARKET_LOT_SIZE",
  "min_qty": "0.00100000",
  "max_qty": "100000.00000000",
  "step_size": "0.00100000"
}
```

### MAX_NUM_ORDERS
The `MAX_NUM_ORDERS` filter defines the maximum number of orders an account is allowed to have open on a symbol.
Note that both "algo" orders and normal orders are counted for this filter.

**/exchangeInfo format:**
```javascript
{
  "filterType": "MAX_NUM_ORDERS",
  "max_num_orders": 25
}
```

### MAX_NUM_ALGO_ORDERS
The `MAX_NUM_ALGO_ORDERS` filter defines the maximum number of "algo" orders an account is allowed to have open on a symbol.
"Algo" orders are `STOP_LOSS`, `STOP_LOSS_LIMIT`, `TAKE_PROFIT`, and `TAKE_PROFIT_LIMIT` orders.

**/exchangeInfo format:**
```javascript
{
  "filterType": "MAX_NUM_ALGO_ORDERS",
  "max_num_algo_orders": 5
}
```

### MAX_NUM_ICEBERG_ORDERS
The `MAX_NUM_ICEBERG_ORDERS` filter defines the maximum number of `ICEBERG` orders an account is allowed to have open on a symbol.
An `ICEBERG` order is any order where the `icebergQty` is > 0.

**/exchangeInfo format:**
```javascript
{
  "filterType": "MAX_NUM_ICEBERG_ORDERS",
  "max_num_iceberg_orders": 5
}
```

### MAX_POSITION 

The `MAX_POSITION` filter defines the allowed maximum position an account can have on the base asset of a symbol. An account's position defined as the sum of the account's:
1. free balance of the base asset
1. locked balance of the base asset
1. sum of the qty of all open BUY orders

`BUY` orders will be rejected if the account's position is greater than the maximum position allowed.

If an order's `quantity` can cause the position to overflow, this will also fail the `MAX_POSITION` filter.

**/exchangeInfo format:**
```javascript
{
  "filterType":"MAX_POSITION",
  "max_position":"10.00000000"
}
```

### TRAILING_DELTA

The `TRAILING_DELTA` filter defines the minimum and maximum value for the parameter `trailingDelta`.

In order for a trailing stop order to pass this filter, the following must be true:

For `STOP_LOSS BUY`, `STOP_LOSS_LIMIT_BUY`,`TAKE_PROFIT SELL` and `TAKE_PROFIT_LIMIT SELL` orders: 

* `trailingDelta` >= `min_trailing_above_delta`
* `trailingDelta` <= `max_trailing_above_delta` 

For `STOP_LOSS SELL`, `STOP_LOSS_LIMIT SELL`, `TAKE_PROFIT BUY`, and `TAKE_PROFIT_LIMIT BUY` orders:

* `trailingDelta` >= `min_trailing_below_delta`
* `trailingDelta` <= `max_trailing_below_delta`


**/exchangeInfo format:**

```javascript
    {
          "filterType": "TRAILING_DELTA",
          "min_trailing_above_delta": 10,
          "max_trailing_above_delta": 2000,
          "min_trailing_below_delta": 10,
          "max_trailing_below_delta": 2000
   }
```


## Exchange Filters
### EXCHANGE_MAX_NUM_ORDERS
The `EXCHANGE_MAX_NUM_ORDERS` filter defines the maximum number of orders an account is allowed to have open on the exchange.
Note that both "algo" orders and normal orders are counted for this filter.

**/exchangeInfo format:**
```javascript
{
  "filterType": "EXCHANGE_MAX_NUM_ORDERS",
  "max_num_orders": 1000
}
```

### EXCHANGE_MAX_NUM_ALGO_ORDERS
The `EXCHANGE_MAX_NUM_ALGO_ORDERS` filter defines the maximum number of "algo" orders an account is allowed to have open on the exchange.
"Algo" orders are `STOP_LOSS`, `STOP_LOSS_LIMIT`, `TAKE_PROFIT`, and `TAKE_PROFIT_LIMIT` orders.

**/exchangeInfo format:**
```javascript
{
  "filterType": "EXCHANGE_MAX_NUM_ALGO_ORDERS",
  "max_num_algo_orders": 200
}
```

### EXCHANGE_MAX_NUM_ICEBERG_ORDERS
The `EXCHANGE_MAX_NUM_ICEBERG_ORDERS` filter defines the maximum number of iceberg orders an account is allowed to have open on the exchange.

**/exchangeInfo format:**
```javascript
{
  "filterType": "EXCHANGE_MAX_NUM_ICEBERG_ORDERS",
  "max_num_iceberg_orders": 10000
}
```
