//! Module containing macros for implementing [`std::ops`] traits.

macro_rules! impl_binops {
    ($(
        $op:ident {
            $method:ident,
            $overflow:ident $msg:expr
        }
    )*) => {$(
        impl std::ops::$op<&'_ $crate::u256> for &'_ $crate::u256 {
            type Output = $crate::u256;

            fn $method(self, rhs: &'_ $crate::u256) -> Self::Output {
                __binop!($method, $overflow [ self, rhs ] $msg)
            }
        }

        __impl_ref_binop! {
            $op <&$crate::u256 ; &$crate::u256>::$method {
                <$crate::u256> for &'_ $crate::u256;
                <&'_ $crate::u256> for $crate::u256;
                <$crate::u256> for $crate::u256;
            }
        }

        /*
        __impl_prim_binop! {
            $op <&$crate::u256 ; $crate::u256>::$method {
                i8, i16, i32, i64, i128,
                u8, u16, u32, u64, u128,
            }
        }
        */
    )*};
}

macro_rules! __binop {
    ($wrap:ident, $overflow:ident [ $lhs:expr, $rhs:expr ] $msg:expr) => {{
        let mut result = u256::uninit();
        #[cfg(not(debug_assertions))]
        {
            $crate::intrinsics::$wrap(&mut result, $lhs, $rhs);
        }
        #[cfg(debug_assertions)]
        {
            let overflow = $crate::intrinsics::$overflow(&mut result, $lhs, $rhs);
            if overflow {
                panic!(concat!("attempt to ", $msg));
            }
        }
        result
    }};
}

macro_rules! __impl_ref_binop {
    (
        $op:ident <$ref:ty ; $tr:ty> :: $method:ident {$(
            <$rhs:ty> for $lhs:ty;
        )*}
    ) => {$(
        impl std::ops::$op<$rhs> for $lhs {
            type Output = $crate::u256;

            fn $method(self, rhs: $rhs) -> Self::Output {
                <$ref as std::ops::$op<$tr>>::$method(&self, &rhs)
            }
        }
    )*};
}

/*
macro_rules! __impl_prim_binop {
    (
        $op:ident <$ref:ty ; $tr:ty> :: $method:ident {$(
            <$rhs:ty> for $lhs:ty;
        )*}
    ) => {$(
        impl std::ops::$op<$rhs> for $lhs {
            type Output = $crate::u256;

            fn $method(self, rhs: $rhs) -> Self::Output {
                <$ref as std::ops::$op<$tr>>::$method(&self, &rhs)
            }
        }
    )*};
}
*/

/*
@str.0 = internal constant [33 x i8] c"attempt to multiply with overflow"
@str.1 = internal constant [28 x i8] c"attempt to add with overflow"
@str.2 = internal constant [25 x i8] c"attempt to divide by zero"
@str.3 = internal constant [31 x i8] c"attempt to negate with overflow"
@str.4 = internal constant [57 x i8] c"attempt to calculate the remainder with a divisor of zero"
@str.5 = internal constant [35 x i8] c"attempt to shift left with overflow"
@str.6 = internal constant [36 x i8] c"attempt to shift right with overflow"
@str.7 = internal constant [33 x i8] c"attempt to subtract with overflow"
*/
