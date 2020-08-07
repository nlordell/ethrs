//! Simple HTTP transport implementation. Uses `curl` for standard targets and
//! `fetch` with `web-sys` for Wasm target.

#[cfg(not(target_arch = "wasm32"))]
#[path = "curl.rs"]
mod platform;
#[cfg(target_arch = "wasm32")]
#[path = "wasm.rs"]
mod platform;

pub use platform::*;
