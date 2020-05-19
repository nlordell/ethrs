mod arith;
mod builtins;
mod endian;
mod intrinsics;

use std::mem::MaybeUninit;

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

    /// Creates an uninitialized value of this integer type.
    ///
    /// This is always safe since for integer types.
    #[inline(always)]
    #[allow(clippy::uninit_assumed_init)]
    pub fn uninit() -> Self {
        unsafe { MaybeUninit::uninit().assume_init() }
    }
}

// TODO(nlordell):
// - Use `clang` with `-flto=thin` and `lld` when available, allowing the
//   inlining of the addition and subtraction routines.
// - Base arithmetic traits on `overflowing_*` and `wrapping_*`.
