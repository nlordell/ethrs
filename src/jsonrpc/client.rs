//! Module containing transport agnostic JSON RPC client implementation.

use super::data::{self, Id, Request, Response, Version};
use crate::fmt::Dbg;
use crate::transport::Transport;
use lifeguard::{MaxSize, Pool, Supplier};
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use std::error::Error;
use std::sync::atomic::{AtomicU32, Ordering};
use thiserror::Error;

/// A JSON RPC client over a generic simplex transport.
#[derive(Debug)]
pub struct Client<T> {
    transport: T,
    current_id: AtomicU32,
    buffer_pool: Dbg<Pool<Vec<u8>>>,
}

impl<T> Client<T> {
    /// Create a new client over the specified transport.
    pub fn new(transport: T) -> Self {
        Client {
            transport,
            current_id: Default::default(),
            buffer_pool: lifeguard::pool()
                .with(MaxSize(128))
                .with(Supplier(|| Vec::with_capacity(1024)))
                .build()
                .into(),
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
        let mut request_buffer = self.buffer_pool.new();
        let id = self.current_id.fetch_add(1, Ordering::SeqCst);
        serde_json::to_writer(
            &mut *request_buffer,
            &Request {
                jsonrpc: Version::V2,
                method,
                params,
                id: Id(id),
            },
        )?;

        let mut response_buffer = self.buffer_pool.new();
        self.transport
            .call(&*request_buffer, &mut *response_buffer)
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
