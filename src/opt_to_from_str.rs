
//! Utilities for serializing and deserializing types using their string representations.
//!
//! This module provides helper functions to serialize and deserialize Options who's types implement
//! [`ToString`] and [`FromStr`] respectively. These functions are intended to be used with
//! Serde's `#[serde(with = "...")]` attribute to enable (de)serialization via string conversion.
//!
//! # Note
//! The `ToString` and `FromStr` implementations for a type must be true inverses of each other
//! for correct round-trip serialization and deserialization. If this is not the case, data loss
//! or errors may occur.
//!
//! # Example
//! ```
//! use std::net::IpAddr;
//! use serde::{Serialize, Deserialize};
//! use serde_json;
//!
//! #[derive(Serialize, Deserialize, Debug, PartialEq)]
//! struct Wrapper {
//!     #[serde(with = "serde_extras::opt_to_from_str")]
//!     ip: Option<IpAddr>,
//! }
//!
//! let w = Wrapper { ip: Some("127.0.0.1".parse().unwrap()) };
//! let json = serde_json::to_string(&w).unwrap();
//! assert_eq!(json, r#"{"ip":"127.0.0.1"}"#);
//! let de: Wrapper = serde_json::from_str(&json).unwrap();
//! assert_eq!(de, w);
//! let w_none = Wrapper { ip: None };
//! let json_none = serde_json::to_string(&w_none).unwrap();
//! assert_eq!(json_none, r#"{"ip":null}"#);
//! let de_none: Wrapper = serde_json::from_str(&json_none).unwrap();
//! assert_eq!(de_none, w_none);
//! ```

use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Deserializer, Serializer};

/// Deserializes an optional value from a string using its [`FromStr`] implementation.
///
/// This function is intended to be used with Serde's `#[serde(deserialize_with = "...")]` attribute.
/// It attempts to parse the input string into the target type `Option<T>`. If parsing fails, a Serde error
/// is returned.
///
/// # Errors
/// Returns a Serde error if the input string cannot be parsed into the target type.
pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let opt: Option<&str> = Deserialize::deserialize(deserializer)?;
    match opt {
        Some(s) => T::from_str(s).map(Some).map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}

/// Serializes an optional value to a string using its [`ToString`] implementation.
///
/// This function is intended to be used with Serde's `#[serde(serialize_with = "...")]` attribute.
/// It converts the value to a string and serializes it as a string.
///
/// # Errors
/// Returns a Serde error if serialization fails.
pub fn serialize<S, T>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    T: ToString,
    S: Serializer,
{
    match value {
        Some(v) => serializer.serialize_str(&v.to_string()),
        None => serializer.serialize_none(),
    }
}
