//! This module contains a Wasm compatible HTTP POST implementation using the
//! `fetch` API. In order to work with NodeJS, a `fetch` polyfill is needed.

use ethrs_transport::Transport2;
use js_sys::{Function, JsString, Promise, Uint8Array};
use std::{
    fmt::{self, Display, Formatter},
    future::Future,
    pin::Pin,
    sync::Once,
};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;

/// A HTTP transport implementation that works on wasm32 targets using the
/// native JavaScript HTTP client.
pub struct Http {
    url: JsString,
}

impl Http {
    /// Creates a new JavaScript HTTP transport.
    pub fn new(url: impl AsRef<str>) -> Self {
        let url = JsString::from(url.as_ref());
        Self { url }
    }
}

impl<'a> Transport2<'a> for Http {
    type Error = Error;
    type Call = Pin<Box<dyn Future<Output = Result<Vec<u8>, Self::Error>> + 'a>>;

    fn call(&'a self, request: &'a [u8]) -> Self::Call {
        Box::pin(async move {
            let request = Uint8Array::from(request);
            let text = JsFuture::from(
                post()
                    .call2(&JsValue::UNDEFINED, &self.url, &request)?
                    .dyn_into::<Promise>()?,
            )
            .await?
            .as_string()
            .ok_or("fetch returned non-string value")?;

            Ok(text.into_bytes())
        })
    }
}

/// Gets a handle to the JavaScript `post` function implementation. This is
/// done instead of using `#[wasm_bindgen(module = "...")]` snippets or the
/// `web-sys` crate in order to support both Web and NodeJS targets.
fn post() -> &'static Function {
    static INIT: Once = Once::new();
    static mut POST: Option<Function> = None;

    INIT.call_once(|| unsafe {
        POST = Some(
            js_sys::eval(include_str!("wasm/js/post.js"))
                .expect("failed to eval JavaScript runtime")
                .dyn_into()
                .unwrap(),
        );
    });

    unsafe { POST.as_ref().expect("panic evaluating JavaScript runtime") }
}

/// An HTTP transport error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Error(pub String);

impl Error {
    /// Creates a new unknown error when inspecting the JavaScript error type
    /// fails.
    #[doc(hidden)]
    pub fn unknown() -> Self {
        Self("unknown error".to_owned())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl std::error::Error for Error {}

impl From<JsValue> for Error {
    fn from(value: JsValue) -> Self {
        js_sys::Error::from(value)
            .message()
            .as_string()
            .map(Error)
            .unwrap_or_else(Error::unknown)
    }
}

impl<'a> From<&'a str> for Error {
    fn from(message: &str) -> Self {
        Self(message.to_owned())
    }
}
