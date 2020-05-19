//! This module contains intrinsics used by the `u256` implementation.

#![allow(dead_code)]

macro_rules! def {
    ($(
        pub fn $name:ident($r:ident, $($p:ident $(: $t:ty)?),*) $(-> $ret:ty)?;
    )*) => {
        mod ffi {
            #![allow(improper_ctypes)]
            extern "C" {$(
                link! {
                    concat!("__ethrs_num_", stringify!($name));
                    pub(crate) fn $name(
                        $r: &mut $crate::u256,
                        $( $p: ty!($($t)? ,|| &crate::u256), )*
                    ) $(-> $ret)?;
                }
            )*}
        }
        $(
            pub fn $name(
                $r: &mut $crate::u256,
                $( $p: ty!($($t)? ,|| &crate::u256), )*
            ) $(-> $ret)? {
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
    (,|| $t:ty) => {
        $t
    };
    ($t:ty ,|| $d:ty) => {
        $t
    };
}

def! {
    pub fn add2(r, a);
    pub fn add3(r, a, b);
    pub fn addc(r, a, b) -> bool;

    pub fn sub2(r, a);
    pub fn sub3(r, a, b);
    pub fn subc(r, a, b) -> bool;

    pub fn mul2(r, a);
    pub fn mul3(r, a, b);
    pub fn mulc(r, a, b) -> bool;

    pub fn shl2(r, a: u32);
    pub fn shl3(r, a, b: u32);

    pub fn shr2(r, a: u32);
    pub fn shr3(r, a, b: u32);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::u256;

    #[test]
    fn unchecked_addition() {
        let mut res = u256::default();
        add3(&mut res, &u256([1, 2]), &u256([3, 0]));
        assert_eq!(res, u256([4, 2]),);
    }
}
