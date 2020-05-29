//! Module containing integer aritimetic methods.

use crate::intrinsics;
use crate::u256;
use std::mem::MaybeUninit;

const BITS: u32 = 256;

impl u256 {
    /// Checked integer addition. Computes `self + rhs`, returning `None`
    /// if overflow occurred.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use ethrs_num::u256;
    /// assert_eq!((u256::MAX - 2).checked_add(u256::new(1)), Some(u256::MAX - 1));
    /// assert_eq!((u256::MAX - 2).checked_add(u256::new(3)), None);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        let (a, b) = self.overflowing_add(rhs);
        if b {
            None
        } else {
            Some(a)
        }
    }

    /// Checked integer subtraction. Computes `self - rhs`, returning
    /// `None` if overflow occurred.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use ethrs_num::u256;
    /// assert_eq!(u256::new(1).checked_sub(u256::new(1)), Some(u256::ZERO));
    /// assert_eq!(u256::new(0).checked_sub(u256::new(1)), None);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        let (a, b) = self.overflowing_sub(rhs);
        if b {
            None
        } else {
            Some(a)
        }
    }

    /// Checked integer multiplication. Computes `self * rhs`, returning
    /// `None` if overflow occurred.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use ethrs_num::u256;
    /// assert_eq!(u256::new(5).checked_mul(u256::new(1)), Some(u256::new(5)));
    /// assert_eq!(u256::MAX.checked_mul(u256::new(2)), None);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub fn checked_mul(self, rhs: Self) -> Option<Self> {
        let (a, b) = self.overflowing_mul(rhs);
        if b {
            None
        } else {
            Some(a)
        }
    }

    /// Checked integer division. Computes `self / rhs`, returning `None`
    /// if `rhs == 0`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```should_panic
    /// # use ethrs_num::u256;
    /// assert_eq!(u256::new(128).checked_div(u256::new(2)), Some(u256::new(64)));
    /// assert_eq!(u256::new(1).checked_div(u256::new(0)), None);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub fn checked_div(self, rhs: Self) -> Option<Self> {
        if rhs == u256::ZERO {
            None
        } else {
            todo!()
        }
    }

    /// Checked Euclidean division. Computes `self.div_euclid(rhs)`, returning `None`
    /// if `rhs == 0`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```should_panic
    /// # use ethrs_num::u256;
    /// assert_eq!(u256::new(128).checked_div_euclid(u256::new(2)), Some(u256::new(64)));
    /// assert_eq!(u256::new(1).checked_div_euclid(u256::new(0)), None);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
        if rhs == u256::ZERO {
            None
        } else {
            Some(self.div_euclid(rhs))
        }
    }

    /// Checked integer remainder. Computes `self % rhs`, returning `None`
    /// if `rhs == 0`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```should_panic
    /// # use ethrs_num::u256;
    /// assert_eq!(u256::new(5).checked_rem(u256::new(2)), Some(u256::new(1)));
    /// assert_eq!(u256::new(5).checked_rem(u256::new(0)), None);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub fn checked_rem(self, rhs: Self) -> Option<Self> {
        if rhs == u256::ZERO {
            None
        } else {
            todo!()
        }
    }

    /// Checked Euclidean modulo. Computes `self.rem_euclid(rhs)`, returning `None`
    /// if `rhs == 0`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```should_panic
    /// # use ethrs_num::u256;
    /// assert_eq!(u256::new(5).checked_rem_euclid(u256::new(2)), Some(u256::new(1)));
    /// assert_eq!(u256::new(5).checked_rem_euclid(u256::new(0)), None);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
        if rhs == u256::ZERO {
            None
        } else {
            Some(self.rem_euclid(rhs))
        }
    }

    /// Checked negation. Computes `-self`, returning `None` unless `self ==
    /// 0`.
    ///
    /// Note that negating any positive integer will overflow.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use ethrs_num::u256;
    /// assert_eq!(u256::ZERO.checked_neg(), Some(u256::ZERO));
    /// assert_eq!(u256::new(1).checked_neg(), None);
    /// ```
    #[inline]
    pub fn checked_neg(self) -> Option<Self> {
        let (a, b) = self.overflowing_neg();
        if b {
            None
        } else {
            Some(a)
        }
    }

    /// Checked shift left. Computes `self << rhs`, returning `None`
    /// if `rhs` is larger than or equal to the number of bits in `self`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use ethrs_num::u256;
    /// assert_eq!(u256::new(0x1).checked_shl(4), Some(u256::new(0x10)));
    /// assert_eq!(u256::new(0x10).checked_shl(257), None);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub fn checked_shl(self, rhs: u32) -> Option<Self> {
        let (a, b) = self.overflowing_shl(rhs);
        if b {
            None
        } else {
            Some(a)
        }
    }

    /// Checked shift right. Computes `self >> rhs`, returning `None`
    /// if `rhs` is larger than or equal to the number of bits in `self`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use ethrs_num::u256;
    /// assert_eq!(u256::new(0x10).checked_shr(4), Some(u256::new(0x1)));
    /// assert_eq!(u256::new(0x10).checked_shr(257), None);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub fn checked_shr(self, rhs: u32) -> Option<Self> {
        let (a, b) = self.overflowing_shr(rhs);
        if b {
            None
        } else {
            Some(a)
        }
    }

    /// Checked exponentiation. Computes `self.pow(exp)`, returning `None` if
    /// overflow occurred.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```should_panic
    /// # use ethrs_num::u256;
    /// assert_eq!(u256::new(2).checked_pow(5), Some(u256::new(32)));
    /// assert_eq!(u256::MAX.checked_pow(2), None);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub fn checked_pow(self, mut _exp: u32) -> Option<Self> {
        /*
        let mut base = self;
        let mut acc: Self = 1;

        while exp > 1 {
            if (exp & 1) == 1 {
                acc = try_opt!(acc.checked_mul(base));
            }
            exp /= 2;
            base = try_opt!(base.checked_mul(base));
        }

        // Deal with the final bit of the exponent separately, since
        // squaring the base afterwards is not necessary and may cause a
        // needless overflow.
        if exp == 1 {
            acc = try_opt!(acc.checked_mul(base));
        }

        Some(acc)
        */
        todo!()
    }

    /// Saturating integer addition. Computes `self + rhs`, saturating at
    /// the numeric bounds instead of overflowing.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use ethrs_num::u256;
    /// assert_eq!(u256::new(100).saturating_add(u256::new(1)), u256::new(101));
    /// assert_eq!(u256::MAX.saturating_add(u256::new(127)), u256::MAX);
    /// ```

    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub fn saturating_add(self, rhs: Self) -> Self {
        self.checked_add(rhs).unwrap_or(u256::MAX)
    }

    /// Saturating integer subtraction. Computes `self - rhs`, saturating
    /// at the numeric bounds instead of overflowing.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use ethrs_num::u256;
    /// assert_eq!(u256::new(100).saturating_sub(u256::new(27)), u256::new(73));
    /// assert_eq!(u256::new(13).saturating_sub(u256::new(127)), u256::new(0));
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub fn saturating_sub(self, rhs: Self) -> Self {
        self.checked_sub(rhs).unwrap_or(u256::MIN)
    }

    /// Saturating integer multiplication. Computes `self * rhs`,
    /// saturating at the numeric bounds instead of overflowing.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use ethrs_num::u256;
    /// assert_eq!(u256::new(2).saturating_mul(u256::new(10)), u256::new(20));
    /// assert_eq!((u256::MAX).saturating_mul(u256::new(10)), u256::MAX);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub fn saturating_mul(self, rhs: Self) -> Self {
        match self.checked_mul(rhs) {
            Some(x) => x,
            None => Self::MAX,
        }
    }

    /// Saturating integer exponentiation. Computes `self.pow(exp)`,
    /// saturating at the numeric bounds instead of overflowing.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```should_panic
    /// # use ethrs_num::u256;
    /// assert_eq!(u256::new(4).saturating_pow(3), u256::new(64));
    /// assert_eq!(u256::MAX.saturating_pow(2), u256::MAX);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub fn saturating_pow(self, exp: u32) -> Self {
        match self.checked_pow(exp) {
            Some(x) => x,
            None => Self::MAX,
        }
    }

    /// Wrapping (modular) addition. Computes `self + rhs`,
    /// wrapping around at the boundary of the type.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use ethrs_num::u256;
    /// assert_eq!(u256::new(200).wrapping_add(u256::new(55)), u256::new(255));
    /// assert_eq!(u256::new(200).wrapping_add(u256::MAX), u256::new(199));
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub fn wrapping_add(self, rhs: Self) -> Self {
        let mut result = MaybeUninit::uninit();
        intrinsics::add3(&mut result, &self, &rhs);
        unsafe { result.assume_init() }
    }

    /// Wrapping (modular) subtraction. Computes `self - rhs`,
    /// wrapping around at the boundary of the type.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use ethrs_num::u256;
    /// assert_eq!(u256::new(100).wrapping_sub(u256::new(100)), u256::new(0));
    /// assert_eq!(u256::new(100).wrapping_sub(u256::MAX), u256::new(101));
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub fn wrapping_sub(self, rhs: Self) -> Self {
        let mut result = MaybeUninit::uninit();
        intrinsics::sub3(&mut result, &self, &rhs);
        unsafe { result.assume_init() }
    }

    /// Wrapping (modular) multiplication. Computes `self *
    /// rhs`, wrapping around at the boundary of the type.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// Please note that this example is shared between integer types.
    /// Which explains why `u8` is used here.
    ///
    /// ```
    /// # use ethrs_num::u256;
    /// assert_eq!(u256::new(10).wrapping_mul(u256::new(12)), u256::new(120));
    /// assert_eq!(u256::MAX.wrapping_mul(u256::new(2)), u256::MAX - 1);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub fn wrapping_mul(self, rhs: Self) -> Self {
        let mut result = MaybeUninit::uninit();
        intrinsics::mul3(&mut result, &self, &rhs);
        unsafe { result.assume_init() }
    }

    /// Wrapping (modular) division. Computes `self / rhs`.
    /// Wrapped division on unsigned types is just normal division.
    /// There's no way wrapping could ever happen.
    /// This function exists, so that all operations
    /// are accounted for in the wrapping operations.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```should_panic
    /// # use ethrs_num::u256;
    /// assert_eq!(u256::new(100).wrapping_div(u256::new(10)), u256::new(10));
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub fn wrapping_div(self, rhs: Self) -> Self {
        self / rhs
    }

    /// Wrapping Euclidean division. Computes `self.div_euclid(rhs)`.
    /// Wrapped division on unsigned types is just normal division.
    /// There's no way wrapping could ever happen.
    /// This function exists, so that all operations
    /// are accounted for in the wrapping operations.
    /// Since, for the positive integers, all common
    /// definitions of division are equal, this
    /// is exactly equal to `self.wrapping_div(rhs)`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```should_panic
    /// # use ethrs_num::u256;
    /// assert_eq!(u256::new(100).wrapping_div_euclid(u256::new(10)), u256::new(10));
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub fn wrapping_div_euclid(self, rhs: Self) -> Self {
        self / rhs
    }

    /// Wrapping (modular) remainder. Computes `self % rhs`.
    /// Wrapped remainder calculation on unsigned types is
    /// just the regular remainder calculation.
    /// There's no way wrapping could ever happen.
    /// This function exists, so that all operations
    /// are accounted for in the wrapping operations.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```should_panic
    /// # use ethrs_num::u256;
    /// assert_eq!(u256::new(100).wrapping_rem(u256::new(10)), u256::new(0));
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub fn wrapping_rem(self, rhs: Self) -> Self {
        self % rhs
    }

    /// Wrapping Euclidean modulo. Computes `self.rem_euclid(rhs)`.
    /// Wrapped modulo calculation on unsigned types is
    /// just the regular remainder calculation.
    /// There's no way wrapping could ever happen.
    /// This function exists, so that all operations
    /// are accounted for in the wrapping operations.
    /// Since, for the positive integers, all common
    /// definitions of division are equal, this
    /// is exactly equal to `self.wrapping_rem(rhs)`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```should_panic
    /// # use ethrs_num::u256;
    /// assert_eq!(u256::new(100).wrapping_rem_euclid(u256::new(10)), u256::new(0));
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub fn wrapping_rem_euclid(self, rhs: Self) -> Self {
        self % rhs
    }

    /// Wrapping (modular) negation. Computes `-self`,
    /// wrapping around at the boundary of the type.
    ///
    /// Since unsigned types do not have negative equivalents
    /// all applications of this function will wrap (except for `-0`).
    /// For values smaller than the corresponding signed type's maximum
    /// the result is the same as casting the corresponding signed value.
    /// Any larger values are equivalent to `MAX + 1 - (val - MAX - 1)` where
    /// `MAX` is the corresponding signed type's maximum.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// Please note that this example is shared between integer types.
    /// Which explains why `i8` is used here.
    ///
    /// ```
    /// # use ethrs_num::{u256, AsU256};
    /// assert_eq!(u256::new(100).wrapping_neg(), (-100i128).as_u256());
    /// assert_eq!(
    ///     u256::from_words(i128::MIN as _, 0).wrapping_neg(),
    ///     u256::from_words(i128::MIN as _, 0),
    /// );
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub fn wrapping_neg(self) -> Self {
        self.overflowing_neg().0
    }

    /// Panic-free bitwise shift-left; yields `self << mask(rhs)`,
    /// where `mask` removes any high-order bits of `rhs` that
    /// would cause the shift to exceed the bitwidth of the type.
    ///
    /// Note that this is *not* the same as a rotate-left; the
    /// RHS of a wrapping shift-left is restricted to the range
    /// of the type, rather than the bits shifted out of the LHS
    /// being returned to the other end. The primitive integer
    /// types all implement a `rotate_left` function, which may
    /// be what you want instead.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use ethrs_num::u256;
    /// assert_eq!(u256::new(1).wrapping_shl(7), u256::new(128));
    /// assert_eq!(u256::new(1).wrapping_shl(128), u256::new(1));
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub fn wrapping_shl(self, rhs: u32) -> Self {
        let mut result = MaybeUninit::uninit();
        intrinsics::shl3(&mut result, &self, rhs);
        unsafe { result.assume_init() }
    }

    /// Panic-free bitwise shift-right; yields `self >> mask(rhs)`,
    /// where `mask` removes any high-order bits of `rhs` that
    /// would cause the shift to exceed the bitwidth of the type.
    ///
    /// Note that this is *not* the same as a rotate-right; the
    /// RHS of a wrapping shift-right is restricted to the range
    /// of the type, rather than the bits shifted out of the LHS
    /// being returned to the other end. The primitive integer
    /// types all implement a `rotate_right` function, which may
    /// be what you want instead.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use ethrs_num::u256;
    /// assert_eq!(u256::new(128).wrapping_shr(7), u256::new(1));
    /// assert_eq!(u256::new(128).wrapping_shr(128), u256::new(128));
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub fn wrapping_shr(self, rhs: u32) -> Self {
        let mut result = MaybeUninit::uninit();
        intrinsics::shr3(&mut result, &self, rhs);
        unsafe { result.assume_init() }
    }

    /// Wrapping (modular) exponentiation. Computes `self.pow(exp)`,
    /// wrapping around at the boundary of the type.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```should_panic
    /// # use ethrs_num::u256;
    /// assert_eq!(u256::new(3).wrapping_pow(5), u256::new(243));
    /// assert_eq!(u256::new(3).wrapping_pow(6), u256::new(217));
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub fn wrapping_pow(self, mut _exp: u32) -> Self {
        /*
        let mut base = self;
        let mut acc: Self = 1;

        while exp > 1 {
            if (exp & 1) == 1 {
                acc = acc.wrapping_mul(base);
            }
            exp /= 2;
            base = base.wrapping_mul(base);
        }

        // Deal with the final bit of the exponent separately, since
        // squaring the base afterwards is not necessary and may cause a
        // needless overflow.
        if exp == 1 {
            acc = acc.wrapping_mul(base);
        }

        acc
        */
        todo!()
    }

    /// Calculates `self` + `rhs`
    ///
    /// Returns a tuple of the addition along with a boolean indicating
    /// whether an arithmetic overflow would occur. If an overflow would
    /// have occurred then the wrapped value is returned.
    ///
    /// # Examples
    ///
    /// Basic usage
    ///
    /// ```
    /// # use ethrs_num::u256;
    /// assert_eq!(u256::new(5).overflowing_add(u256::new(2)), (u256::new(7), false));
    /// assert_eq!(u256::MAX.overflowing_add(u256::new(1)), (u256::new(0), true));
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        let mut result = MaybeUninit::uninit();
        let overflow = intrinsics::addc(&mut result, &self, &rhs);
        (unsafe { result.assume_init() }, overflow)
    }

    /// Calculates `self` - `rhs`
    ///
    /// Returns a tuple of the subtraction along with a boolean indicating
    /// whether an arithmetic overflow would occur. If an overflow would
    /// have occurred then the wrapped value is returned.
    ///
    /// # Examples
    ///
    /// Basic usage
    ///
    /// ```
    /// # use ethrs_num::u256;
    /// assert_eq!(u256::new(5).overflowing_sub(u256::new(2)), (u256::new(3), false));
    /// assert_eq!(u256::new(0).overflowing_sub(u256::new(1)), (u256::MAX, true));
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        let mut result = MaybeUninit::uninit();
        let overflow = intrinsics::subc(&mut result, &self, &rhs);
        (unsafe { result.assume_init() }, overflow)
    }

    /// Calculates the multiplication of `self` and `rhs`.
    ///
    /// Returns a tuple of the multiplication along with a boolean
    /// indicating whether an arithmetic overflow would occur. If an
    /// overflow would have occurred then the wrapped value is returned.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// Please note that this example is shared between integer types.
    /// Which explains why `u32` is used here.
    ///
    /// ```
    /// # use ethrs_num::u256;
    /// assert_eq!(u256::new(5).overflowing_mul(u256::new(2)), (u256::new(10), false));
    /// assert_eq!(
    ///     u256::MAX.overflowing_mul(u256::new(2)),
    ///     (u256::MAX - 1, true),
    /// );
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        let mut result = MaybeUninit::uninit();
        let overflow = intrinsics::mulc(&mut result, &self, &rhs);
        (unsafe { result.assume_init() }, overflow)
    }

    /// Calculates the divisor when `self` is divided by `rhs`.
    ///
    /// Returns a tuple of the divisor along with a boolean indicating
    /// whether an arithmetic overflow would occur. Note that for unsigned
    /// integers overflow never occurs, so the second value is always
    /// `false`.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs` is 0.
    ///
    /// # Examples
    ///
    /// Basic usage
    ///
    /// ```should_panic
    /// # use ethrs_num::u256;
    /// assert_eq!(u256::new(5).overflowing_div(u256::new(2)), (u256::new(2), false));
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub fn overflowing_div(self, rhs: Self) -> (Self, bool) {
        (self / rhs, false)
    }

    /// Calculates the quotient of Euclidean division `self.div_euclid(rhs)`.
    ///
    /// Returns a tuple of the divisor along with a boolean indicating
    /// whether an arithmetic overflow would occur. Note that for unsigned
    /// integers overflow never occurs, so the second value is always
    /// `false`.
    /// Since, for the positive integers, all common
    /// definitions of division are equal, this
    /// is exactly equal to `self.overflowing_div(rhs)`.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs` is 0.
    ///
    /// # Examples
    ///
    /// Basic usage
    ///
    /// ```should_panic
    /// # use ethrs_num::u256;
    /// assert_eq!(u256::new(5).overflowing_div_euclid(u256::new(2)), (u256::new(2), false));
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
        (self / rhs, false)
    }

    /// Calculates the remainder when `self` is divided by `rhs`.
    ///
    /// Returns a tuple of the remainder after dividing along with a boolean
    /// indicating whether an arithmetic overflow would occur. Note that for
    /// unsigned integers overflow never occurs, so the second value is
    /// always `false`.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs` is 0.
    ///
    /// # Examples
    ///
    /// Basic usage
    ///
    /// ```should_panic
    /// # use ethrs_num::u256;
    /// assert_eq!(u256::new(5).overflowing_rem(u256::new(2)), (u256::new(1), false));
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
        (self % rhs, false)
    }

    /// Calculates the remainder `self.rem_euclid(rhs)` as if by Euclidean division.
    ///
    /// Returns a tuple of the modulo after dividing along with a boolean
    /// indicating whether an arithmetic overflow would occur. Note that for
    /// unsigned integers overflow never occurs, so the second value is
    /// always `false`.
    /// Since, for the positive integers, all common
    /// definitions of division are equal, this operation
    /// is exactly equal to `self.overflowing_rem(rhs)`.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs` is 0.
    ///
    /// # Examples
    ///
    /// Basic usage
    ///
    /// ```should_panic
    /// # use ethrs_num::u256;
    /// assert_eq!(u256::new(5).overflowing_rem_euclid(u256::new(2)), (u256::new(1), false));
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
        (self % rhs, false)
    }

    /// Negates self in an overflowing fashion.
    ///
    /// Returns `!self + 1` using wrapping operations to return the value
    /// that represents the negation of this unsigned value. Note that for
    /// positive unsigned values overflow always occurs, but negating 0 does
    /// not overflow.
    ///
    /// # Examples
    ///
    /// Basic usage
    ///
    /// ```
    /// # use ethrs_num::{u256, AsU256};
    /// assert_eq!(u256::new(0).overflowing_neg(), (u256::new(0), false));
    /// assert_eq!(u256::new(2).overflowing_neg(), ((-2i32).as_u256(), true));
    /// ```
    #[inline]
    pub fn overflowing_neg(self) -> (Self, bool) {
        ((!self).wrapping_add(u256::ONE), self != u256::ZERO)
    }

    /// Shifts self left by `rhs` bits.
    ///
    /// Returns a tuple of the shifted version of self along with a boolean
    /// indicating whether the shift value was larger than or equal to the
    /// number of bits. If the shift value is too large, then value is
    /// masked (N-1) where N is the number of bits, and this value is then
    /// used to perform the shift.
    ///
    /// # Examples
    ///
    /// Basic usage
    ///
    /// ```
    /// # use ethrs_num::u256;
    /// assert_eq!(u256::new(0x1).overflowing_shl(4), (u256::new(0x10), false));
    /// assert_eq!(u256::new(0x1).overflowing_shl(260), (u256::new(0x10), true));
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub fn overflowing_shl(self, rhs: u32) -> (Self, bool) {
        (self.wrapping_shl(rhs), (rhs > (BITS - 1)))
    }

    /// Shifts self right by `rhs` bits.
    ///
    /// Returns a tuple of the shifted version of self along with a boolean
    /// indicating whether the shift value was larger than or equal to the
    /// number of bits. If the shift value is too large, then value is
    /// masked (N-1) where N is the number of bits, and this value is then
    /// used to perform the shift.
    ///
    /// # Examples
    ///
    /// Basic usage
    ///
    /// ```
    /// # use ethrs_num::u256;
    /// assert_eq!(u256::new(0x10).overflowing_shr(4), (u256::new(0x1), false));
    /// assert_eq!(u256::new(0x10).overflowing_shr(260), (u256::new(0x1), true));
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub fn overflowing_shr(self, rhs: u32) -> (Self, bool) {
        (self.wrapping_shr(rhs), (rhs > (BITS - 1)))
    }

    /// Raises self to the power of `exp`, using exponentiation by squaring.
    ///
    /// Returns a tuple of the exponentiation along with a bool indicating
    /// whether an overflow happened.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```should_panic
    /// # use ethrs_num::u256;
    /// assert_eq!(u256::new(3).overflowing_pow(5), (u256::new(243), false));
    /// assert_eq!(u256::new(3).overflowing_pow(6), (u256::new(217), true));
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub fn overflowing_pow(self, mut _exp: u32) -> (Self, bool) {
        /*
        let mut base = self;
        let mut acc: Self = 1;
        let mut overflown = false;
        // Scratch space for storing results of overflowing_mul.
        let mut r;

        while exp > 1 {
            if (exp & 1) == 1 {
                r = acc.overflowing_mul(base);
                acc = r.0;
                overflown |= r.1;
            }
            exp /= 2;
            r = base.overflowing_mul(base);
            base = r.0;
            overflown |= r.1;
        }

        // Deal with the final bit of the exponent separately, since
        // squaring the base afterwards is not necessary and may cause a
        // needless overflow.
        if exp == 1 {
            r = acc.overflowing_mul(base);
            acc = r.0;
            overflown |= r.1;
        }

        (acc, overflown)
        */
        todo!()
    }

    /// Raises self to the power of `exp`, using exponentiation by squaring.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```should_panic
    /// # use ethrs_num::u256;
    /// assert_eq!(u256::new(2).pow(5), u256::new(32));
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub fn pow(self, mut _exp: u32) -> Self {
        /*
        let mut base = self;
        let mut acc = 1;

        while exp > 1 {
            if (exp & 1) == 1 {
                acc = acc * base;
            }
            exp /= 2;
            base = base * base;
        }

        // Deal with the final bit of the exponent separately, since
        // squaring the base afterwards is not necessary and may cause a
        // needless overflow.
        if exp == 1 {
            acc = acc * base;
        }

        acc
        */
        todo!()
    }

    /// Performs Euclidean division.
    ///
    /// Since, for the positive integers, all common
    /// definitions of division are equal, this
    /// is exactly equal to `self / rhs`.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs` is 0.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```should_panic
    /// # use ethrs_num::u256;
    /// assert_eq!(u256::new(7).div_euclid(u256::new(4)), u256::new(1));
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub fn div_euclid(self, rhs: Self) -> Self {
        self / rhs
    }

    /// Calculates the least remainder of `self (mod rhs)`.
    ///
    /// Since, for the positive integers, all common
    /// definitions of division are equal, this
    /// is exactly equal to `self % rhs`.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs` is 0.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```should_panic
    /// # use ethrs_num::u256;
    /// assert_eq!(u256::new(7).rem_euclid(u256::new(4)), u256::new(3));
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub fn rem_euclid(self, rhs: Self) -> Self {
        self % rhs
    }
}
