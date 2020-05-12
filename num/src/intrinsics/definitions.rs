// This module contains intrinsic definitions for the `u256` implementation.
//
// Note that this source file is used for both generating a template for the
// LLVM IR based on code generated for `i128` in the build script, as well as
// import the generated assembly in the actual crate.

intrinsics! {
    add = (r, a, b) => { *r = *a + *b }
    add_overflow = (r, a, b): bool => {
        let (c, overflow) = a.overflowing_add(*b);
        *r = c;
        overflow
    }
    add_sat = (r, a, b) => { *r = a.saturating_add(*b) }
    add_assign = (r, a) => { *r += *a }

    sub = (r, a, b) => { *r = *a - *b }
    sub_overflow = (r, a, b): bool => {
        let (c, overflow) = a.overflowing_sub(*b);
        *r = c;
        overflow
    }
    sub_sat = (r, a, b) => { *r = a.saturating_sub(*b) }
    sub_assign = (r, a) => { *r -= *a }
}
