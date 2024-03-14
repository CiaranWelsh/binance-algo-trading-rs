use serde::{Deserialize, Serialize};

/// Represents a message for depth updates, applicable for both full and partial book depth streams.
#[derive(Debug, Serialize, Deserialize)]
pub struct DepthMessage {
    stream: String, // Stream name identifies whether it's full or partial depth (e.g., btcusdt@depth or btcusdt@depth5).
    data: DepthData,
}

/// Contains depth data including bids and asks. For partial depth streams,
/// the number of bids and asks corresponds to the specified level in the stream name.
#[derive(Debug, Serialize, Deserialize)]
pub struct DepthData {
    #[serde(rename = "e")]
    event_type: String, // Event type
    #[serde(rename = "E")]
    event_time: u64, // Event time
    #[serde(rename = "s")]
    symbol: String, // Symbol
    #[serde(rename = "U")]
    first_update_id: u64, // First update ID in event
    #[serde(rename = "u")]
    final_update_id: u64, // Final update ID in event
    #[serde(rename = "b")]
    bids: Vec<Bid>, // Bids, limited by the depth level for partial streams
    #[serde(rename = "a")]
    asks: Vec<Ask>, // Asks, limited by the depth level for partial streams
}

/// Represents a single bid in the order book.
#[derive(Debug, Serialize, Deserialize)]
pub struct Bid {
    #[serde(rename = "p")]
    price: String, // Price of bid
    #[serde(rename = "q")]
    quantity: String, // Quantity of bid
}

/// Represents a single ask in the order book.
#[derive(Debug, Serialize, Deserialize)]
pub struct Ask {
    #[serde(rename = "p")]
    price: String, // Price of ask
    #[serde(rename = "q")]
    quantity: String, // Quantity of ask
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn depth_message_deserialization() {
        let json_str = r#"{
            "stream": "btcusdt@depth",
            "data": {
                "e": "depthUpdate",
                "E": 1710418797466,
                "s": "BTCUSDT",
                "U": 754130,
                "u": 754134,
                "b": [["72724.10000000", "0.00092000"]],
                "a": [
                    ["66863.00000000", "0.00000000"],
                    ["72179.86000000", "0.00000000"],
                    ["72724.11000000", "0.00654000"],
                    ["72724.28000000", "0.00399000"],
                    ["72852.00000000", "0.04784000"],
                    ["72852.01000000", "0.00000000"]
                ]
            }
        }"#;

        // Attempt to deserialize the JSON string into a DepthMessage
        let depth_message: Result<DepthMessage, _> = from_str(json_str);

        // Verify the deserialization was successful
        assert!(depth_message.is_ok(), "Failed to deserialize JSON into DepthMessage");

        // Further tests could include verifying the contents of the deserialized object
        let depth_message = depth_message.unwrap();
        assert_eq!(depth_message.stream, "btcusdt@depth");
        assert_eq!(depth_message.data.event_type, "depthUpdate");
        assert_eq!(depth_message.data.symbol, "BTCUSDT");
        assert_eq!(depth_message.data.first_update_id, 754130);
        assert_eq!(depth_message.data.final_update_id, 754134);
        assert_eq!(depth_message.data.bids.len(), 1);
        assert_eq!(depth_message.data.asks.len(), 6);
        // Verify the first bid
        assert_eq!(depth_message.data.bids[0].price, "72724.10000000");
        assert_eq!(depth_message.data.bids[0].quantity, "0.00092000");
        // Verify the first ask
        assert_eq!(depth_message.data.asks[0].price, "66863.00000000");
        assert_eq!(depth_message.data.asks[0].quantity, "0.00000000");
    }
}
