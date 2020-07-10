//! This crate provides a mock [`ethrs::transport::Transport`] implementation
//! that can be used for unit testing.

use crate::transport::Transport;
use serde_json::{json, Value};
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
        result: Result<Value, String>,
    ) -> &mut Self {
        self.calls
            .push_back((method.into(), params, result.map_err(Error)));
        self
    }

    /// Check call to the mock transport.
    fn call_inner(&mut self, request: &[u8], response: &mut Vec<u8>) -> Result<(), Error> {
        /// Macro for construcing a mock transport error.
        macro_rules! error {
            ($($x:tt)*) => {
                Error(format!($($x)*))
            }
        };
        macro_rules! ensure_eq {
            ($actual:expr, $expected:expr, $msg:expr) => {{
                let (actual, expected) = (&$actual, &$expected);
                if actual != expected {
                    return Err(error!(
                        "{}, got {:?} but expected {:?}",
                        $msg, actual, expected,
                    ));
                }
            }};
        };

        let request = serde_json::from_slice::<Value>(request).map_err(|err| {
            error!(
                "invalid request JSON '{}': {}",
                String::from_utf8_lossy(request),
                err,
            )
        })?;

        let (method, expected, result) = self
            .calls
            .pop_front()
            .ok_or_else(|| error!("unexpected '{}' request", request["method"]))?;
        ensure_eq!(request["method"], method, "unexpected method");
        ensure_eq!(request["params"], expected, "unexpected parameters");

        let json = match result {
            Ok(result) => json!({
                "jsonrpc": "2.0",
                "result": result,
                "id": request["id"],
            }),
            Err(err) => json!({
                "jsonrpc": "2.0",
                "error": {
                    "code": -32000,
                    "error": err.0,
                    "data": {},
                },
                "id": request["id"],
            }),
        };
        serde_json::to_writer(response, &json)
            .map_err(|err| error!("error serializing response JSON: {}", err))?;

        Ok(())
    }
}

impl Transport for MockTransport {
    type Error = Error;
    type Call = Ready<Result<(), Error>>;

    fn call(&mut self, request: &[u8], response: &mut Vec<u8>) -> Self::Call {
        Ready::new(self.call_inner(request, response))
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
