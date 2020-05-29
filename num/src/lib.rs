mod arith;
mod endian;
pub mod intrinsics;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct u256(pub [u128; 2]);

pub struct ParseIntError;

impl u256 {
    /// The smallest value that can be represented by this integer type.
    pub const MIN: Self = u256([0; 2]);

    /// The largest value that can be represented by this integer type.
    pub const MAX: Self = u256([!0; 2]);

    /// Creates a 0-valued integer.
    #[inline(always)]
    pub const fn zero() -> Self {
        u256::new(0)
    }

    /// Creates a new 256-bit integer value from a primitive `u128` integer.
    #[inline(always)]
    pub const fn new(value: u128) -> Self {
        u256::from_words(0, value)
    }
}

impl From<u128> for u256 {
    #[inline(always)]
    fn from(value: u128) -> Self {
        u256::new(value)
    }
}

// TODO(nlordell):
// - Base arithmetic traits on `overflowing_*` and `wrapping_*`.
