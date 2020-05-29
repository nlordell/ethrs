//! Module containing macros for implementing [`std::ops`] traits.

use crate::intrinsics::*;
use crate::u256;
use std::mem::MaybeUninit;

macro_rules! impl_binops {
    ($(
        $op:ident {
            $method:ident =>
            $wrap:path,
            $overflow:path; $msg:expr
        }
    )*) => {$(
        impl std::ops::$op<&'_ u256> for &'_ u256 {
            type Output = u256;

            #[inline(always)]
            fn $method(self, rhs: &'_ u256) -> Self::Output {
                binop!($wrap, $overflow [ self, rhs ] $msg)
            }
        }

        impl_auto_binop!($op { $method });
    )*};
}

macro_rules! binop {
    ($wrap:path, $overflow:path [ $lhs:expr, $rhs:expr ] $msg:expr) => {{
        let mut result = MaybeUninit::uninit();
        #[cfg(not(debug_assertions))]
        {
            $wrap(&mut result, $lhs, $rhs);
        }
        #[cfg(debug_assertions)]
        {
            let overflow = $overflow(&mut result, $lhs, $rhs);
            if overflow {
                panic!(concat!("attempt to ", $msg));
            }
        }
        unsafe { result.assume_init() }
    }};
}

macro_rules! impl_auto_binop {
    ($op:ident { $method:ident }) => {
        impl_ref_binop! {
            $op <&u256 ; &u256>::$method (rhs) {
                <u256> for &'_ u256 => { &rhs }
                <&'_ u256> for u256 => { rhs }
                <u256> for u256 => { &rhs }
            }
        }

        impl_ref_binop! {
            $op <&u256 ; u256>::$method (rhs) { u128 } => { u256::new(rhs) }
        }
    };
}

macro_rules! impl_ref_binop {
    (
        $op:ident <$ref:ty ; $tr:ty> :: $method:ident ($x:ident) {$(
            <$rhs:ty> for $lhs:ty => $conv:block
        )*}
    ) => {$(
        impl std::ops::$op<$rhs> for $lhs {
            type Output = u256;

            #[inline(always)]
            fn $method(self, $x: $rhs) -> Self::Output {
                <$ref as std::ops::$op<$tr>>::$method(&self, $conv)
            }
        }
    )*};
    (
        $op:ident <$ref:ty ; $tr:ty> :: $method:ident ($x:ident) {
            $($rhs:ty),* $(,)?
        } => $conv:block
    ) => {$(
        impl_ref_binop! {
            $op <&u256 ; $tr>::$method (rhs) {
                <&'_ $rhs> for &'_ u256 => { let $x = *rhs; $conv }
                <&'_ $rhs> for u256 => { let $x = *rhs; $conv }
                <$rhs> for &'_ u256 => { let $x = rhs; $conv }
                <$rhs> for u256 => { let $x = rhs; $conv }
            }
        }
    )*};
}

impl_binops! {
    Add { add => add3, addc; "add with overflow" }
    Sub { sub => sub3, subc; "subtract with overflow" }
    Mul { mul => mul3, mulc; "multiply with overflow" }
}

macro_rules! impl_shifts {
    ($(
        $op:ident {
            $method:ident =>
            $wrap:path; $msg:expr
        }
    )*) => {$(
        impl std::ops::$op<u32> for &'_ u256 {
            type Output = u256;

            #[inline(always)]
            fn $method(self, rhs: u32) -> Self::Output {
                shift!($wrap [ self, rhs ] $msg)
            }
        }

        impl_ref_binop! {
            $op <&u256 ; u32>::$method (rhs) {
                <&'_ u32> for &'_ u256 => { *rhs }
                <&'_ u32> for u256 => { *rhs }
                <u32> for u256 => { rhs }
            }
        }

        impl_ref_binop! {
            $op <&u256 ; u32>::$method (rhs) { u256 } => { *rhs.low() as _ }
        }

        impl_ref_binop! {
            $op <&u256 ; u32>::$method (rhs) {
                i8, i16, i32, i64, i128, isize,
                u8, u16, u64, u128, usize,
            } => { rhs as _ }
        }
    )*};
}

macro_rules! shift {
    ($wrap:path [ $lhs:expr, $rhs:expr ] $msg:expr) => {{
        let mut result = MaybeUninit::uninit();
        #[cfg(not(debug_assertions))]
        {
            $wrap(&mut result, $lhs, $rhs & 0xff);
        }
        #[cfg(debug_assertions)]
        {
            if $rhs > 0xff {
                panic!(concat!("attempt to ", $msg));
            }
            $wrap(&mut result, $lhs, $rhs);
        }
        unsafe { result.assume_init() }
    }};
}

impl_shifts! {
    Shl { shl => shl3; "shift left with overflow" }
    Shr { shr => shr3; "shift right with overflow" }
}

/*
overflowing_div
overflowing_div_euclid
overflowing_neg
overflowing_pow
overflowing_rem
overflowing_rem_euclid

@str.2 = internal constant [25 x i8] c"attempt to divide by zero"
@str.3 = internal constant [31 x i8] c"attempt to negate with overflow"
@str.4 = internal constant [57 x i8] c"attempt to calculate the remainder with a divisor of zero"
*/
