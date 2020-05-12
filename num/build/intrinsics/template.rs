/// This module contains the `intrinsics` macro definition used for generating
/// an LLVM IR for `u128` intrinsics which is used as a template `u256`.
///
/// This is done instead of hand-writting LLVM IR to ensure that the attributes
/// on functions and parameters are as accurate as possible for the resulting
/// IR.

macro_rules! intrinsics {
    ($(
        $name:ident = (
            $r:ident,
            $( $p:ident $(: $pt:ty)? ),*
        ) $(: $ret:ty)? => $block:block
    )*) => {$(
        export! {
            concat!("__ethrs_num_", stringify!($name));
            pub extern "C" fn $name(
                $r: &mut u128,
                $( $p: ty!(param: $($pt)?), )*
            ) -> ty!(ret: $($ret)?) {
                $block
            }
        }
    )*};
}

macro_rules! export {
    ($sym:expr; $fn:item) => {
        #[export_name = $sym]
        $fn
    };
}

macro_rules! ty {
    (param:) => {
        &u128
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

include!("../../src/intrinsics/definitions.rs");
