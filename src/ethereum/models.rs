//! Module containing all the data model definitions for JSON parameters and
//! results used for RPC.

use super::encoding::quantity;
use serde::{Deserialize, Serialize};

/// A 32-byte hash.
pub type Hash = [u8; 32];

/// A 20-byte Ethereum address.
pub type Address = [u8; 20];

/// Sync status data.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Syncing {
    /// The block at which the import started (will only be reset, after the
    /// sync reached his head).
    #[serde(rename = "startingBlock", with = "quantity")]
    pub starting_block: u64,

    /// The current block, same as [`ethrs::ethereum::Eth::block_number`].
    #[serde(rename = "currentBlock", with = "quantity")]
    pub current_block: u64,

    /// The estimated highest block.
    #[serde(rename = "highestBlock", with = "quantity")]
    pub highest_block: u64,
}
