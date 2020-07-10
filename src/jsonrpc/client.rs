//! Module containing transport agnostic JSON RPC client implementation.

use super::data::{self, Id, Request, Response, Version};
use crate::transport::Transport;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use std::error::Error;
use std::sync::atomic::{AtomicU32, Ordering};
use thiserror::Error;

/// Default buffer size used for writing response bytes.
const BUFFER_SIZE: usize = 1024;

/// A JSON RPC client over a generic simplex transport.
#[derive(Debug)]
pub struct Client<T> {
    transport: T,
    current_id: AtomicU32,
}

impl<T> Client<T> {
    /// Create a new client over the specified transport.
    pub fn new(transport: T) -> Self {
        Client {
            transport,
            current_id: Default::default(),
        }
    }
}

impl<T> Client<T>
where
    T: Transport,
{
    pub async fn call<P, R>(&mut self, method: &str, params: P) -> Result<R, ClientError<T::Error>>
    where
        P: Serialize,
        R: DeserializeOwned,
    {
        let id = self.current_id.fetch_add(1, Ordering::SeqCst);
        let request_buffer = serde_json::to_vec(&Request {
            jsonrpc: Version::V2,
            method,
            params,
            id: Id(id),
        })?;

        let mut response_buffer = Vec::with_capacity(BUFFER_SIZE);
        self.transport
            .call(&*request_buffer, &mut response_buffer)
            .await
            .map_err(ClientError::Transport)?;

        let response = serde_json::from_slice::<Response<R>>(&*response_buffer)?;

        Ok(response.result?)
    }
}

/// An error reprenting an issue performing a client operation such as a JSON
/// RPC call.
#[derive(Debug, Error)]
pub enum ClientError<T>
where
    T: Error,
{
    /// An error sending or receiving data over a transport.
    #[error("transport error: {0}")]
    Transport(T),
    /// An error serializing or deserializing the JSON RPC message objects.
    #[error("serialization error: {0}")]
    Json(#[from] serde_json::Error),
    /// An RPC error was returned by the transport.
    #[error("RPC error: {0}")]
    Rpc(#[from] data::Error),
}
