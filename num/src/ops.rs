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

            #[inline]
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

            #[inline]
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
    Mul { mul => mul3, mulc; "multiply with overflow" }
    Sub { sub => sub3, subc; "subtract with overflow" }
}

impl std::ops::Div for &'_ u256 {
    type Output = u256;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        if rhs == &u256::ZERO {
            panic!("attempt to divide by zero");
        }

        todo!()
        // let mut result = MaybeUninit::uninit();
        // div3(&mut result, self, rhs);
        // unsafe { result.assume_init() }
    }
}

impl_auto_binop!(Div { div });

impl std::ops::Rem for &'_ u256 {
    type Output = u256;

    #[inline]
    fn rem(self, rhs: Self) -> Self::Output {
        if rhs == &u256::ZERO {
            panic!("attempt to calculate the remainder with a divisor of zero");
        }

        todo!()
        // let mut result = MaybeUninit::uninit();
        // rem3(&mut result, self, rhs);
        // unsafe { result.assume_init() }
    }
}

impl_auto_binop!(Rem { rem });

macro_rules! impl_shifts {
    ($(
        $op:ident {
            $method:ident =>
            $wrap:path; $msg:expr
        }
    )*) => {$(
        impl std::ops::$op<u32> for &'_ u256 {
            type Output = u256;

            #[inline]
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
        #[cfg(debug_assertions)]
        if $rhs > 0xff {
            panic!(concat!("attempt to ", $msg));
        }

        let mut result = MaybeUninit::uninit();
        $wrap(&mut result, $lhs, $rhs);

        unsafe { result.assume_init() }
    }};
}

impl_shifts! {
    Shl { shl => shl3; "shift left with overflow" }
    Shr { shr => shr3; "shift right with overflow" }
}

impl std::ops::Not for u256 {
    type Output = u256;

    #[inline]
    fn not(self) -> Self::Output {
        let u256([a, b]) = self;
        u256([!a, !b])
    }
}

impl std::ops::Not for &'_ u256 {
    type Output = u256;

    #[inline]
    fn not(self) -> Self::Output {
        let u256([a, b]) = self;
        u256([!a, !b])
    }
}

macro_rules! impl_bitwiseops {
    ($(
        $op:ident { $method:ident }
    )*) => {$(
        impl std::ops::$op<&'_ u256> for &'_ u256 {
            type Output = u256;

            #[inline]
            fn $method(self, rhs: &'_ u256) -> Self::Output {
                let u256([a, b]) = self;
                let u256([rhs_a, rhs_b]) = rhs;
                u256([a.$method(rhs_a), b.$method(rhs_b)])
            }
        }

        impl_auto_binop!($op { $method });
    )*};
}

impl_bitwiseops! {
    BitAnd { bitand }
    BitOr { bitor }
    BitXor { bitxor }
}

macro_rules! impl_binops_assign {
    ($(
        $op:ident {
            $method:ident =>
            $wrap:path,
            $binop:tt
        }
    )*) => {$(
        impl std::ops::$op<&'_ u256> for u256 {
            #[inline]
            fn $method(&mut self, rhs: &'_ u256) {
                binop_assign!($wrap, $binop [ self, rhs ])
            }
        }

        impl_auto_binop_assign!($op { $method });
    )*};
}

macro_rules! binop_assign {
    ($wrap:path, $binop:tt [ $lhs:expr, $rhs:expr ]) => {{
        #[cfg(not(debug_assertions))]
        {
            $wrap($lhs, $rhs);
        }
        #[cfg(debug_assertions)]
        {
            *($lhs) = *($lhs) $binop *($rhs);
        }
    }};
}

macro_rules! impl_auto_binop_assign {
    ($op:ident { $method:ident }) => {
        impl_ref_binop_assign! {
            $op <u256 ; &u256>::$method (rhs) {
                <u256> for u256 => { &rhs }
            }
        }

        impl_ref_binop_assign! {
            $op <u256 ; u256>::$method (rhs) { u128 } => { u256::new(rhs) }
        }
    };
}

macro_rules! impl_ref_binop_assign {
    (
        $op:ident <$ref:ty ; $tr:ty> :: $method:ident ($x:ident) {$(
            <$rhs:ty> for u256 => $conv:block
        )*}
    ) => {$(
        impl std::ops::$op<$rhs> for u256 {
            #[inline]
            fn $method(&mut self, $x: $rhs) {
                <$ref as std::ops::$op<$tr>>::$method(self, $conv)
            }
        }
    )*};
    (
        $op:ident <$ref:ty ; $tr:ty> :: $method:ident ($x:ident) {
            $($rhs:ty),* $(,)?
        } => $conv:block
    ) => {$(
        impl_ref_binop_assign! {
            $op <u256 ; $tr>::$method (rhs) {
                <&'_ $rhs> for u256 => { let $x = *rhs; $conv }
                <$rhs> for u256 => { let $x = rhs; $conv }
            }
        }
    )*};
}

impl_binops_assign! {
    AddAssign { add_assign => add2, + }
    DivAssign { div_assign => div2, / }
    MulAssign { mul_assign => mul2, * }
    RemAssign { rem_assign => rem2, % }
    SubAssign { sub_assign => sub2, - }
}

macro_rules! impl_shifts_assign {
    ($(
        $op:ident {
            $method:ident =>
                $wrap:path, $sh:tt
        }
    )*) => {$(
        impl std::ops::$op<u32> for u256 {
            #[inline]
            fn $method(&mut self, rhs: u32) {
                binop_assign!($wrap, $sh [ self, &rhs ])
            }
        }

        impl_ref_binop_assign! {
            $op <u256 ; u32>::$method (rhs) {
                <&'_ u32> for u256 => { *rhs }
            }
        }

        impl_ref_binop_assign! {
            $op <u256 ; u32>::$method (rhs) { u256 } => { *rhs.low() as _ }
        }

        impl_ref_binop_assign! {
            $op <u256 ; u32>::$method (rhs) {
                i8, i16, i32, i64, i128, isize,
                u8, u16, u64, u128, usize,
            } => { rhs as _ }
        }
    )*};
}

impl_shifts_assign! {
    ShlAssign { shl_assign => shl2, << }
    ShrAssign { shr_assign => shr2, >> }
}

macro_rules! impl_bitwiseops_assign {
    ($(
        $op:ident { $method:ident }
    )*) => {$(
        impl std::ops::$op<&'_ u256> for u256 {
            #[inline]
            fn $method(&mut self, rhs: &'_ u256) {
                let u256([a, b]) = self;
                let u256([rhs_a, rhs_b]) = rhs;
                a.$method(rhs_a);
                b.$method(rhs_b);
            }
        }

        impl_auto_binop_assign!($op { $method });
    )*};
}

impl_bitwiseops_assign! {
    BitAndAssign { bitand_assign }
    BitOrAssign { bitor_assign }
    BitXorAssign { bitxor_assign }
}

#[cfg(test)]
mod tests {
    use crate::u256;
    use std::ops::*;

    #[test]
    fn trait_implementations() {
        trait Implements {}
        impl Implements for u256 {}
        impl Implements for &'_ u256 {}

        fn assert_ops<T>()
        where
            for<'a> T: Implements
                + Add<&'a u128>
                + Add<&'a u256>
                + Add<u128>
                + Add<u256>
                + AddAssign<&'a u128>
                + AddAssign<&'a u256>
                + AddAssign<u128>
                + AddAssign<u256>
                + BitAnd<&'a u128>
                + BitAnd<&'a u256>
                + BitAnd<u128>
                + BitAnd<u256>
                + BitAndAssign<&'a u128>
                + BitAndAssign<&'a u256>
                + BitAndAssign<u128>
                + BitAndAssign<u256>
                + BitOr<&'a u128>
                + BitOr<&'a u256>
                + BitOr<u128>
                + BitOr<u256>
                + BitOrAssign<&'a u128>
                + BitOrAssign<&'a u256>
                + BitOrAssign<u128>
                + BitOrAssign<u256>
                + BitXor<&'a u128>
                + BitXor<&'a u256>
                + BitXor<u128>
                + BitXor<u256>
                + BitXorAssign<&'a u128>
                + BitXorAssign<&'a u256>
                + BitXorAssign<u128>
                + BitXorAssign<u256>
                + Div<&'a u128>
                + Div<&'a u256>
                + Div<u128>
                + Div<u256>
                + DivAssign<&'a u128>
                + DivAssign<&'a u256>
                + DivAssign<u128>
                + DivAssign<u256>
                + Mul<&'a u128>
                + Mul<&'a u256>
                + Mul<u128>
                + Mul<u256>
                + MulAssign<&'a u128>
                + MulAssign<&'a u256>
                + MulAssign<u128>
                + MulAssign<u256>
                + Not
                + Rem<&'a u128>
                + Rem<&'a u256>
                + Rem<u128>
                + Rem<u256>
                + RemAssign<&'a u128>
                + RemAssign<&'a u256>
                + RemAssign<u128>
                + RemAssign<u256>
                + Shl<&'a i128>
                + Shl<&'a i16>
                + Shl<&'a i32>
                + Shl<&'a i64>
                + Shl<&'a i8>
                + Shl<&'a isize>
                + Shl<&'a u128>
                + Shl<&'a u16>
                + Shl<&'a u256>
                + Shl<&'a u32>
                + Shl<&'a u64>
                + Shl<&'a u8>
                + Shl<&'a usize>
                + Shl<i128>
                + Shl<i16>
                + Shl<i32>
                + Shl<i64>
                + Shl<i8>
                + Shl<isize>
                + Shl<u128>
                + Shl<u16>
                + Shl<u256>
                + Shl<u32>
                + Shl<u64>
                + Shl<u8>
                + Shl<usize>
                + ShlAssign<&'a i128>
                + ShlAssign<&'a i16>
                + ShlAssign<&'a i32>
                + ShlAssign<&'a i64>
                + ShlAssign<&'a i8>
                + ShlAssign<&'a isize>
                + ShlAssign<&'a u128>
                + ShlAssign<&'a u16>
                + ShlAssign<&'a u256>
                + ShlAssign<&'a u32>
                + ShlAssign<&'a u64>
                + ShlAssign<&'a u8>
                + ShlAssign<&'a usize>
                + ShlAssign<i128>
                + ShlAssign<i16>
                + ShlAssign<i32>
                + ShlAssign<i64>
                + ShlAssign<i8>
                + ShlAssign<isize>
                + ShlAssign<u128>
                + ShlAssign<u16>
                + ShlAssign<u256>
                + ShlAssign<u32>
                + ShlAssign<u64>
                + ShlAssign<u8>
                + ShlAssign<usize>
                + Shr<&'a i128>
                + Shr<&'a i16>
                + Shr<&'a i32>
                + Shr<&'a i64>
                + Shr<&'a i8>
                + Shr<&'a isize>
                + Shr<&'a u128>
                + Shr<&'a u16>
                + Shr<&'a u256>
                + Shr<&'a u32>
                + Shr<&'a u64>
                + Shr<&'a u8>
                + Shr<&'a usize>
                + Shr<i128>
                + Shr<i16>
                + Shr<i32>
                + Shr<i64>
                + Shr<i8>
                + Shr<isize>
                + Shr<u128>
                + Shr<u16>
                + Shr<u256>
                + Shr<u32>
                + Shr<u64>
                + Shr<u8>
                + Shr<usize>
                + ShrAssign<&'a i128>
                + ShrAssign<&'a i16>
                + ShrAssign<&'a i32>
                + ShrAssign<&'a i64>
                + ShrAssign<&'a i8>
                + ShrAssign<&'a isize>
                + ShrAssign<&'a u128>
                + ShrAssign<&'a u16>
                + ShrAssign<&'a u256>
                + ShrAssign<&'a u32>
                + ShrAssign<&'a u64>
                + ShrAssign<&'a u8>
                + ShrAssign<&'a usize>
                + ShrAssign<i128>
                + ShrAssign<i16>
                + ShrAssign<i32>
                + ShrAssign<i64>
                + ShrAssign<i8>
                + ShrAssign<isize>
                + ShrAssign<u128>
                + ShrAssign<u16>
                + ShrAssign<u256>
                + ShrAssign<u32>
                + ShrAssign<u64>
                + ShrAssign<u8>
                + ShrAssign<usize>
                + Sub<&'a u128>
                + Sub<&'a u256>
                + Sub<u128>
                + Sub<u256>
                + SubAssign<&'a u128>
                + SubAssign<&'a u256>
                + SubAssign<u128>
                + SubAssign<u256>,
            for<'a> &'a T: Implements
                + Add<&'a u128>
                + Add<&'a u256>
                + Add<u128>
                + Add<u256>
                + BitAnd<&'a u128>
                + BitAnd<&'a u256>
                + BitAnd<u128>
                + BitAnd<u256>
                + BitOr<&'a u128>
                + BitOr<&'a u256>
                + BitOr<u128>
                + BitOr<u256>
                + BitXor<&'a u128>
                + BitXor<&'a u256>
                + BitXor<u128>
                + BitXor<u256>
                + Div<&'a u128>
                + Div<&'a u256>
                + Div<u128>
                + Div<u256>
                + Mul<&'a u128>
                + Mul<&'a u256>
                + Mul<u128>
                + Mul<u256>
                + Not
                + Rem<&'a u128>
                + Rem<&'a u256>
                + Rem<u128>
                + Rem<u256>
                + Shl<&'a i128>
                + Shl<&'a i16>
                + Shl<&'a i32>
                + Shl<&'a i64>
                + Shl<&'a i8>
                + Shl<&'a isize>
                + Shl<&'a u128>
                + Shl<&'a u16>
                + Shl<&'a u256>
                + Shl<&'a u32>
                + Shl<&'a u64>
                + Shl<&'a u8>
                + Shl<&'a usize>
                + Shl<i128>
                + Shl<i16>
                + Shl<i32>
                + Shl<i64>
                + Shl<i8>
                + Shl<isize>
                + Shl<u128>
                + Shl<u16>
                + Shl<u256>
                + Shl<u32>
                + Shl<u64>
                + Shl<u8>
                + Shl<usize>
                + Shr<&'a i128>
                + Shr<&'a i16>
                + Shr<&'a i32>
                + Shr<&'a i64>
                + Shr<&'a i8>
                + Shr<&'a isize>
                + Shr<&'a u128>
                + Shr<&'a u16>
                + Shr<&'a u256>
                + Shr<&'a u32>
                + Shr<&'a u64>
                + Shr<&'a u8>
                + Shr<&'a usize>
                + Shr<i128>
                + Shr<i16>
                + Shr<i32>
                + Shr<i64>
                + Shr<i8>
                + Shr<isize>
                + Shr<u128>
                + Shr<u16>
                + Shr<u256>
                + Shr<u32>
                + Shr<u64>
                + Shr<u8>
                + Shr<usize>
                + Sub<&'a u128>
                + Sub<&'a u256>
                + Sub<u128>
                + Sub<u256>,
        {
        }

        assert_ops::<u256>();
    }
}
