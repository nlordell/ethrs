//! Module contains conversions for [`u256`] to and from primimitive types.

use crate::u256;
use std::convert::TryFrom;
use std::num::TryFromIntError;

macro_rules! impl_from {
    ($($t:ty),* $(,)?) => {$(
        impl From<$t> for u256 {
            #[inline]
            fn from(value: $t) -> Self {
                u256::new(value.into())
            }
        }
    )*};
}

impl_from! {
    u8, u16, u32, u64, u128,
}

macro_rules! impl_try_from {
    ($($t:ty),* $(,)?) => {$(
        impl TryFrom<$t> for u256 {
            type Error = TryFromIntError;

            #[inline]
            fn try_from(value: $t) -> Result<Self, Self::Error> {
                Ok(u256::new(u128::try_from(value)?))
            }
        }
    )*};
}

impl_try_from! {
    i8, i16, i32, i64, i128,
    isize, usize,
}

/// This trait defines `as` conversions from primitive types to [`u256`].
pub trait AsU256 {
    fn as_u256(self) -> u256;
}

impl AsU256 for u256 {
    #[inline]
    fn as_u256(self) -> u256 {
        self
    }
}

macro_rules! impl_as_u256 {
    ($($t:ty),* $(,)?) => {$(
        impl AsU256 for $t {
            #[inline]
            fn as_u256(self) -> u256 {
                #[allow(unused_comparisons)]
                let hi = if self >= 0 { 0 } else { !0 };
                u256::from_words(hi, self as _)
            }
        }
    )*};
}

impl_as_u256! {
    i8, i16, i32, i64, i128,
    u8, u16, u32, u64, u128,
    isize, usize,
}
