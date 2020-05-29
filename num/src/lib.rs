mod arith;
mod convert;
mod endian;
pub mod intrinsics;
mod ops;

pub use self::convert::AsU256;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct u256(pub [u128; 2]);

impl u256 {
    /// The smallest value that can be represented by this integer type.
    pub const MIN: Self = u256([0; 2]);

    /// The largest value that can be represented by this integer type.
    pub const MAX: Self = u256([!0; 2]);

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
