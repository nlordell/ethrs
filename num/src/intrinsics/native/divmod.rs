//! This module contains a Rust port of the `__udivmodti4` compiler builtin that
//! is typically used for implementing 64-bit unsigned division on 32-bit
//! platforms.
//!
//! This port is adapted to use 128-bit high and low words in order to implement
//! 256-bit unsigned division.
//!
//! This source is ported from LLVM project from C:
//! https://github.com/llvm/llvm-project/blob/master/compiler-rt/lib/builtins/udivmodti4.c

use crate::u256;
use std::mem::MaybeUninit;

#[allow(warnings)]
pub fn udivmodti4(
    res: &mut MaybeUninit<u256>,
    a: &u256,
    b: &u256,
    rem: Option<&mut MaybeUninit<u256>>,
) {
    macro_rules! set {
        ($x:ident = $value:expr) => {
            unsafe { $x.as_mut_ptr().write($value) }
        };
    }

    const n_udword_bits: u32 = 128;
    const n_utword_bits: u32 = 256;
    let n = a;
    let d = b;
    let mut q = MaybeUninit::<u256>::uninit();
    let mut r = MaybeUninit::<u256>::uninit();
    let mut sr;
    // special cases, X is unknown, K != 0
    if *n.high() == 0 {
        if *d.high() == 0 {
            // 0 X
            // ---
            // 0 X
            if let Some(rem) = rem {
                unsafe { rem.as_mut_ptr().write(u256::new(n.low() % d.low())) };
            }
            set!(res = u256::new(n.low() / d.low()));
            return;
        }
        // 0 X
        // ---
        // K X
        if let Some(rem) = rem {
            unsafe { rem.as_mut_ptr().write(u256::new(*n.low())) };
        }
        set!(res = u256::ZERO);
        return;
    }
    // n.high() != 0
    if *d.low() == 0 {
        if *d.high() == 0 {
            // K X
            // ---
            // 0 0
            if let Some(rem) = rem {
                unsafe { rem.as_mut_ptr().write(u256::new(n.high() % d.low())) };
            }
            set!(res = u256::new(n.high() / d.low()));
            return;
        }
        // d.high() != 0
        if *n.low() == 0 {
            // K 0
            // ---
            // K 0
            if let Some(rem) = rem {
                unsafe {
                    rem.as_mut_ptr()
                        .write(u256::from_words(n.high() % d.high(), 0))
                };
            }
            set!(res = u256::new(n.high() / d.high()));
            return;
        }
        // K K
        // ---
        // K 0
        if (d.high() & (d.high() - 1)) == 0
        /* if d is a power of 2 */
        {
            if let Some(rem) = rem {
                unsafe {
                    rem.as_mut_ptr()
                        .write(u256::from_words(*n.low(), n.high() & (d.high() - 1)))
                };
            }
            set!(res = u256::new(n.high() >> d.high().trailing_zeros()));
            return;
        }
        // K K
        // ---
        // K 0
        sr = d.high().leading_zeros() - n.high().leading_zeros();
        // 0 <= sr <= n_udword_bits - 2 or sr large
        if sr > n_udword_bits - 2 {
            if let Some(rem) = rem {
                unsafe { rem.as_mut_ptr().write(*n) };
            }
            set!(res = u256::ZERO);
            return;
        }
        sr += 1;
        // 1 <= sr <= n_udword_bits - 1
        // q.all = n.all << (n_utword_bits - sr);
        set!(q = u256::from_words(n.low() << (n_udword_bits - sr), 0));
        // r.all = n.all >> sr;
        set!(
            r = u256::from_words(
                n.high() >> sr,
                (n.high() << (n_udword_bits - sr)) | (n.low() >> sr),
            )
        );
    } else {
        /* d.low() != 0 */
        if *d.high() == 0 {
            // K X
            // ---
            // 0 K
            if (d.low() & (d.low() - 1)) == 0 {
                /* if d is a power of 2 */
                if let Some(rem) = rem {
                    unsafe { rem.as_mut_ptr().write(u256::new(n.low() & (d.low() - 1))) };
                }
                if *d.low() == 1 {
                    set!(res = *n);
                    return;
                }
                sr = d.low().trailing_zeros();
                set!(
                    res = u256::from_words(
                        n.high() >> sr,
                        (n.high() << (n_udword_bits - sr)) | (n.low() >> sr),
                    )
                );
                return;
            }
            // K X
            // ---
            // 0 K
            sr = 1 + n_udword_bits + d.low().leading_zeros() - (n.high()).leading_zeros();
            // 2 <= sr <= n_utword_bits - 1
            // q.all = n.all << (n_utword_bits - sr);
            // r.all = n.all >> sr;
            if sr == n_udword_bits {
                set!(q = u256::from_words(*n.low(), 0));
                set!(r = u256::from_words(0, *n.high()));
            } else if sr < n_udword_bits {
                /* 2 <= sr <= n_udword_bits - 1 */
                set!(q = u256::from_words(n.low() << (n_udword_bits - sr), 0));
                set!(
                    r = u256::from_words(
                        n.high() >> sr,
                        (n.high() << (n_udword_bits - sr)) | (n.low() >> sr),
                    )
                );
            } else {
                /* n_udword_bits + 1 <= sr <= n_utword_bits - 1 */
                set!(
                    q = u256::from_words(
                        (n.high() << (n_utword_bits - sr)) | (n.low() >> (sr - n_udword_bits)),
                        n.low() << (n_utword_bits - sr),
                    )
                );
                set!(r = u256::from_words(0, n.high() >> (sr - n_udword_bits)));
            }
        } else {
            // K X
            // ---
            // K K
            sr = (d.high()).leading_zeros() - (n.high()).leading_zeros();
            // 0 <= sr <= n_udword_bits - 1 or sr large
            if sr > n_udword_bits - 1 {
                if let Some(rem) = rem {
                    unsafe { rem.as_mut_ptr().write(*n) };
                }
                set!(res = u256::ZERO);
                return;
            }
            sr += 1;
            // 1 <= sr <= n_udword_bits
            // q.all = n.all << (n_utword_bits - sr);
            // r.all = n.all >> sr;
            if sr == n_udword_bits {
                set!(q = u256::from_words(*n.low(), 0));
                set!(r = u256::from_words(0, *n.high()));
            } else {
                set!(
                    r = u256::from_words(
                        n.high() >> sr,
                        (n.high() << (n_udword_bits - sr)) | (n.low() >> sr),
                    )
                );
                set!(q = u256::from_words(n.low() << (n_udword_bits - sr), 0));
            }
        }
    }
    // Not a special case
    // q and r are initialized with:
    // q.all = n.all << (n_utword_bits - sr);
    // r.all = n.all >> sr;
    // 1 <= sr <= n_utword_bits - 1
    let mut carry = 0u32;
    let mut q = unsafe { q.assume_init() };
    let mut r = unsafe { r.assume_init() };
    while sr > 0 {
        // r:q = ((r:q)  << 1) | carry
        *r.high_mut() = (r.high() << 1) | (r.low() >> (n_udword_bits - 1));
        *r.low_mut() = (r.low() << 1) | (q.high() >> (n_udword_bits - 1));
        *q.high_mut() = (q.high() << 1) | (q.low() >> (n_udword_bits - 1));
        *q.low_mut() = (q.low() << 1) | (carry as u128);
        // carry = 0;
        // if (r.all >= d.all)
        // {
        //     r.all -= d.all;
        //      carry = 1;
        // }
        let s: u256 = (d - r - 1) >> (n_utword_bits - 1);
        carry = (*s.low() as u32) & 1;
        todo!("r -= d & s");
        sr -= 1;
    }
    q = (q << 1) | u256::from(carry);
    if let Some(rem) = rem {
        unsafe { rem.as_mut_ptr().write(r) };
    }
    set!(res = q);
    return;
}

pub fn div2(r: &mut u256, a: &u256) {
    let (a, b) = (*r, a);
    // SAFETY: `udivmodti4` does not write `MaybeUninit::uninit()` to `res` and
    // `u256` does not implement `Drop`.
    let res = unsafe { &mut *(r as *mut u256).cast() };
    udivmodti4(res, &a, b, None);
}

pub fn div3(r: &mut MaybeUninit<u256>, a: &u256, b: &u256) {
    udivmodti4(r, a, b, None);
}

pub fn rem2(r: &mut u256, a: &u256) {
    let mut res = MaybeUninit::uninit();
    let (a, b) = (*r, a);
    // SAFETY: `udivmodti4` does not write `MaybeUninit::uninit()` to `rem` and
    // `u256` does not implement `Drop`.
    let r = unsafe { &mut *(r as *mut u256).cast() };
    udivmodti4(&mut res, &a, b, Some(r));
}

pub fn rem3(r: &mut MaybeUninit<u256>, a: &u256, b: &u256) {
    let mut res = MaybeUninit::uninit();
    udivmodti4(&mut res, &a, b, Some(r));
}
