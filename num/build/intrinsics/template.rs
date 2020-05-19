//! This module contains the `intrinsics` macro definition used for generating
//! an LLVM IR for `u128` intrinsics which is used as a template `u256`.
//!
//! This is done instead of hand-writting LLVM IR to ensure that the attributes
//! on functions and parameters are as accurate as possible for the resulting
//! IR.

macro_rules! def {
    ($(
        pub fn $name:ident($r:ident, $($p:ident $(: $t:ty)?),*) $(-> $ret:ty)? $impl:block
    )*) => {$(
        export! {
            name = concat!("__ethrs_num_", stringify!($name));
            pub extern "C" fn $name(
                $r: &mut u128,
                $($p: ty!($($t)? ,|| &u128),)*
            ) $(-> $ret)? {
                $impl
            }
        }
    )*};
}

macro_rules! export {
    (name = $sym:expr; $fn:item) => {
        #[export_name = $sym]
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
    pub fn add2(r, a) {
        *r += *a;
    }
    pub fn add3(r, a, b) {
        *r = *a + *b;
    }
    pub fn addc(r, a, b) -> bool {
        let (sum, carry) = a.overflowing_add(*b);
        *r = sum;
        carry
    }

    pub fn sub2(r, a) {
        *r -= *a;
    }
    pub fn sub3(r, a, b) {
        *r = *a - *b;
    }
    pub fn subc(r, a, b) -> bool {
        let (sum, carry) = a.overflowing_sub(*b);
        *r = sum;
        carry
    }

    pub fn mul2(r, a) {
        *r *= *a;
    }
    pub fn mul3(r, a, b) {
        *r = *a * *b;
    }
    pub fn mulc(r, a, b) -> bool {
        let (sum, carry) = a.overflowing_mul(*b);
        *r = sum;
        carry
    }

    pub fn shl2(r, a: u32) {
        *r <<= a;
    }
    pub fn shl3(r, a, b: u32) {
        *r = *a << b;
    }

    pub fn shr2(r, a: u32) {
        *r >>= a;
    }
    pub fn shr3(r, a, b: u32) {
        *r = *a >> b;
    }
}
