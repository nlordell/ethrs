//! Module contains conversions for [`u256`] to and from primimitive types.

use crate::u256;
use std::convert::{TryFrom, TryInto};
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
    bool, u8, u16, u32, u64, u128,
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

/// This trait defines `as` conversions (casting) from primitive types to
/// [`u256`].
///
/// [`u256`]: struct.u256.html
///
/// # Examples
///
/// Note that in Rust casting from a negative signed integer sign to a larger
/// unsigned interger sign extends. Additionally casting a floating point value
/// to an integer is a saturating operation, with `NaN` converting to `0`. So:
///
/// ```
/// # use ethnum::{u256, AsU256};
/// assert_eq!((-1i32).as_u256(), u256::MAX);
/// assert_eq!(u32::MAX.as_u256(), 0xffffffff);
///
/// assert_eq!(f64::NEG_INFINITY.as_u256(), 0);
/// assert_eq!((-1.0f64).as_u256(), 0);
/// assert_eq!(f64::INFINITY.as_u256(), u256::MAX);
/// assert_eq!(2.0f64.powi(257).as_u256(), u256::MAX);
/// assert_eq!(f64::NAN.as_u256(), 0);
/// ```
pub trait AsU256 {
    /// Perform an `as` conversion to a [`u256`].
    ///
    /// [`u256`]: struct.u256.html
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

impl AsU256 for bool {
    #[inline]
    fn as_u256(self) -> u256 {
        if self {
            u256::ONE
        } else {
            u256::ZERO
        }
    }
}

macro_rules! impl_as_u256_float {
    ($($t:ty [$b:ty]),* $(,)?) => {$(
        impl AsU256 for $t {
            #[inline]
            fn as_u256(self) -> u256 {
                // The conversion follows roughly the same rules as converting
                // `f64` to other primitive integer types:
                // - `NaN` => `0`
                // - `(-∞, 0]` => `0`
                // - `(0, u256::MAX]` => `value as u256`
                // - `(u256::MAX, +∞)` => `u256::MAX`

                const M: u32 = <$t>::MANTISSA_DIGITS - 1;
                const MAN_MASK: $b = !(!0 << M);
                const MAN_ONE: $b = 1 << M;
                const EXP_MASK: $b = !0 >> <$t>::MANTISSA_DIGITS;
                const EXP_OFFSET: $b = EXP_MASK / 2;

                if self >= 1.0 {
                    let bits = self.to_bits();
                    let exponent = ((bits >> M) & EXP_MASK) - EXP_OFFSET;
                    let mantissa = (bits & MAN_MASK) | MAN_ONE;
                    if exponent <= 52 {
                        u256::from(mantissa >> (52 - exponent))
                    } else if exponent >= 256 {
                        u256::MAX
                    } else {
                        u256::from(mantissa) << (exponent - 52)
                    }
                } else {
                    u256::ZERO
                }
            }
        }
    )*};
}

impl_as_u256_float! {
    f32[u32], f64[u64],
}

impl TryInto<u128> for u256 {
    type Error = TryFromIntError;

    #[inline]
    fn try_into(self) -> Result<u128, Self::Error> {
        let (hi, lo) = self.into_words();
        if hi != 0 {
            // NOTE: Work around not being able to construct an error.
            (-1isize).try_into()
        } else {
            Ok(lo)
        }
    }
}

macro_rules! impl_try_into {
    ($($t:ty),* $(,)?) => {$(
        impl TryInto<$t> for u256 {
            type Error = TryFromIntError;

            #[inline]
            fn try_into(self) -> Result<$t, Self::Error> {
                let (hi, lo) = self.into_words();
                let x = if hi != 0 { u128::MAX } else { lo };
                x.try_into()
            }
        }
    )*};
}

impl_try_into! {
    i8, i16, i32, i64, i128,
    u8, u16, u32, u64,
    isize, usize,
}

macro_rules! impl_into_float {
    ($($t:ty => $f:ident),* $(,)?) => {$(
        impl Into<$t> for u256 {
            #[inline]
            fn into(self) -> $t {
                self.$f()
            }
        }
    )*};
}

impl_into_float! {
    f32 => as_f32, f64 => as_f64,
}

impl u256 {
    /// Casts a [`u256`] to a primitive [`i8`], wrapping if it would overflow
    /// the primitive integer.
    ///
    /// [`u256`]: struct.u256.html
    pub const fn as_i8(self) -> i8 {
        let (_, lo) = self.into_words();
        lo as _
    }

    /// Casts a [`u256`] to a primitive [`i16`], wrapping if it would overflow
    /// the primitive integer.
    ///
    /// [`u256`]: struct.u256.html
    pub const fn as_i16(self) -> i16 {
        let (_, lo) = self.into_words();
        lo as _
    }

    /// Casts a [`u256`] to a primitive [`i32`], wrapping if it would overflow
    /// the primitive integer.
    ///
    /// [`u256`]: struct.u256.html
    pub const fn as_i32(self) -> i32 {
        let (_, lo) = self.into_words();
        lo as _
    }

    /// Casts a [`u256`] to a primitive [`i64`], wrapping if it would overflow
    /// the primitive integer.
    ///
    /// [`u256`]: struct.u256.html
    pub const fn as_i64(self) -> i64 {
        let (_, lo) = self.into_words();
        lo as _
    }

    /// Casts a [`u256`] to a primitive [`i128`], wrapping if it would overflow
    /// the primitive integer.
    ///
    /// [`u256`]: struct.u256.html
    pub const fn as_i128(self) -> i128 {
        let (_, lo) = self.into_words();
        lo as _
    }

    /// Casts a [`u256`] to a primitive [`u8`], wrapping if it would overflow
    /// the primitive integer.
    ///
    /// [`u256`]: struct.u256.html
    pub const fn as_u8(self) -> u8 {
        let (_, lo) = self.into_words();
        lo as _
    }

    /// Casts a [`u256`] to a primitive [`u16`], wrapping if it would overflow
    /// the primitive integer.
    ///
    /// [`u256`]: struct.u256.html
    pub const fn as_u16(self) -> u16 {
        let (_, lo) = self.into_words();
        lo as _
    }

    /// Casts a [`u256`] to a primitive [`u32`], wrapping if it would overflow
    /// the primitive integer.
    ///
    /// [`u256`]: struct.u256.html
    pub const fn as_u32(self) -> u32 {
        let (_, lo) = self.into_words();
        lo as _
    }

    /// Casts a [`u256`] to a primitive [`u64`], wrapping if it would overflow
    /// the primitive integer.
    ///
    /// [`u256`]: struct.u256.html
    pub const fn as_u64(self) -> u64 {
        let (_, lo) = self.into_words();
        lo as _
    }

    /// Casts a [`u256`] to a primitive [`u128`], wrapping if it would overflow
    /// the primitive integer.
    ///
    /// [`u256`]: struct.u256.html
    pub const fn as_u128(self) -> u128 {
        let (_, lo) = self.into_words();
        lo
    }

    /// Casts a [`u256`] to a primitive [`isize`], wrapping if it would overflow
    /// the primitive integer.
    ///
    /// [`u256`]: struct.u256.html
    pub const fn as_isize(self) -> isize {
        let (_, lo) = self.into_words();
        lo as _
    }

    /// Casts a [`u256`] to a primitive [`usize`], wrapping if it would overflow
    /// the primitive integer.
    ///
    /// [`u256`]: struct.u256.html
    pub const fn as_usize(self) -> usize {
        let (_, lo) = self.into_words();
        lo as _
    }

    /// Casts a [`u256`] to a primitive [`f32`].
    ///
    /// [`u256`]: struct.u256.html
    pub fn as_f32(self) -> f32 {
        match self.into_words() {
            (0, lo) => lo as _,
            _ => f32::INFINITY,
        }
    }

    /// Casts a [`u256`] to a primitive [`f64`].
    ///
    /// [`u256`]: struct.u256.html
    pub fn as_f64(self) -> f64 {
        match self.into_words() {
            (0, lo) => lo as _,
            (hi, lo) => (hi as f64) * (2.0f64).powi(128) + (lo as f64),
        }
    }
}
