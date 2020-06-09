//! This module contains native implementations for intrinsics. These are used
//! when generated IR intrinsics are disabled.

mod add;
mod divmod;
mod mul;
mod shl;
mod shr;
mod sub;

pub use self::add::*;
pub use self::divmod::*;
pub use self::mul::{mulc, multi3 as mul3};
pub use self::shl::*;
pub use self::shr::*;
pub use self::sub::*;
