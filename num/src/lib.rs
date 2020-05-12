#[macro_use]
mod arith;
mod intrinsics;

use std::mem::MaybeUninit;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct u256(pub [u64; 4]);

pub struct ParseIntError;

impl u256 {
    /// The smallest value that can be represented by this integer type.
    pub const MIN: Self = u256([0; 4]);

    /// The largest value that can be represented by this integer type.
    pub const MAX: Self = u256([!0; 4]);

    /// Converts a string slice in a given base to an integer.
    ///
    /// The string is expected to be an optional `+` sign followed by digits.
    /// Leading and trailing whitespace represent an error. Digits are a subset
    /// of these characters, depending on `radix`:
    ///
    /// * `0-9`
    /// * `a-z`
    /// * `A-Z`
    ///
    /// # Panics
    ///
    /// This function panics if `radix` is not in the range from 2 to 36.
    pub fn from_str_radix(_src: &str, _radix: u32) -> Result<Self, ParseIntError> {
        todo!();
    }
}

impl u256 {
    /// Creates an uninitialized value of this integer type.
    ///
    /// This is always safe since for integer types.
    #[inline]
    #[allow(clippy::uninit_assumed_init)]
    pub fn uninit() -> Self {
        unsafe { MaybeUninit::uninit().assume_init() }
    }
}

impl_binops! {
    Add { add, add_overflow "add with overflow" }
    Mul { mul, mul_overflow "multiply with overflow" }
    Sub { sub, sub_overflow "subtract with overflow" }
}

impl From<u128> for u256 {
    #[inline]
    fn from(value: u128) -> Self {
        u256([value as _, (value >> 64) as _, 0, 0])
    }
}
