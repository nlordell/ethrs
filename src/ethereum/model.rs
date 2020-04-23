//! Module containing all the data model definitions for JSON parameters and
//! results used for RPC.

use serde::{Deserialize, Serialize};

/// Sync status data.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Syncing {
    /// The block at which the import started (will only be reset, after the
    /// sync reached his head).
    pub starting_block: u64,

    /// The current block, same as [`ethrs::ethereum::Eth::block_number`].
    pub current_block: u64,

    /// The estimated highest block.
    pub highest_block: u64,
}
