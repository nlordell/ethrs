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

// TODO(nlordell): Renable this option.
//#![deny(missing_docs, unsafe_code)]

#[macro_use]
pub mod ethereum;
pub mod jsonrpc;
pub mod transport;

pub use ethereum::Web3;
