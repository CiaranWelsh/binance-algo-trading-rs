use serde::{Deserialize, Deserializer};
use std::str::FromStr;
use std::marker::PhantomData;

// Generic function to deserialize a string to a numeric type T
pub fn deserialize_string_to_numeric<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: FromStr, // Require the type to be created from a string
        D: Deserializer<'de>, // The deserializer type
        <T as FromStr>::Err: std::fmt::Display, // Error type must implement Display
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse::<T>().map_err(serde::de::Error::custom)
}

// Helper function to facilitate specific type deserialization
pub fn deserialize_string_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
{
    deserialize_string_to_numeric::<f64, D>(deserializer)
}

pub fn deserialize_string_to_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
{
    deserialize_string_to_numeric::<u64, D>(deserializer)
}

pub fn deserialize_string_to_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
    where
        D: Deserializer<'de>,
{
    deserialize_string_to_numeric::<i64, D>(deserializer)
}


// Generic function to deserialize an optional string to a numeric type T
pub fn deserialize_optional_string_to_numeric<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        T: FromStr, // Require the type to be created from a string
        D: Deserializer<'de>, // The deserializer type
        <T as FromStr>::Err: std::fmt::Display, // Error type must implement Display
{
    let option: Option<String> = Option::deserialize(deserializer)?;
    match option {
        Some(s) => s.parse::<T>().map(Some).map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}

// Helper functions to facilitate specific optional type deserialization
pub fn deserialize_optional_string_to_f64<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
    where
        D: Deserializer<'de>,
{
    deserialize_optional_string_to_numeric::<f64, D>(deserializer)
}

pub fn deserialize_optional_string_to_u64<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
    where
        D: Deserializer<'de>,
{
    deserialize_optional_string_to_numeric::<u64, D>(deserializer)
}

pub fn deserialize_optional_string_to_i64<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
    where
        D: Deserializer<'de>,
{
    deserialize_optional_string_to_numeric::<i64, D>(deserializer)
}


pub fn deserialize_optional_string_to_bool<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
    where
        D: Deserializer<'de>,
{
    let option: Option<String> = Option::deserialize(deserializer)?;
    match option {
        Some(s) => s.parse::<bool>().map(Some).map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}

