//! This module contains intrinsics used by the `u256` implementation.
//!
//! Note that this source file is used for both generating a template for the
//! LLVM IR based on code generated for `i128`, as well as import the generated
//! assembly for `i256`s.

#![allow(dead_code)]

macro_rules! intrinsics {
    ($(
        $name:ident = (
            $r:ident,
            $( $p:ident $(: $pt:ty)? ),*
        ) $(: $ret:ty)? => $block:block
    )*) => {
        mod ffi {
            extern "C" {$(
                link! {
                    concat!("__ethrs_num_", stringify!($name));
                    pub(crate) fn $name(
                        $r: &mut $crate::u256,
                        $( $p: ty!(param: $($pt)?), )*
                    ) -> ty!(ret: $($ret)?);
                }
            )*}
        }
        $(
            pub fn $name(
                $r: &mut $crate::u256,
                $( $p: ty!(param: $($pt)?), )*
            ) -> ty!(ret: $($ret)?) {
                unsafe {
                    self::ffi::$name($r, $($p),*)
                }
            }
        )*
    };
}

macro_rules! link {
    ($sym:expr; $fn:item) => {
        #[link_name = $sym]
        $fn
    };
}

macro_rules! ty {
    (param:) => {
        &$crate::u256
    };
    (param: $t:ty) => {
        $t
    };
    (ret:) => {
        ()
    };
    (ret: $t:ty) => {
        $t
    };
}

include!("intrinsics/definitions.rs");

#[cfg(test)]
mod tests {
    use super::*;
    use crate::u256;

    #[test]
    fn unchecked_addition() {
        let mut res = u256::default();
        add(&mut res, &u256([1, 2]), &u256([3, 0]));
        assert_eq!(res, u256([4, 2]),);
    }
}
