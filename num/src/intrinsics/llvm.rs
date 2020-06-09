//! This module contains definitions for LLVM IR generated intrinsics.

// NOTE: LLVM IR generated intrinsics for `udiv i256` and `urem i256` produce an
// error when compiling, so use the native `divmod` implementation even when
// generated intrinsics are enabled.
#[path = "native/divmod.rs"]
mod divmod;

pub use self::divmod::*;
use crate::u256;
use std::mem::MaybeUninit;

macro_rules! def {
    ($(
        $(#[$a:meta])*
        pub fn $name:ident(
            $($p:ident : $t:ty),*
        ) $(-> $ret:ty)?;
    )*) => {$(
        $(#[$a])*
        pub fn $name(
            $($p: $t,)*
        ) $(-> $ret)? {
            #[allow(improper_ctypes)]
            extern "C" {
                link! {
                    concat!("__ethrs_num_", stringify!($name));
                    fn $name(
                        $($p: $t,)*
                    ) $(-> $ret)?;
                }
            }

            unsafe {
                $name($($p),*)
            }
        }
    )*};
}

macro_rules! link {
    ($sym:expr; $fn:item) => {
        #[link_name = $sym]
        $fn
    };
}

def! {
    pub fn add2(r: &mut u256, a: &u256);
    pub fn add3(r: &mut MaybeUninit<u256>, a: &u256, b: &u256);
    pub fn addc(r: &mut MaybeUninit<u256>, a: &u256, b: &u256) -> bool;

    pub fn mul2(r: &mut u256, a: &u256);
    pub fn mul3(r: &mut MaybeUninit<u256>, a: &u256, b: &u256);
    pub fn mulc(r: &mut MaybeUninit<u256>, a: &u256, b: &u256) -> bool;

    pub fn sub2(r: &mut u256, a: &u256);
    pub fn sub3(r: &mut MaybeUninit<u256>, a: &u256, b: &u256);
    pub fn subc(r: &mut MaybeUninit<u256>, a: &u256, b: &u256) -> bool;

    pub fn shl2(r: &mut u256, a: u32);
    pub fn shl3(r: &mut MaybeUninit<u256>, a: &u256, b: u32);

    pub fn shr2(r: &mut u256, a: u32);
    pub fn shr3(r: &mut MaybeUninit<u256>, a: &u256, b: u32);
}
