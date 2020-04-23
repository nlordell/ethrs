//! This crate provides a mock [`ethrs::transport::Transport`] implementation
//! that can be used for unit testing.

use ethrs_transport_common::Transport;
use serde_json::Value;
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use thiserror::Error;

/// A mock transport that allows setting request expectations
#[derive(Clone, Debug, Default)]
pub struct MockTransport {
    calls: VecDeque<(String, Value, Result<Value, Error>)>,
}

impl MockTransport {
    /// Adds a call expectation to the transport.
    pub fn expect_call(
        &mut self,
        method: impl Into<String>,
        params: Value,
        result: Result<Value, impl Into<String>>,
    ) -> &mut Self {
        self.calls.push_back((
            method.into(),
            params,
            result.map_err(|msg| Error(msg.into())),
        ));
        self
    }
}

impl Transport for MockTransport {
    type Error = Error;
    type Call = Ready<Result<(), Error>>;

    fn call(&mut self, request: &[u8], response: &mut Vec<u8>) -> Self::Call {
        let request = serde_json::from_slice::<Value>(request).unwrap_or_else(|_| {
            panic!(
                "invalid request JSON '{}'",
                String::from_utf8_lossy(request)
            )
        });

        let (method, expected, result) = self
            .calls
            .pop_front()
            .unwrap_or_else(|| panic!("unexpected '{}' request", request["method"]));
        assert_eq!(request["method"], method);
        assert_eq!(request["params"], expected);

        Ready::new(result.map(|value| {
            serde_json::to_writer(response, &value).unwrap();
        }))
    }
}

/// A result future. This is a copy of the implementation from the [`futures`]
/// crate to avoid pulling in multiple dependencies for such a small piece of
/// code.
#[derive(Debug)]
pub struct Ready<T>(Option<T>);

impl<T> Ready<T> {
    /// Create a new ready future.
    fn new(value: T) -> Self {
        Ready(Some(value))
    }
}

impl<T> Future for Ready<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, _: &mut Context) -> Poll<Self::Output> {
        Poll::Ready(
            self.as_mut()
                .0
                .take()
                .expect("future polled after completion"),
        )
    }
}

impl<T> Unpin for Ready<T> {}

/// Error type with an associated message.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("{0}")]
pub struct Error(String);
