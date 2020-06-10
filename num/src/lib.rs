mod cmp;
mod convert;
mod endian;
pub mod intrinsics;
mod iter;
mod ops;
mod uint;

pub use self::convert::AsU256;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq)]
#[repr(transparent)]
pub struct u256(pub [u128; 2]);

impl u256 {
    /// The additive identity for this integer type, i.e. `0`.
    pub const ZERO: Self = u256([0; 2]);

    /// The multiplicative identity for this integer type, i.e. `1`.
    pub const ONE: Self = u256::new(1);

    /// Creates a new 256-bit integer value from a primitive `u128` integer.
    #[inline]
    pub const fn new(value: u128) -> Self {
        u256::from_words(0, value)
    }
}

// TODO(nlordell):
//
// # Trait Implementations
//
// - Debug
// - Display
// - FromStr
//
// - Binary
// - LowerExp
// - LowerHex
// - Octal
// - UpperExp
// - UpperHex
