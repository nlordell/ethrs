//! This module contains serialization helper types used by the APIs.

use super::encoding::Decode;
use super::models::Syncing;
use serde::de::{self, Deserialize, Deserializer};
use serde_json::Value;

/// A struct for deserializing [`std::option::Option`] of [`ethrs::Syncing`]
/// where `None` is `false` instead of `null`.
pub enum MaybeSyncing {
    /// Not syncing.
    False,
    /// Syncing
    Syncing(Syncing),
}

impl Decode<MaybeSyncing> for Option<Syncing> {
    fn decode(encoded: MaybeSyncing) -> Self {
        match encoded {
            MaybeSyncing::False => None,
            MaybeSyncing::Syncing(value) => Some(value),
        }
    }
}

impl<'de> Deserialize<'de> for MaybeSyncing {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // TODO(nlordell): This implementation can be made more efficient by
        // using a visitor to propertly implement `Deserialize`.

        let json = Value::deserialize(deserializer)?;
        match json {
            Value::Bool(false) => Ok(MaybeSyncing::False),
            value => Ok(MaybeSyncing::Syncing(
                Syncing::deserialize(value).map_err(de::Error::custom)?,
            )),
        }
    }
}
