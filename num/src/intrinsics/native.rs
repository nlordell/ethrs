//! This module contains native implementations for intrinsics. These are used
//! when generated IR intrinsics are disabled.

mod divmod;
mod mul;

pub use self::divmod::*;
pub use self::mul::{mulc, multi3 as mul3};
use crate::u256;
use std::mem::MaybeUninit;

#[inline]
pub fn add2(r: &mut u256, a: &u256) {
    let (lo, carry) = r.low().overflowing_add(*a.low());
    *r.low_mut() = lo;
    *r.high_mut() = r.high().wrapping_add(carry as _).wrapping_add(*a.high());
}

#[inline]
pub fn add3(r: &mut MaybeUninit<u256>, a: &u256, b: &u256) {
    let (lo, carry) = a.low().overflowing_add(*b.low());
    let hi = a.high().wrapping_add(carry as _).wrapping_add(*b.high());

    unsafe {
        r.as_mut_ptr().write(u256::from_words(hi, lo));
    }
}

#[inline]
pub fn addc(r: &mut MaybeUninit<u256>, a: &u256, b: &u256) -> bool {
    let (lo, carry_lo) = a.low().overflowing_add(*b.low());
    let (hi, carry_c) = a.high().overflowing_add(carry_lo as _);
    let (hi, carry_hi) = hi.overflowing_add(*b.high());

    unsafe {
        r.as_mut_ptr().write(u256::from_words(hi, lo));
    }
    carry_c || carry_hi
}

#[inline]
pub fn sub2(r: &mut u256, a: &u256) {
    let (lo, carry) = r.low().overflowing_sub(*a.low());
    *r.low_mut() = lo;
    *r.high_mut() = r.high().wrapping_sub(carry as _).wrapping_sub(*a.high());
}

#[inline]
pub fn sub3(r: &mut MaybeUninit<u256>, a: &u256, b: &u256) {
    let (lo, carry) = a.low().overflowing_sub(*b.low());
    let hi = a.high().wrapping_sub(carry as _).wrapping_sub(*b.high());

    unsafe {
        r.as_mut_ptr().write(u256::from_words(hi, lo));
    }
}

#[inline]
pub fn subc(r: &mut MaybeUninit<u256>, a: &u256, b: &u256) -> bool {
    let (lo, carry_lo) = a.low().overflowing_sub(*b.low());
    let (hi, carry_c) = a.high().overflowing_sub(carry_lo as _);
    let (hi, carry_hi) = hi.overflowing_sub(*b.high());

    unsafe {
        r.as_mut_ptr().write(u256::from_words(hi, lo));
    }
    carry_c || carry_hi
}

#[inline]
pub fn shl2(r: &mut u256, a: u32) {
    let a = a & 0xff;
    let (hi, lo) = if a == 0 {
        return;
    } else if a < 128 {
        ((r.high() << a) | (r.low() >> (128 - a)), r.low() << a)
    } else {
        (r.low() << (a & 0x7f), 0)
    };

    *r = u256::from_words(hi, lo);
}

#[inline]
pub fn shl3(r: &mut MaybeUninit<u256>, a: &u256, b: u32) {
    let b = b & 0xff;
    let (hi, lo) = if b == 0 {
        (*a.high(), *a.low())
    } else if b < 128 {
        ((a.high() << b) | (a.low() >> (128 - b)), a.low() << b)
    } else {
        (a.low() << (b & 0x7f), 0)
    };

    unsafe {
        r.as_mut_ptr().write(u256::from_words(hi, lo));
    }
}

#[inline]
pub fn shr2(r: &mut u256, a: u32) {
    let a = a & 0xff;
    let (hi, lo) = if a == 0 {
        return;
    } else if a < 128 {
        ((r.high() >> a) | (r.low() << (128 - a)), r.low() >> a)
    } else {
        (r.low() >> (a & 0x7f), 0)
    };

    *r = u256::from_words(hi, lo);
}

#[inline]
pub fn shr3(r: &mut MaybeUninit<u256>, a: &u256, b: u32) {
    let b = b & 0xff;
    let (hi, lo) = if b == 0 {
        (*a.high(), *a.low())
    } else if b < 128 {
        (a.high() >> b, a.low() >> b | (a.high() << (128 - b)))
    } else {
        (0, a.high() >> (b & 0x7f))
    };

    unsafe {
        r.as_mut_ptr().write(u256::from_words(hi, lo));
    }
}
