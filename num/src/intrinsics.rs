//! This module contains intrinsics used by the `u256` implementation.

mod mul;
pub mod native;
mod udivmod;

pub use self::mul::{mulc as multi3c, multi3};
#[cfg(not(linker_plugin_lto))]
pub use self::native::*;
pub use self::udivmod::udivmodti4;
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
    pub fn mul2(r: &mut u256, a: &u256);
    pub fn mul3(r: &mut MaybeUninit<u256>, a: &u256, b: &u256);
    pub fn mulc(r: &mut MaybeUninit<u256>, a: &u256, b: &u256) -> bool;
}

#[cfg(linker_plugin_lto)]
def! {
    pub fn add2(r: &mut u256, a: &u256);
    pub fn add3(r: &mut MaybeUninit<u256>, a: &u256, b: &u256);
    pub fn addc(r: &mut MaybeUninit<u256>, a: &u256, b: &u256) -> bool;

    pub fn sub2(r: &mut u256, a: &u256);
    pub fn sub3(r: &mut MaybeUninit<u256>, a: &u256, b: &u256);
    pub fn subc(r: &mut MaybeUninit<u256>, a: &u256, b: &u256) -> bool;

    pub fn shl2(r: &mut u256, a: u32);
    pub fn shl3(r: &mut MaybeUninit<u256>, a: &u256, b: u32);

    pub fn shr2(r: &mut u256, a: u32);
    pub fn shr3(r: &mut MaybeUninit<u256>, a: &u256, b: u32);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::u256;

    #[test]
    fn unchecked_addition() {
        let mut res = MaybeUninit::uninit();
        add3(&mut res, &u256([1, 2]), &u256([3, 0]));
        assert_eq!(unsafe { res.assume_init() }, u256([4, 2]));
    }
}
