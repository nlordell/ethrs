//! # `ethrs` - Pronounced EE-thers
//!
//! This crate provides a lightweight Ethereum JSON RPC client implementation
//! for interacting with the blockchain. This project's main goals are:
//! - *Standard* compliant to the Ethereum JSON RPC spec.
//! - *Lightweight* and pay for what you use, this means that transport
//!   implementations can be ommitted if not used.
//! - *Extensible* with an ability to add node-specific APIs easily through the
//!   black magic of macros.
//! - *WASM-Ready* and should Just Work (tm) with WASM.

//#![deny(missing_docs)]

#[macro_use]
pub mod ethereum;
mod fmt;
pub mod jsonrpc;

pub use ethereum::Web3;

/// Module re-exporting transport implementations included via features.
pub mod transport {
    pub use ethrs_transport_common::*;

    #[cfg(feature = "http")]
    pub use ethrs_transport_http as http;

    #[cfg(feature = "mock")]
    pub use ethrs_transport_mock as mock;
    #[cfg(feature = "mock")]
    pub use ethrs_transport_mock::MockTransport;
}
