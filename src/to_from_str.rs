//! Utilities for serializing and deserializing types using their string representations.
//!
//! This module provides helper functions to serialize and deserialize types that implement
//! [`ToString`] and [`FromStr`] respectively. These functions are intended to be used with
//! Serde's `#[serde(with = "...")]` attribute to enable (de)serialization via string conversion.
//!
//! # Note
//! The `ToString` and `FromStr` implementations for a type must be true inverses of each other
//! for correct round-trip serialization and deserialization. If this is not the case, data loss
//! or errors may occur.
//!
//! # Example
//! ```rust
//! use std::net::IpAddr;
//! use serde::{Serialize, Deserialize};
//! use serde_json;
//!
//! #[derive(Serialize, Deserialize, Debug, PartialEq)]
//! struct Wrapper {
//!     #[serde(with = "serde_extras::to_from_str")]
//!     ip: IpAddr,
//! }
//!
//! let w = Wrapper { ip: IpAddr::V4("127.0.0.1".parse().unwrap()) };
//! let json = serde_json::to_string(&w).unwrap();
//! assert_eq!(json, r#"{"ip":"127.0.0.1"}"#);
//! let de: Wrapper = serde_json::from_str(&json).unwrap();
//! assert_eq!(de, w);
//! ```

use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Deserializer, Serializer};

/// Deserializes a value from a string using its [`FromStr`] implementation.
///
/// This function is intended to be used with Serde's `#[serde(deserialize_with = "...")]` attribute.
/// It attempts to parse the input string into the target type `T`. If parsing fails, a Serde error
/// is returned.
///
/// # Errors
/// Returns a Serde error if the input string cannot be parsed into the target type.
pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    T::from_str(s).map_err(serde::de::Error::custom)
}

/// Serializes a value to a string using its [`ToString`] implementation.
///
/// This function is intended to be used with Serde's `#[serde(serialize_with = "...")]` attribute.
/// It converts the value to a string and serializes it as a string.
///
/// # Errors
/// Returns a Serde error if serialization fails.
pub fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: ToString,
    S: Serializer,
{
    serializer.serialize_str(&value.to_string())
}
