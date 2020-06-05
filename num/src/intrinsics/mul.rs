//! This module contains a Rust port of the `__multi3` compiler builtin that is
//! typically used for implementing 64-bit multiplication on 32-bit platforms.
//!
//! This port is adapted to use 128-bit high and low words and return carry
//! information in order to implement 256-bit overflowing multiplication.
//!
//! This source is ported from LLVM project from C:
//! https://github.com/llvm/llvm-project/blob/master/compiler-rt/lib/builtins/multi3.c

use crate::u256;
use std::mem::MaybeUninit;

#[inline(always)]
fn mulddi3(a: &u128, b: &u128) -> u256 {
    let mut high;
    let mut low;

    const BITS_IN_DWORD_2: u32 = 64;
    const LOWER_MASK: u128 = u128::MAX >> BITS_IN_DWORD_2;

    low = (a & LOWER_MASK) * (b & LOWER_MASK);
    let mut t = low >> BITS_IN_DWORD_2;
    low &= LOWER_MASK;
    t += (a >> BITS_IN_DWORD_2) * (b & LOWER_MASK);
    low += (t & LOWER_MASK) << BITS_IN_DWORD_2;
    high = t >> BITS_IN_DWORD_2;
    t = low >> BITS_IN_DWORD_2;
    low &= LOWER_MASK;
    t += (b >> BITS_IN_DWORD_2) * (a & LOWER_MASK);
    low += (t & LOWER_MASK) << BITS_IN_DWORD_2;
    high += t >> BITS_IN_DWORD_2;
    high += (a >> BITS_IN_DWORD_2) * (b >> BITS_IN_DWORD_2);

    u256::from_words(high, low)
}

#[inline]
pub fn multi3(res: &mut MaybeUninit<u256>, a: &u256, b: &u256) {
    let mut r = mulddi3(a.low(), b.low());

    let hi_lo = a.high().wrapping_mul(*b.low());
    let lo_hi = a.low().wrapping_mul(*b.high());
    *r.high_mut() += hi_lo.wrapping_add(lo_hi);

    unsafe {
        res.as_mut_ptr().write(r);
    }
}

#[inline]
pub fn mulc(res: &mut MaybeUninit<u256>, a: &u256, b: &u256) -> bool {
    let mut r = mulddi3(a.low(), b.low());

    let (hi_lo, overflow_hi_lo) = a.high().overflowing_mul(*b.low());
    let (lo_hi, overflow_lo_hi) = a.low().overflowing_mul(*b.high());
    let (high, overflow_high) = hi_lo.overflowing_add(lo_hi);
    *r.high_mut() += high;

    unsafe {
        res.as_mut_ptr().write(r);
    }
    overflow_hi_lo | overflow_lo_hi | overflow_high
}
