//! Module containing serializable JSON RPC data types.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::convert::TryFrom;
use thiserror::Error;

/// JSON RPC supported version.
#[derive(Debug, Deserialize, Serialize)]
pub enum Version {
    /// Version 2.0 of the JSON RPC specification.
    #[serde(rename = "2.0")]
    V2,
}

/// Request and response ID.
///
/// Note that `u32` is used. This is so it always fits in a `f64` and obeys the
/// "SHOULD NOT have fractional parts" rule from the specification.  Since the
/// ID is set by the client, we shouldn't run into issues where a numerical ID
/// does not fit into this value or a string ID is used.
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Id(pub u32);

/// Request object.
#[derive(Debug, Serialize)]
pub struct Request<'a, P> {
    pub jsonrpc: Version,
    pub method: &'a str,
    pub params: P,
    pub id: Id,
}

/// Response object.
#[derive(Debug, Deserialize)]
#[serde(try_from = "RawResponse<R>")]
pub struct Response<R> {
    pub jsonrpc: Version,
    pub result: Result<R, Error>,
    pub id: Option<Id>,
}

impl<R> TryFrom<RawResponse<R>> for Response<R> {
    type Error = MissingResultError;

    fn try_from(raw: RawResponse<R>) -> Result<Self, Self::Error> {
        Ok(Response {
            jsonrpc: raw.jsonrpc,
            result: match (raw.result, raw.error) {
                (Some(result), _) => Ok(result),
                (None, Some(error)) => Err(error),
                (None, None) => return Err(MissingResultError),
            },
            id: raw.id,
        })
    }
}

#[derive(Debug, Error)]
#[error("missing 'result' or 'error' field")]
pub struct MissingResultError;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawResponse<R> {
    pub jsonrpc: Version,
    pub result: Option<R>,
    pub error: Option<Error>,
    pub id: Option<Id>,
}

/// An RPC error that may be produced on a response.
#[derive(Debug, Deserialize, Error)]
#[error("{code}: {error}")]
#[serde(deny_unknown_fields)]
pub struct Error {
    pub code: ErrorCode,
    pub error: String,
    pub data: Value,
}

/// An error code.
#[derive(Debug, Deserialize, Error)]
#[serde(from = "i32")]
pub enum ErrorCode {
    #[error("parse error")]
    ParseError,
    #[error("invalid request")]
    InvalidRequest,
    #[error("method not found")]
    MethodNotFound,
    #[error("invalid params")]
    InvalidParams,
    #[error("internal error")]
    InternalError,
    #[error("server error ({0})")]
    ServerError(i32),
    #[error("reserved ({0})")]
    Reserved(i32),
    #[error("{0}")]
    Other(i32),
}

impl From<i32> for ErrorCode {
    fn from(code: i32) -> Self {
        #[allow(clippy::match_overlapping_arm)]
        match code {
            -32700 => ErrorCode::ParseError,
            -32600 => ErrorCode::InvalidRequest,
            -32601 => ErrorCode::MethodNotFound,
            -32602 => ErrorCode::InvalidParams,
            -32603 => ErrorCode::InternalError,
            -32099..=-32000 => ErrorCode::ServerError(code),
            -32768..=-32000 => ErrorCode::Reserved(code),
            _ => ErrorCode::Other(code),
        }
    }
}
