//! This module contains a collection of Rust implemented "builtins" for the
//! 256-bit integer type.

mod udivmod;

pub use self::udivmod::udivmodti4;
