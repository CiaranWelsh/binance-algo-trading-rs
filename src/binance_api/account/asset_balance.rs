use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AssetBalance {
    asset: String,
    free: String,
    locked: String,
}

impl AssetBalance {
    // Constructor for creating a new AssetBalance with specified values.
    pub fn new(asset: &str, free: &str, locked: &str) -> Self {
        AssetBalance {
            asset: asset.to_string(),
            free: free.to_string(),
            locked: locked.to_string(),
        }
    }

    // Constructor for creating a default AssetBalance, useful for testing or default values.
    pub fn default() -> Self {
        AssetBalance {
            asset: String::new(),
            free: "0".to_string(),
            locked: "0".to_string(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // Test the new constructor.
    #[test]
    fn test_new() {
        let balance = AssetBalance::new("BTC", "100", "10");
        assert_eq!(
            balance,
            AssetBalance {
                asset: "BTC".to_string(),
                free: "100".to_string(),
                locked: "10".to_string(),
            }
        );
    }

    // Test the default constructor.
    #[test]
    fn test_default() {
        let balance = AssetBalance::default();
        assert_eq!(
            balance,
            AssetBalance {
                asset: "".to_string(),
                free: "0".to_string(),
                locked: "0".to_string(),
            }
        );
    }
}

