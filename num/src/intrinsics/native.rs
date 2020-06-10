//! This module contains native implementations for intrinsics. These are used
//! when generated IR intrinsics are disabled.

mod add;
mod ctz;
mod divmod;
mod mul;
mod rotate;
mod shl;
mod shr;
mod sub;

pub use self::add::*;
pub use self::ctz::*;
pub use self::divmod::*;
pub use self::mul::{mul2, mulc, multi3 as mul3};
pub use self::rotate::*;
pub use self::shl::*;
pub use self::shr::*;
pub use self::sub::*;
