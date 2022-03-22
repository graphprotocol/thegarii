// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! encoding utils
use serde::de::{self, Deserialize, Deserializer};
use serde_json::value::Value;

/// parse number or string to string
pub fn number_or_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let v = Value::deserialize(deserializer)?;
    Ok(match v {
        Value::Number(n) => n.to_string(),
        Value::String(s) => s,
        _ => {
            return Err(de::Error::custom(
                "invalid diff type, expect number or string",
            ))
        }
    })
}

/// parse number or string to string
pub fn option_number_or_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let v = Value::deserialize(deserializer)?;
    Ok(match v {
        Value::Number(n) => Some(n.to_string()),
        Value::String(s) => Some(s),
        Value::Null => None,
        _ => {
            return Err(de::Error::custom(
                "invalid diff type, expect number or string",
            ))
        }
    })
}
