#[macro_use]
mod arith;
mod builtins;
mod intrinsics;

use std::mem::MaybeUninit;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct u256(pub [u128; 2]);

pub struct ParseIntError;

impl u256 {
    /// The smallest value that can be represented by this integer type.
    pub const MIN: Self = u256([0; 2]);

    /// The largest value that can be represented by this integer type.
    pub const MAX: Self = u256([!0; 2]);

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

#[cfg(target_endian = "little")]
const LO: usize = 0;
#[cfg(target_endian = "big")]
const LO: usize = 1;
#[cfg(target_endian = "little")]
const HI: usize = 1;
#[cfg(target_endian = "big")]
const HI: usize = 0;

impl u256 {
    /// Creates a new 256-bit integer value from a primitive `u128` integer.
    #[inline(always)]
    pub const fn new(value: u128) -> Self {
        #[cfg(target_endian = "little")]
        {
            u256([value, 0])
        }
        #[cfg(target_endian = "big")]
        {
            u256([0, value])
        }
    }

    /// Creates a 0-valued integer.
    #[inline(always)]
    pub const fn zero() -> Self {
        u256::new(0)
    }

    /// Creates an uninitialized value of this integer type.
    ///
    /// This is always safe since for integer types.
    #[inline(always)]
    #[allow(clippy::uninit_assumed_init)]
    pub fn uninit() -> Self {
        unsafe { MaybeUninit::uninit().assume_init() }
    }

    /// Get the low 128-bit word for this unsigned integer.
    #[inline(always)]
    pub fn low(&self) -> &u128 {
        &self.0[LO]
    }

    /// Get the low 128-bit word for this unsigned integer as a mutable
    /// reference.
    #[inline(always)]
    pub fn low_mut(&self) -> &mut u128 {
        &mut self.0[LO]
    }

    /// Get the high 128-bit word for this unsigned integer.
    #[inline(always)]
    pub fn high(&self) -> &u128 {
        &self.0[HI]
    }

    /// Get the high 128-bit word for this unsigned integer as a mutable
    /// reference.
    #[inline(always)]
    pub fn high_mut(&mut self) -> &mut u128 {
        &mut self.0[HI]
    }
}

impl_binops! {
    Add { add, add_overflow "add with overflow" }
    Mul { mul, mul_overflow "multiply with overflow" }
    Sub { sub, sub_overflow "subtract with overflow" }
}
