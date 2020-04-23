//! Module implementing a JSON RPC provider based on a JSON RPC client. The
//! subtle difference between the two is that a provider type acts similarly to
//! `Cow` in that it can be owned or borrowed so that the provider can be shared
//! with a API's sub namespaces.

use crate::jsonrpc::Client;
use std::ops::{Deref, DerefMut};

/// A JSON RPC client API provider. This enum wraps [`ethrs::jsonrpc::Client`]
/// but allows sharing.
#[derive(Debug)]
pub enum Provider<'a, T> {
    /// An owned JSON RPC client.
    Owned(Client<T>),
    /// A shared JSON RPC client.
    Shared(&'a mut Client<T>),
}

impl<T> Provider<'static, T> {
    /// Creates a new provider from the specified transport.
    pub fn new(transport: T) -> Self {
        Provider::Owned(Client::new(transport))
    }
}

impl<T> Provider<'_, T> {
    /// Get a shared copy of the provider.
    pub fn shared(&mut self) -> Provider<'_, T> {
        match self {
            Provider::Owned(client) => Provider::Shared(client),
            Provider::Shared(client) => Provider::Shared(client),
        }
    }
}

impl<T> From<Client<T>> for Provider<'static, T> {
    fn from(client: Client<T>) -> Self {
        Provider::Owned(client)
    }
}

impl<T> Deref for Provider<'_, T> {
    type Target = Client<T>;

    fn deref(&self) -> &Self::Target {
        match self {
            Provider::Owned(client) => client,
            Provider::Shared(client) => client,
        }
    }
}

impl<T> DerefMut for Provider<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Provider::Owned(client) => client,
            Provider::Shared(client) => client,
        }
    }
}
