//! This module contains trait and type definitions needed for implementing
//! `ethrs` transports.

use std::error::Error;
use std::future::Future;

/// A trait to represent a simplex transport that can be used perform JSON RPC
/// calls where the transport layer garantees that the requests and responses
/// are matched together (JSON RPC over HTTP(S) for example).
///
/// This trait is not suitable for duplex transports where the sent requests may
/// come back out of order (JSON RPC over WebSockets for example).
pub trait Transport {
    /// Error type that this transport produces.
    type Error: Error;

    /// Future returned by the `call` method.
    type Call: Future<Output = Result<(), Self::Error>>;

    /// Perform a JSON RPC call over this transport by sending the serialized
    /// `request` bytes and receive the result into `response` buffer.
    fn call(&mut self, request: &[u8], response: &mut Vec<u8>) -> Self::Call;
}
